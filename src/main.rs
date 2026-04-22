// OAP/src/main.rs
// Core Entry Point - Orbital Analysis Pro (V1.0 - Sovereign Edition)

use std::panic;
use std::time::{Duration, Instant};

use oap::collision::coordination::{negotiate_evasion, OapCryptoCore, TargetIdentity};
use oap::collision::evasive_action::compute_evasion_maneuver;
use oap::collision::radar_scanner::{ObjectClass, RadarFilter, SpaceObject};
use oap::collision::threat_eval::{assess_threat, ThreatLevel};
use oap::engine::gravity_model::calculate_j2_perturbation;
use oap::engine::orbital_mechanics::{OrbitalState, Vector3, EARTH_RADIUS};
use oap::telemetry::public_boardcast::CcsdsTelemetry;

// Thermal Control & Tick Rate: OAP runs at 60 Calculations per second.
const TICK_RATE_HZ: u64 = 60;
const TICK_INTERVAL: Duration = Duration::from_millis(1000 / TICK_RATE_HZ);

fn main() {
    // 1. AEROSPACE-GRADE PANIC HANDLER (Anti-Crash System)
    panic::set_hook(Box::new(|info| {
        eprintln!("\n[CRITICAL FAIL] OAP System Anomaly Detected!");
        eprintln!("Detail Error: {:?}", info);
        eprintln!("[AUTO-RECOVERY] Mengaktifkan Mode Darurat & Transmisi SOS ke Stasiun Bumi...");
        oap::telemetry::transmitter::send_sos();
    }));

    println!("==================================================");
    println!(" OAP (Orbital Analysis Pro) - KERNEL INITIALIZED");
    println!(" VERSION: 1.0 (SOVEREIGN COMMAND EDITION)");
    println!("==================================================");

    // 2. BOOTING SEQUENCE & STATE INITIALIZATION
    // Positioning the satellite in Low Earth Orbit (LEO) at an altitude of 500 km
    let mut sat_state = OrbitalState {
        position: Vector3 {
            x: EARTH_RADIUS + 500_000.0,
            y: 0.0,
            z: 0.0,
        },
        velocity: Vector3 {
            x: 0.0,
            y: 7600.0,
            z: 0.0,
        },
    };
    let my_fuel_percent = 85.0;

    let radar = RadarFilter::new(50000.0);
    let crypto_core = OapCryptoCore::new(0x123456789ABCDEF0);

    // Simulation of object database in space (In the real world this data is supplied by sensors)
    let mut space_catalog = vec![SpaceObject {
        id: 99,
        position: Vector3 {
            x: sat_state.position.x + 800.0,
            y: 1500.0,
            z: 0.0,
        },
        velocity: Vector3 {
            x: 0.0,
            y: -7000.0,
            z: 0.0,
        },
        object_class: ObjectClass::Debris,
    }];

    let mut mission_clock = 0u64;
    println!("[SYS] Boot Sequence Complete. Entering Main Orbital Loop.\n");

    // 3. MAIN FLIGHT LOOP
    loop {
        let start_time = Instant::now();
        let current_timestamp = 1680000000 + (mission_clock / TICK_RATE_HZ);

        // --- FASE 1: ORBITAL PROPAGATION & J2 PERTURBATION ---
        // Moving according to the real laws of physics, the earth is oval.
        let delta_t = 1.0 / TICK_RATE_HZ as f64;
        let j2_accel = calculate_j2_perturbation(sat_state.position);
        sat_state.velocity.x += j2_accel.x * delta_t;
        sat_state.velocity.y += j2_accel.y * delta_t;
        sat_state.velocity.z += j2_accel.z * delta_t;
        sat_state.propagate(delta_t);

        space_catalog[0].position.y += space_catalog[0].velocity.y * delta_t;

        // --- FASE 2: RADAR SCANNING ---
        let threats = radar.get_proximate_threats(&sat_state.position, &space_catalog);

        // --- FASE 3: THREAT EVALUATION & AI SWARM COORDINATION ---
        for threat in threats {
            let assessment = assess_threat(&sat_state.position, &sat_state.velocity, &threat);

            if assessment.level == ThreatLevel::Critical {
                println!(
                    "\n[!] CRITICAL PROXIMITY ALERT! Collision in: {:.2}s",
                    assessment.tca_seconds
                );

                let simulated_ping = 0;
                let identity = crypto_core.verify_ally(simulated_ping, current_timestamp);

                match identity {
                    TargetIdentity::VerifiedAlly => {
                        let ally_fuel = 10.0;
                        if negotiate_evasion(my_fuel_percent, ally_fuel) {
                            let maneuver =
                                compute_evasion_maneuver(&sat_state.position, &sat_state.velocity);
                            println!(
                                " >> [SWARM] Give in to friends. Ignites maneuvering rockets: {:?}",
                                maneuver.delta_v
                            );
                        } else {
                            println!(" >> [SWARM] Friends who will maneuver. We are silent.");
                        }
                    }
                    TargetIdentity::StandardInternational => {
                        let maneuver =
                            compute_evasion_maneuver(&sat_state.position, &sat_state.velocity);
                        println!(
                            " >> [FOREIGN] Foreign / Spy Satellites. Evasive Mode: {:?}",
                            maneuver.delta_v
                        );
                    }
                    TargetIdentity::UnknownDebris => {
                        let maneuver =
                            compute_evasion_maneuver(&sat_state.position, &sat_state.velocity);
                        println!(
                            " >> [DEBRIS] SPACE DUST! Delta-V Execution Normal: {:?}",
                            maneuver.delta_v
                        );

                        space_catalog[0].position.y = 9999999.0;
                    }
                }
            }
        }

        // --- FASE 4: TELEMETRY STREAMING ---
        if mission_clock % TICK_RATE_HZ == 0 {
            let seconds_online = mission_clock / TICK_RATE_HZ;
            println!(
                "[T-PLUS: {:02}s] NAV-SYS: Pos(X:{:.1}, Y:{:.1}) | J2: COMPENSATED | LINK: SECURE",
                seconds_online, sat_state.position.x, sat_state.position.y
            );
            let my_sat_id = "ID-OAP-001";
            let ccsds_packet =
                CcsdsTelemetry::new(my_sat_id, &sat_state.position, &sat_state.velocity);
            let broadcast_string = ccsds_packet.encode_to_string();

            println!("[PUBLIC TX] Memancarkan data CCSDS: {}", broadcast_string);
        }

        mission_clock += 1;

        // --- FASE 5: THERMAL CONTROL (Frame Limiter) ---
        // Prevents the Satellite HP/CPU from overheating by putting the processor to sleep for a fraction of a millisecond.
        let elapsed = start_time.elapsed();
        if elapsed < TICK_INTERVAL {
            std::thread::sleep(TICK_INTERVAL - elapsed);
        } else {
            eprintln!("[WARNING] Frame Drop! CPU Overload: {:?}", elapsed);
        }
    }
}
