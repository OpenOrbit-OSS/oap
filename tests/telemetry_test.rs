// OAP/tests/telemetry_test.rs
// Ensures communication robustness and memory safety against corrupted data.

use oap::telemetry::receiver::parse_incoming_data;

#[test]
fn test_valid_packet_parsing() {
    // Construct a dummy valid packet (minimum 32 bytes as defined in receiver.rs)
    let valid_raw_data = vec![0xFF; 32];

    let result = parse_incoming_data(&valid_raw_data);

    assert!(
        result.is_some(),
        "System rejected a perfectly valid telemetry packet."
    );

    let packet = result.unwrap();
    assert!(packet.system_health_ok, "Health status parsed incorrectly.");
}

#[test]
fn test_corrupted_packet_rejection() {
    // Construct an incomplete packet (less than 32 bytes)
    let corrupted_raw_data = vec![0xFF, 0xAA, 0x01, 0x02];

    let result = parse_incoming_data(&corrupted_raw_data);

    // System MUST return None and not panic/crash
    assert!(
        result.is_none(),
        "CRITICAL: System attempted to parse corrupted data, risking memory overflow."
    );
}
