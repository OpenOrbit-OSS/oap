// OAP/src/engine/gravity_model.rs
// Implementation of Earth's non-spherical gravity perturbations (J2 Model).

use super::orbital_mechanics::{Vector3, EARTH_GRAVITY_MU, EARTH_RADIUS};

/// J2 Perturbation coefficient for Earth's oblateness.
const J2_COEFFICIENT: f64 = 1.08262668e-3;

/// Calculates the perturbing acceleration due to Earth's J2 effect.
/// This increases accuracy for Low Earth Orbit (LEO) satellites.
pub fn calculate_j2_perturbation(pos: Vector3) -> Vector3 {
    let r_sq = pos.x.powi(2) + pos.y.powi(2) + pos.z.powi(2);
    let r = r_sq.sqrt();

    // Pre-calculate common factors for efficiency
    let z_sq_ratio = (pos.z.powi(2)) / r_sq;
    let factor = (1.5 * J2_COEFFICIENT * EARTH_GRAVITY_MU * EARTH_RADIUS.powi(2)) / r.powi(5);

    Vector3 {
        x: factor * pos.x * (5.0 * z_sq_ratio - 1.0),
        y: factor * pos.y * (5.0 * z_sq_ratio - 1.0),
        z: factor * pos.z * (5.0 * z_sq_ratio - 3.0),
    }
}
