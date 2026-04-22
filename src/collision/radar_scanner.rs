// OAP/src/collision/radar_scanner.rs
// Handles spatial filtering of space objects based on radar range.

use crate::engine::orbital_mechanics::Vector3;

#[derive(Clone, PartialEq, Debug)]
pub enum ObjectClass {
    Debris,
    Satellite,
    Asteroid,
}

#[derive(Clone, Debug)]
pub struct SpaceObject {
    pub id: u32,
    pub position: Vector3,
    pub velocity: Vector3,
    pub object_class: ObjectClass,
}

pub struct RadarFilter {
    pub scan_radius: f64,
}

impl RadarFilter {
    pub fn new(radius: f64) -> Self {
        RadarFilter {
            scan_radius: radius,
        }
    }

    /// Evaluates the catalog and returns only objects within the scan radius.
    pub fn get_proximate_threats(
        &self,
        sat_pos: &Vector3,
        catalog: &[SpaceObject],
    ) -> Vec<SpaceObject> {
        let mut threats = Vec::new();

        for obj in catalog {
            // Calculate true 3D distance using the Pythagorean theorem
            let dx = sat_pos.x - obj.position.x;
            let dy = sat_pos.y - obj.position.y;
            let dz = sat_pos.z - obj.position.z;

            let distance = (dx * dx + dy * dy + dz * dz).sqrt();

            // If the object is inside our radar bubble, add it to threats!
            if distance <= self.scan_radius {
                threats.push(obj.clone());
            }
        }

        threats
    }
}
