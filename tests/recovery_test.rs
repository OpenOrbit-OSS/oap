// tests/recovery_test.rs
// Integration tests for the Adaptive Orbit Recovery (AOR) module.
// Ensures the PD controller calculates accurate and fuel-efficient maneuvers.

use oap::collision::recovery::{compute_recovery_maneuver, RecoveryConfig};
use oap::engine::orbital_mechanics::Vector3;

#[test]
fn test_zero_drift_zero_thrust() {
    // SCENARIO 1: The satellite is exactly on its reference orbit.
    // EXPECTATION: The system should be smart enough to apply ZERO thrust (save fuel).

    let ideal_pos = Vector3 {
        x: 6871000.0,
        y: 0.0,
        z: 0.0,
    };
    let ideal_vel = Vector3 {
        x: 0.0,
        y: 7600.0,
        z: 0.0,
    };
    let config = RecoveryConfig::default();

    let maneuver =
        compute_recovery_maneuver(&ideal_pos, &ideal_vel, &ideal_pos, &ideal_vel, &config);

    let magnitude =
        (maneuver.delta_v.x.powi(2) + maneuver.delta_v.y.powi(2) + maneuver.delta_v.z.powi(2))
            .sqrt();

    // Assert that thrust is practically zero
    assert!(
        magnitude < 0.0001,
        "CRITICAL FAILURE: Thrusters fired when satellite is already on target. Wasting fuel!"
    );
}

#[test]
fn test_directional_correction() {
    // SCENARIO 2: The satellite drifted 1000 meters too far on the X-axis.
    // EXPECTATION: The thrust vector should pull it back (negative X direction).

    let current_pos = Vector3 {
        x: 6872000.0,
        y: 0.0,
        z: 0.0,
    }; // 1000m off-target
    let reference_pos = Vector3 {
        x: 6871000.0,
        y: 0.0,
        z: 0.0,
    };
    let current_vel = Vector3 {
        x: 0.0,
        y: 7600.0,
        z: 0.0,
    };
    let config = RecoveryConfig::default();

    let maneuver = compute_recovery_maneuver(
        &current_pos,
        &current_vel,
        &reference_pos,
        &current_vel,
        &config,
    );

    assert!(
        maneuver.delta_v.x < 0.0,
        "CRITICAL FAILURE: Satellite pushed in the wrong direction!"
    );
}

#[test]
fn test_fuel_clamping_safety() {
    // SCENARIO 3: The satellite is drastically pushed away (e.g., severe impact or anomaly).
    // EXPECTATION: The algorithm must NOT request a massive thrust that depletes all fuel at once. It must clamp to `max_delta_v`.

    let current_pos = Vector3 {
        x: 9999999.0,
        y: 9999999.0,
        z: 0.0,
    }; // Drastically lost
    let reference_pos = Vector3 {
        x: 6871000.0,
        y: 0.0,
        z: 0.0,
    };
    let current_vel = Vector3 {
        x: 0.0,
        y: 7600.0,
        z: 0.0,
    };
    let config = RecoveryConfig::default();

    let maneuver = compute_recovery_maneuver(
        &current_pos,
        &current_vel,
        &reference_pos,
        &current_vel,
        &config,
    );

    let magnitude =
        (maneuver.delta_v.x.powi(2) + maneuver.delta_v.y.powi(2) + maneuver.delta_v.z.powi(2))
            .sqrt();

    // Allow a tiny floating-point margin (0.0001) for safety
    assert!(
        magnitude <= config.max_delta_v + 0.0001,
        "CRITICAL FAILURE: Fuel clamp bypassed! Thruster command exceeded max safe limits (Requested: {:.2}, Max: {:.2})", 
        magnitude, config.max_delta_v
    );
}
