// OAP/src/telemetry/receiver.rs
// Safely parses incoming data packets from ground stations or satellite sensors.

use crate::engine::orbital_mechanics::Vector3;

#[derive(Debug)]
pub struct TelemetryPacket {
    pub timestamp: u64,
    pub current_position: Vector3,
    pub current_velocity: Vector3,
    pub system_health_ok: bool,
}

/// Safely attempts to parse raw byte data into a structured packet.
/// Uses Option to prevent system crashes if corrupted data is received.
pub fn parse_incoming_data(raw_data: &[u8]) -> Option<TelemetryPacket> {
    // Aerospace safety check: Verify minimum packet size to prevent buffer overflows
    if raw_data.len() < 32 {
        return None; // Data is corrupted or incomplete, reject silently.
    }

    // Simulated parsing logic (In production, this extracts bits based on ICD standards)
    // We assume the data is valid for this simulation.
    let parsed_packet = TelemetryPacket {
        timestamp: 1672531200, // Example epoch time
        current_position: Vector3 {
            x: 7000.0,
            y: 0.0,
            z: 0.0,
        },
        current_velocity: Vector3 {
            x: 0.0,
            y: 7.5,
            z: 0.0,
        },
        system_health_ok: true,
    };

    Some(parsed_packet)
}

/// Fetches the latest data from the hardware buffer.
pub fn poll_hardware_buffer() -> Vec<u8> {
    // Simulating a stream of bytes received from the satellite's antenna
    vec![0xFF, 0xAA, 0x01, 0x02, 0x03, 0x00] // Dummy byte stream
}
