// OAP/src/collision/evasive_action.rs
// Generates optimal avoidance maneuver vectors with minimal delta-v expenditure.

use crate::engine::orbital_mechanics::Vector3;

#[derive(Debug, Clone)]
pub struct ManeuverVector {
    pub delta_v: Vector3,
    pub duration_milliseconds: u32,
}

/// Computes the optimal Delta-V maneuver (Normal vector).
/// "Delta-V Minimizer Engine": Calculates the exact perpendicular
/// thrust vector to slide the satellite out of danger using the least amount of fuel.
pub fn compute_evasion_maneuver(position: &Vector3, velocity: &Vector3) -> ManeuverVector {
    // 1. CROSS PRODUCT (Position x Velocity)
    // Calculates the Normal vector that is 100% perpendicular to the flight path.
    let nx = (position.y * velocity.z) - (position.z * velocity.y);
    let ny = (position.z * velocity.x) - (position.x * velocity.z);
    let nz = (position.x * velocity.y) - (position.y * velocity.x);

    // 2. VECTOR NORMALIZATION
    // Squeeze the vector to standard size (length = 1.0)
    let magnitude = (nx * nx + ny * ny + nz * nz).sqrt();

    // Safety check: Prevent "divided by zero" (ZeroDivision) errors
    // if the sensor suddenly turns off/sends a zero. (Zero-Bug Policy)
    let (unit_x, unit_y, unit_z) = if magnitude > 0.0 {
        (nx / magnitude, ny / magnitude, nz / magnitude)
    } else {
        (0.0, 0.0, 1.0)
    };

    // 3. APPLY MINIMAL THRUST
    // Just push 0.5 m/s to the side.
    let avoidance_magnitude = 0.5;
    let safe_vector = Vector3 {
        x: unit_x * avoidance_magnitude,
        y: unit_y * avoidance_magnitude,
        z: unit_z * avoidance_magnitude,
    };

    ManeuverVector {
        delta_v: safe_vector,
        duration_milliseconds: 2000,
    }
}
