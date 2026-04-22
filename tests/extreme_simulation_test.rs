// OAP/tests/extreme_simulation_test.rs
// Simulates an extreme orbital scenario: J2 perturbations + High-speed debris field.

use oap::collision::radar_scanner::{ObjectClass, SpaceObject};
use oap::collision::threat_eval::{assess_threat, ThreatLevel};
use oap::engine::gravity_model::calculate_j2_perturbation;
use oap::engine::orbital_mechanics::{OrbitalState, Vector3, EARTH_RADIUS};

#[test]
fn test_extreme_orbital_survival() {
    // 1. INITIAL CONDITION
    let initial_altitude = EARTH_RADIUS + 500_000.0;
    let mut sat_state = OrbitalState {
        position: Vector3 {
            x: initial_altitude,
            y: 0.0,
            z: 0.0,
        },
        velocity: Vector3 {
            x: 0.0,
            y: 7600.0,
            z: 0.0,
        },
    };

    // 2. TIME SIMULATION (Fast forward 1 hour)
    let delta_t = 1.0;
    for _ in 0..3600 {
        let j2_accel = calculate_j2_perturbation(sat_state.position);

        sat_state.velocity.x += j2_accel.x * delta_t;
        sat_state.velocity.y += j2_accel.y * delta_t;
        sat_state.velocity.z += j2_accel.z * delta_t;

        sat_state.propagate(delta_t);
    }

    let final_altitude = sat_state.get_altitude();
    assert!(
        final_altitude > 400_000.0,
        "FATAL: Satellite decayed into atmosphere!"
    );

    // 3. DISASTER SCENARIO (MATHEMATICAL IMPROVEMENT)
    // We place the trash at a relative position (500m, 500m, 500m) from the satellite.
    let debris = SpaceObject {
        id: 666,
        position: Vector3 {
            x: sat_state.position.x + 500.0,
            y: sat_state.position.y + 500.0,
            z: sat_state.position.z + 500.0,
        },
        velocity: Vector3 {
            // Satellite speed plus impact speed
            x: sat_state.velocity.x - 500.0,
            y: sat_state.velocity.y - 500.0,
            z: sat_state.velocity.z - 500.0,
        },
        object_class: ObjectClass::Debris,
    };

    // 4. OAP SYSTEM ANALYZES THREATS
    let assessment = assess_threat(&sat_state.position, &sat_state.velocity, &debris);

    assert_eq!(
        assessment.level,
        ThreatLevel::Critical,
        "OAP FAILED to detect high-speed collision!"
    );
    assert!(
        assessment.tca_seconds < 5.0,
        "OAP is too slow, calculation gives wrong impact time!"
    );
}
