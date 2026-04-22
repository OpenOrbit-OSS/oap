// OAP/tests/coordination_test.rs
// Validates the Sovereign Swarm Protocol against spoofing and tests negotiation logic.

use oap::collision::coordination::{negotiate_evasion, OapCryptoCore, TargetIdentity};

#[test]
fn test_secret_handshake_ally_verified() {
    let crypto = OapCryptoCore::new(0x123456789ABCDEF0);
    let current_time = 1680000000;

    let valid_ping = crypto.generate_secret_ping(current_time);

    let identity = crypto.verify_ally(valid_ping, current_time);
    assert_eq!(
        identity,
        TargetIdentity::VerifiedAlly,
        "FATAL: OAP failed to recognize friend's satellite (False Negative)!"
    );
}

#[test]
fn test_secret_handshake_foreign_spoofing() {
    let crypto = OapCryptoCore::new(0x123456789ABCDEF0);
    let current_time = 1680000000;

    let fake_ping_from_hacker = 0x9999999999999999;

    let identity = crypto.verify_ally(fake_ping_from_hacker, current_time);

    // OAP should intelligently reject this fake encryption and label it as Foreign Satellite
    assert_eq!(
        identity,
        TargetIdentity::StandardInternational,
        "CRITICAL: OAP encryption successfully cracked by enemy (False Positive)!"
    );
}

#[test]
fn test_secret_handshake_debris_detection() {
    let crypto = OapCryptoCore::new(0x123456789ABCDEF0);
    let current_time = 1680000000;

    // Inanimate objects (Garbage) do not emit any radio signals (0)
    let dead_ping = 0;

    let identity = crypto.verify_ally(dead_ping, current_time);
    assert_eq!(
        identity,
        TargetIdentity::UnknownDebris,
        "FATAL: OAP thought space junk was a satellite!"
    );
}

#[test]
fn test_swarm_negotiation_logic() {
    let my_fuel = 85.0;
    let ally_fuel = 10.0;

    // OAP must decide who to sacrifice for maneuvering
    let should_i_move = negotiate_evasion(my_fuel, ally_fuel);

    assert!(
        should_i_move,
        "LOGIC BUG: OAP instead tells the dying satellite to maneuver!"
    );

    // Reverse scenario test (We are the ones dying)
    let should_i_move_if_low = negotiate_evasion(5.0, 90.0);
    assert!(
        !should_i_move_if_low,
        "LOGIC BUG: OAP tells us to maneuver when we're running out of gas!"
    );
}
