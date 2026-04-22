// OAP/src/collision/threat_eval.rs
// Evaluates collision probability using Time of Closest Approach (TCA).

use crate::collision::radar_scanner::SpaceObject;
use crate::engine::orbital_mechanics::Vector3;

#[derive(Debug, PartialEq)]
pub enum ThreatLevel {
    Clear,
    Monitor,
    Critical,
}

pub struct ThreatAssessment {
    pub level: ThreatLevel,
    pub tca_seconds: f64,
    pub miss_distance_meters: f64,
}

// Calculates the closest approach using relative vectors.
// Extremely fast deterministic logic suitable for real-time execution.
pub fn assess_threat(
    sat_pos: &Vector3,
    sat_vel: &Vector3,
    threat: &SpaceObject,
) -> ThreatAssessment {
    let rel_pos = Vector3 {
        x: threat.position.x - sat_pos.x,
        y: threat.position.y - sat_pos.y,
        z: threat.position.z - sat_pos.z,
    };

    let rel_vel = Vector3 {
        x: threat.velocity.x - sat_vel.x,
        y: threat.velocity.y - sat_vel.y,
        z: threat.velocity.z - sat_vel.z,
    };

    let rel_vel_sq = (rel_vel.x * rel_vel.x) + (rel_vel.y * rel_vel.y) + (rel_vel.z * rel_vel.z);

    // If relative velocity is near zero, objects are static relative to each other
    if rel_vel_sq < 1e-6 {
        let dist =
            ((rel_pos.x * rel_pos.x) + (rel_pos.y * rel_pos.y) + (rel_pos.z * rel_pos.z)).sqrt();
        return ThreatAssessment {
            level: if dist < 1000.0 {
                ThreatLevel::Critical
            } else {
                ThreatLevel::Clear
            },
            tca_seconds: 0.0,
            miss_distance_meters: dist,
        };
    }

    // TCA Formula: -(r . v) / |v|^2
    let dot_product = (rel_pos.x * rel_vel.x) + (rel_pos.y * rel_vel.y) + (rel_pos.z * rel_vel.z);
    let tca = -dot_product / rel_vel_sq;

    // If TCA is negative, the object has already passed us
    if tca < 0.0 {
        return ThreatAssessment {
            level: ThreatLevel::Clear,
            tca_seconds: tca,
            miss_distance_meters: f64::MAX,
        };
    }

    // Calculate predicted minimum distance at the exact time of approach
    let min_x = rel_pos.x + (rel_vel.x * tca);
    let min_y = rel_pos.y + (rel_vel.y * tca);
    let min_z = rel_pos.z + (rel_vel.z * tca);
    let miss_distance = ((min_x * min_x) + (min_y * min_y) + (min_z * min_z)).sqrt();

    // Categorize the threat strictly based on distance and timeframe
    let level = if tca < 3600.0 && miss_distance < 1000.0 {
        ThreatLevel::Critical // Less than 1 hour away and closer than 1km
    } else if tca < 86400.0 && miss_distance < 5000.0 {
        ThreatLevel::Monitor // Within 24 hours and 5km
    } else {
        ThreatLevel::Clear
    };

    ThreatAssessment {
        level,
        tca_seconds: tca,
        miss_distance_meters: miss_distance,
    }
}
