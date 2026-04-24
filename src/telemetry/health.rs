// OAP/src/telemetry/health.rs
// Fault Detection, Isolation, and Recovery (FDIR) Module

use crate::engine::orbital_mechanics::{OrbitalState, Vector3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SubsystemState {
    Nominal,        //  Walk perfectly
    Warning,        // There is a slight anomaly, it needs to be
    Critical,       // Severe damage
    Isolated,       // Forced shutdown to prevent infection in the system
}

#[derive(Debug)]
pub struct FdirController {
    pub nav_system: SubsystemState,
    pub radar_system: SubsystemState,
    pub telemetry_system: SubsystemState,
    pub is_safe_mode_active: bool,
}

impl FdirController {
    /// Initialize the FDIR system when the satellite first turns on
    pub fn new() -> Self {
        Self {
            nav_system: SubsystemState::Nominal,
            radar_system: SubsystemState::Nominal,
            telemetry_system: SubsystemState::Nominal,
            is_safe_mode_active: false,
        }
    }

    // The main function is to check the satellite's "Vital Signs".
    pub fn run_diagnostics(&mut self, pos_x: f64, pos_y: f64, pos_z: f64, fuel_level: f64) {
        // 1. MEMORY INTEGRITY CHECK
        // Checks whether there are "NaN" (Not a Number) or infinite numbers in memory.
        if pos_x.is_nan() || pos_y.is_nan() || pos_z.is_nan() || pos_x.is_infinite() {
            self.nav_system = SubsystemState::Critical;
        }

        // 2. LEO HARD-DECK CHECK
        // Using the distance formula from the center point of the earth (Geocenter)
        let altitude_squared = (pos_x * pos_x) + (pos_y * pos_y) + (pos_z * pos_z);
        let earth_radius_sq = 6371000.0 * 6371000.0;

        // If the satellite altitude is lower than the earth's radius + 150km (thick atmosphere)
        if altitude_squared < (earth_radius_sq + (150_000.0 * 150_000.0)) {
            self.nav_system = SubsystemState::Critical;
        }

        // 3. FUEL ANOMALY CHECK
        if fuel_level < 5.0 && fuel_level >= 0.0 {
            self.nav_system = SubsystemState::Warning;
        } else if fuel_level < 0.0 {
            self.nav_system = SubsystemState::Critical;
        }

        self.evaluate_system_health();
    }

    /// Damage Evaluation and Isolation
    fn evaluate_system_health(&mut self) {
        if self.nav_system == SubsystemState::Critical {
            self.engage_safe_mode();
        }

        // If the radar is acting up, isolate the radar so the battery doesn't run out.
        if self.radar_system == SubsystemState::Critical {
            self.radar_system = SubsystemState::Isolated;
            // TODO: Activate backup device if available
        }
    }

    /// Shut down all non-essential systems
    fn engage_safe_mode(&mut self) {
        if !self.is_safe_mode_active {
            self.is_safe_mode_active = true;
            self.radar_system = SubsystemState::Isolated; // Matikan radar untuk hemat daya
            
            self.broadcast_sos();
        }
    }

    /// Internal function to trigger transmission hardware
    fn broadcast_sos(&self) {
        let sos_packet: [u8; 5] = [0xFF, 0x53, 0x4F, 0x53, 0xAA];
        println!("[CRITICAL] SAFE MODE ENGAGED. Broadcasting SOS: {:X?}", sos_packet);
    }
}
