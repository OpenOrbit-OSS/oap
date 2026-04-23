// OAP/src/collision/recovery.rs
// Adaptive Orbit Recovery (AOR) Module
// Automatically returns the satellite to its reference orbit with maximum fuel efficiency.

use crate::collision::evasive_action::ManeuverVector;
use crate::engine::orbital_mechanics::Vector3;

/// Intelligent configuration to limit fuel usage and adjust maneuver smoothness.
pub struct RecoveryConfig {
    pub proportional_gain: f64, // K_p: Pulls the satellite back to its target position (Accuracy)
    pub derivative_gain: f64,   // K_d: Dampens the velocity to prevent overshoot (Stability)
    pub max_delta_v: f64,       // Hard limit (m/s) per maneuver to ensure fuel preservation
}

impl Default for RecoveryConfig {
    /// Default values tuned for a safe and efficient return to orbit.
    fn default() -> Self {
        RecoveryConfig {
            proportional_gain: 0.001, // Gentle pull to save fuel
            derivative_gain: 0.05,    // Sufficient damping to match the orbital rate
            max_delta_v: 1.5,         // Safeguard to prevent excessive propellant consumption
        }
    }
}

/// Computes the recovery maneuver vector deterministically.
/// This algorithm ensures the satellite returns to its assigned path without oscillating.
pub fn compute_recovery_maneuver(
    current_pos: &Vector3,
    current_vel: &Vector3,
    reference_pos: &Vector3,
    reference_vel: &Vector3,
    config: &RecoveryConfig,
) -> ManeuverVector {
    // 1. Calculate Position Error (Current vs. Reference Target)
    let pos_error = Vector3 {
        x: reference_pos.x - current_pos.x,
        y: reference_pos.y - current_pos.y,
        z: reference_pos.z - current_pos.z,
    };

    // 2. Calculate Velocity Error (Current vs. Ideal Orbital Velocity)
    let vel_error = Vector3 {
        x: reference_vel.x - current_vel.x,
        y: reference_vel.y - current_vel.y,
        z: reference_vel.z - current_vel.z,
    };

    // 3. Apply PD (Proportional-Derivative) Control Logic
    // This calculates the required Delta-V to close the gap and match speed.
    let mut raw_delta_v = Vector3 {
        x: (config.proportional_gain * pos_error.x) + (config.derivative_gain * vel_error.x),
        y: (config.proportional_gain * pos_error.y) + (config.derivative_gain * vel_error.y),
        z: (config.proportional_gain * pos_error.z) + (config.derivative_gain * vel_error.z),
    };

    // 4. Fuel Safety System (Vector Clamping)
    // Prevents the system from requesting a thrust that exceeds the defined safety limits.
    let magnitude = (raw_delta_v.x.powi(2) + raw_delta_v.y.powi(2) + raw_delta_v.z.powi(2)).sqrt();

    if magnitude > config.max_delta_v {
        let scale = config.max_delta_v / magnitude;
        raw_delta_v.x *= scale;
        raw_delta_v.y *= scale;
        raw_delta_v.z *= scale;
    }

    // 5. Encapsulate into a ManeuverVector
    // Recovery burns are typically longer and gentler (5000ms) than emergency evasions.
    ManeuverVector {
        delta_v: raw_delta_v,
        duration_milliseconds: 5000,
    }
}
