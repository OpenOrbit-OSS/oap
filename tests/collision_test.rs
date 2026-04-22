// OAP/tests/collision_test.rs
// Validates radar scanning, threat evaluation, and evasion logic.

use oap::collision::evasive_action::compute_evasion_maneuver;
use oap::collision::radar_scanner::{ObjectClass, RadarFilter, SpaceObject};
use oap::collision::threat_eval::{assess_threat, ThreatLevel};
use oap::engine::orbital_mechanics::Vector3;

#[test]
fn test_radar_threat_filtering() {
    let filter = RadarFilter::new(50.0); // 50 km scan radius
    let sat_pos = Vector3 {
        x: 7000.0,
        y: 0.0,
        z: 0.0,
    };

    let catalog = vec![
        SpaceObject {
            id: 1,
            position: Vector3 {
                x: 7010.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            object_class: ObjectClass::Debris,
        }, // 10km away
        SpaceObject {
            id: 2,
            position: Vector3 {
                x: 8000.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            object_class: ObjectClass::Debris,
        }, // 1000km away
    ];

    let threats = filter.get_proximate_threats(&sat_pos, &catalog);

    assert_eq!(
        threats.len(),
        1,
        "Radar failed to filter out distant objects."
    );
    assert_eq!(threats[0].id, 1, "Radar identified the wrong object.");
}

#[test]
fn test_head_on_collision_detection() {
    let sat_pos = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let sat_vel = Vector3 {
        x: 7000.0,
        y: 0.0,
        z: 0.0,
    };

    // Debris moving directly opposite to the satellite
    let threat = SpaceObject {
        id: 99,
        position: Vector3 {
            x: 14000.0,
            y: 0.0,
            z: 0.0,
        },
        velocity: Vector3 {
            x: -7000.0,
            y: 0.0,
            z: 0.0,
        },
        object_class: ObjectClass::Debris,
    };

    let assessment = assess_threat(&sat_pos, &sat_vel, &threat);

    // Relative speed is 14000 m/s, distance is 14000m. TCA should be exactly 1.0 second.
    assert_eq!(
        assessment.level,
        ThreatLevel::Critical,
        "Failed to flag critical head-on collision!"
    );
    assert!(
        (assessment.tca_seconds - 1.0).abs() < 1e-5,
        "TCA calculation is mathematically incorrect."
    );
}

#[test]
fn test_evasion_maneuver_generation() {
    // WE ADD THE SATELLITE POSITION (For example, at the equatorial orbital altitude)
    let current_pos = Vector3 {
        x: 7000000.0,
        y: 0.0,
        z: 0.0,
    };
    let current_vel = Vector3 {
        x: 0.0,
        y: 7500.0,
        z: 0.0,
    };

    // Call the function with 2 new arguments: position & velocity
    let maneuver = compute_evasion_maneuver(&current_pos, &current_vel);

    assert!(
        maneuver.delta_v.z > 0.0,
        "Evasion vector must utilize the Z-axis (normal/anti-normal) for efficiency."
    );
    assert_eq!(
        maneuver.duration_milliseconds, 2000,
        "Burn duration exceeds hardware safety limits."
    );
}
