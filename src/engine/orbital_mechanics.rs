// OAP/src/engine/orbital_mechanics.rs
// Core orbital physics and state propagation logic.

/// Physical constants for Earth-centric orbits.
pub const EARTH_GRAVITY_MU: f64 = 3.986004418e14; // Standard gravitational parameter (m^3/s^2)
pub const EARTH_RADIUS: f64 = 6371000.0; // Mean radius of Earth in meters

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct OrbitalState {
    pub position: Vector3,
    pub velocity: Vector3,
}

impl OrbitalState {
    /// Calculate current altitude from Earth's surface.
    pub fn get_altitude(&self) -> f64 {
        let distance_from_center =
            (self.position.x.powi(2) + self.position.y.powi(2) + self.position.z.powi(2)).sqrt();
        distance_from_center - EARTH_RADIUS
    }

    /// Update state using a simple Euler integration (Efficient for high-frequency ticks).
    /// Delta_t is the time step in seconds (e.g., 1/60 for 60Hz).
    pub fn propagate(&mut self, delta_t: f64) {
        let r_mag =
            (self.position.x.powi(2) + self.position.y.powi(2) + self.position.z.powi(2)).sqrt();

        // Calculate acceleration due to gravity: a = -mu * r / r^3
        let accel_factor = -EARTH_GRAVITY_MU / r_mag.powi(3);

        let accel = Vector3 {
            x: accel_factor * self.position.x,
            y: accel_factor * self.position.y,
            z: accel_factor * self.position.z,
        };

        // Update velocity: v = v + a * dt
        self.velocity.x += accel.x * delta_t;
        self.velocity.y += accel.y * delta_t;
        self.velocity.z += accel.z * delta_t;

        // Update position: p = p + v * dt
        self.position.x += self.velocity.x * delta_t;
        self.position.y += self.velocity.y * delta_t;
        self.position.z += self.velocity.z * delta_t;
    }
}
