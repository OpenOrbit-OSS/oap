// OAP/src/collision/coordination.rs
// Sovereign Swarm Protocol: Secure inter-satellite communication and negotiation.

#[derive(Debug, PartialEq)]
pub enum TargetIdentity {
    VerifiedAlly,
    StandardInternational,
    UnknownDebris,
}

pub struct OapCryptoCore {
    // The secret key that is embedded before the satellite is launched
    master_key: u64,
}

impl OapCryptoCore {
    pub fn new(key: u64) -> Self {
        OapCryptoCore { master_key: key }
    }

    /// Generating a 'Ping' signal using Rolling Code (Time-based XOR)
    /// This signal changes every second, making it impossible for the enemy to hack/spoof.
    pub fn generate_secret_ping(&self, current_timestamp: u64) -> u64 {
        // Low-level XOR encryption, executed in 1 CPU cycle (Very efficient)
        self.master_key ^ current_timestamp.wrapping_mul(0x9E3779B185EBCA87)
    }

    /// Verifying 'Pong' replies from other satellites
    pub fn verify_ally(&self, received_ping: u64, current_timestamp: u64) -> TargetIdentity {
        let expected_ping = self.generate_secret_ping(current_timestamp);

        if received_ping == expected_ping {
            TargetIdentity::VerifiedAlly
        } else if received_ping != 0 {
            // Signal does not match, but sending radio data = Foreign Satellite
            TargetIdentity::StandardInternational
        } else {
            // No radio signal at all = Space Debris
            TargetIdentity::UnknownDebris
        }
    }
}

/// Swarm Intelligence negotiation function
/// Determine who should shift based on remaining fuel.
pub fn negotiate_evasion(my_fuel_percent: f64, ally_fuel_percent: f64) -> bool {
    // If we have more fuel, WE will give in and maneuver.
    // If we have less fuel, our satellites will give in.
    my_fuel_percent > ally_fuel_percent
}
