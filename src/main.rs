// OAP/src/main.rs
// Core Entry Point - Orbital Analysis Pro (V1.2.0 - Sovereign Edition with FDIR)

use std::panic;
use std::time::{Duration, Instant};

use oap::collision::coordination::{negotiate_evasion, OapCryptoCore, TargetIdentity};
use oap::collision::evasive_action::compute_evasion_maneuver;
use oap::collision::radar_scanner::{ObjectClass, RadarFilter, SpaceObject};
use oap::collision::recovery::{compute_recovery_maneuver, RecoveryConfig};
use oap::collision::threat_eval::{assess_threat, ThreatLevel};
use oap::engine::gravity_model::calculate_j2_perturbation;
use oap::engine::orbital_mechanics::{OrbitalState, Vector3, EARTH_RADIUS};
use oap::telemetry::public_boardcast::CcsdsTelemetry;
use oap::telemetry::health::FdirController; // <-- FDIR IMPORTED

// Thermal Control & Tick Rate: OAP runs at 60 Calculations per second.
const TICK_RATE_HZ: u64 = 60;
const TICK_INTERVAL: Duration = Duration::from_millis(1000 / TICK_RATE_HZ);

fn main() {
    // 1. AEROSPACE-GRADE PANIC HANDLER (Anti-Crash System)
    panic::set_hook(Box::new(|info| {
        eprintln!("\n[CRITICAL FAIL] OAP System Anomaly Detected!");
        eprintln!("Detail Error: {:?}", info);
        eprintln!("[AUTO-RECOVERY] Activating Emergency Mode & Transmitting SOS to Ground Station...");
        oap::telemetry::transmitter::send_sos();
    }));

    println!("==================================================");
    println!(" OAP (Orbital Analysis Pro) - KERNEL INITIALIZED");
    println!(" VERSION: 1.2.0 (SOVEREIGN COMMAND - FDIR INTEGRATED)");
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

    let reference_state = sat_state;
    let mut needs_recovery = false;
    let recovery_config = RecoveryConfig::default();

    // Changed to mutable so fuel can be consumed during maneuvers
    let mut my_fuel_percent = 85.0; 

    let radar = RadarFilter::new(50000.0);
    let crypto_core = OapCryptoCore::new(0x123456789ABCDEF0);
    
    // Initializing the FDIR Controller (Satellite Doctor)
    let mut fdir = FdirController::new();

    // Simulation of object database in space
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

        // --- PHASE 1: ORBITAL PROPAGATION & J2 PERTURBATION ---
        // Moving according to the real laws of physics, compensating for Earth's oblateness.
        let delta_t = 1.0 / TICK_RATE_HZ as f64;
        let j2_accel = calculate_j2_perturbation(sat_state.position);
        sat_state.velocity.x += j2_accel.x * delta_t;
        sat_state.velocity.y += j2_accel.y * delta_t;
        sat_state.velocity.z += j2_accel.z * delta_t;
        sat_state.propagate(delta_t);

        space_catalog[0].position.y += space_catalog[0].velocity.y * delta_t;

        // ====================================================================
        // --- PHASE 1.5: BRUTAL SIMULATION EVENT (T+4 Seconds) ---
        // Simulating an extreme fuel leak caused by a micrometeoroid impact
        // ====================================================================
        if mission_clock == (TICK_RATE_HZ * 4) {
            println!("\n[!] WARNING: SENSOR DETECTS MICROMETEOROID IMPACT!");
            println!("[!] WARNING: FUEL TANK PRESSURE DROPPING RAPIDLY!");
            my_fuel_percent = -5.0; // Triggers critical error in FDIR
        }

        // --- PHASE 1.6: SYSTEM DIAGNOSTICS (FDIR) ---
        // The doctor checks vital signs every frame (60 times per second)
        fdir.run_diagnostics(sat_state.position.x, sat_state.position.y, sat_state.position.z, my_fuel_percent);

        // If the satellite enters Safe Mode, disable radar and maneuvers
        if fdir.is_safe_mode_active {
            if mission_clock.is_multiple_of(TICK_RATE_HZ) {
                println!(" >> [FDIR-LOCK] SAFE MODE ACTIVE! Thrusters offline, Radar isolated. Awaiting ground control...");
            }
            // Skipping Phase 2 and 3 to prevent explosion or battery depletion
        } else {
            // --- PHASE 2: RADAR SCANNING (Nominal Mode) ---
            let threats = radar.get_proximate_threats(&sat_state.position, &space_catalog);

            // --- PHASE 3: THREAT EVALUATION, EVASION & RECOVERY ---
            let mut current_threat_level = ThreatLevel::Clear;

            for threat in &threats {
                let assessment = assess_threat(&sat_state.position, &sat_state.velocity, threat);

                if assessment.level == ThreatLevel::Critical {
                    current_threat_level = ThreatLevel::Critical;

                    println!(
                        "\n[!] CRITICAL PROXIMITY ALERT! Collision in: {:.2}s",
                        assessment.tca_seconds
                    );

                    let simulated_ping = 0;
                    let identity = crypto_core.verify_ally(simulated_ping, current_timestamp);

                    match identity {
                        TargetIdentity::VerifiedAlly => {
                            if negotiate_evasion(my_fuel_percent, 10.0) {
                                let maneuver = compute_evasion_maneuver(&sat_state.position, &sat_state.velocity);
                                println!(" >> [SWARM] Giving in to a friend. Avoiding: {:?}", maneuver.delta_v);
                                
                                sat_state.velocity.x += maneuver.delta_v.x;
                                sat_state.velocity.y += maneuver.delta_v.y;
                                sat_state.velocity.z += maneuver.delta_v.z;
                                my_fuel_percent -= 2.5; // Consuming fuel during maneuver
                                needs_recovery = true;
                            }
                        }
                        TargetIdentity::StandardInternational | TargetIdentity::UnknownDebris => {
                            let maneuver = compute_evasion_maneuver(&sat_state.position, &sat_state.velocity);
                            println!(" >> [EVASION] Foreign Object/Debris! Delta-V Execution: {:?}", maneuver.delta_v);

                            sat_state.velocity.x += maneuver.delta_v.x;
                            sat_state.velocity.y += maneuver.delta_v.y;
                            sat_state.velocity.z += maneuver.delta_v.z;
                            my_fuel_percent -= 2.5; // Consuming fuel
                            needs_recovery = true;

                            if identity == TargetIdentity::UnknownDebris {
                                space_catalog[0].position.y = 9999999.0;
                            }
                        }
                    }
                }
            }

            if current_threat_level != ThreatLevel::Critical && needs_recovery {
                let recovery_burn = compute_recovery_maneuver(
                    &sat_state.position, &sat_state.velocity,
                    &reference_state.position, // Return to reference target
                    &reference_state.velocity,
                    &recovery_config,
                );

                sat_state.velocity.x += recovery_burn.delta_v.x;
                sat_state.velocity.y += recovery_burn.delta_v.y;
                sat_state.velocity.z += recovery_burn.delta_v.z;
                my_fuel_percent -= 0.1; // Micro-burn fuel consumption for AOR

                let burn_magnitude = (recovery_burn.delta_v.x.powi(2) + recovery_burn.delta_v.y.powi(2) + recovery_burn.delta_v.z.powi(2)).sqrt();
                if burn_magnitude < 0.001 {
                    println!(" >> [NAV-SYS] Orbit Recovery Complete. Return to main task..");
                    needs_recovery = false;
                } else {
                    println!(" >> [AOR] Thrust back to original orbit... Thrust: {:.5} m/s", burn_magnitude);
                }
            }
        }

        // --- PHASE 4: TELEMETRY STREAMING ---
        if mission_clock.is_multiple_of(TICK_RATE_HZ) {
            let seconds_online = mission_clock / TICK_RATE_HZ;
            let fdir_status = if fdir.is_safe_mode_active { "SAFE-MODE" } else { "NOMINAL" };

            println!(
                "[T-PLUS: {:02}s] NAV: Pos(X:{:.1}) | SYS: {} | FUEL: {:.1}% | SECURE",
                seconds_online, sat_state.position.x, fdir_status, my_fuel_percent
            );
            
            let my_sat_id = "ID-OAP-001";
            let ccsds_packet = CcsdsTelemetry::new(my_sat_id, &sat_state.position, &sat_state.velocity);
            println!("[PUBLIC TX] Broadcasting CCSDS data: {}", ccsds_packet.encode_to_string());
        }

        mission_clock += 1;

        // --- PHASE 5: THERMAL CONTROL (Frame Limiter) ---
        // Prevents the Satellite HP/CPU from overheating by putting the processor to sleep for a fraction of a millisecond.
        let elapsed = start_time.elapsed();
        if elapsed < TICK_INTERVAL {
            std::thread::sleep(TICK_INTERVAL - elapsed);
        } else {
            eprintln!("[WARNING] Frame Drop! CPU Overload: {:?}", elapsed);
        }
    }
}
