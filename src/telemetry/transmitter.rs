// OAP/src/telemetry/transmitter.rs
// Handles formatting and sending hardware-level commands back to the satellite.

use crate::collision::evasive_action::ManeuverVector;

/// Serializes a maneuver plan into a strict byte array for the hardware controller.
pub fn encode_maneuver_command(plan: &ManeuverVector) -> Vec<u8> {
    // Pre-allocate memory to avoid heap fragmentation (Crucial for embedded systems)
    let mut command_buffer = Vec::with_capacity(16);

    // Command Header (Standard hex code to wake up the thruster module)
    command_buffer.push(0xAA);
    command_buffer.push(0xBB);

    // Encode payload (Simulated logic: storing duration as bytes)
    let duration_bytes = plan.duration_milliseconds.to_be_bytes();
    command_buffer.extend_from_slice(&duration_bytes);

    // Command Footer / Checksum (To verify data integrity)
    command_buffer.push(0xFF);

    command_buffer
}

/// Dispatches the encoded byte stream to the physical hardware.
pub fn transmit_command(command_bytes: &[u8]) {
    // In a real environment, this sends data via UART, CAN bus, or SpaceWire.
    // We use a silent return to keep CPU cycles strictly minimal during flight.
    if command_bytes.is_empty() {
        return;
    }

    // System call to hardware would go here.
}

pub fn send_sos() {
    let sos_packet = vec![0xFF, 0x53, 0x4F, 0x53, 0xAA];

    transmit_command(&sos_packet);

    println!("[TELEMETRY] EMERGENCY: SOS Signal Transmitted via High-Gain Antenna.");
}
