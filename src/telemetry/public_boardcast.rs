// OAP/src/telemetry/public_broadcast.rs
// International CCSDS Telemetry Formatter for Public Broadcast

use crate::engine::orbital_mechanics::Vector3;

/// International standard data representation for public broadcast.
/// Use reference (&) to zero additional memory allocation (Zero-Copy).
pub struct CcsdsTelemetry<'a> {
    pub satellite_id: &'a str,
    pub position: &'a Vector3,
    pub velocity: &'a Vector3,
}

impl<'a> CcsdsTelemetry<'a> {
    pub fn new(id: &'a str, pos: &'a Vector3, vel: &'a Vector3) -> Self {
        Self {
            satellite_id: id,
            position: pos,
            velocity: vel,
        }
    }

    /// Generates CCSDS format string (Decimal accuracy is limited for bandwidth efficiency)
    /// Standard Format: CCSDS|ID|Px,Py,Pz|Vx,Vy,Vz
    pub fn encode_to_string(&self) -> String {
        format!(
            "CCSDS|{}|{:.1},{:.1},{:.1}|{:.2},{:.2},{:.2}",
            self.satellite_id,
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z
        )
    }

    /// Convert directly to transmit-ready byte sequence (u8) for hardware antennas
    pub fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_string().into_bytes()
    }
}
