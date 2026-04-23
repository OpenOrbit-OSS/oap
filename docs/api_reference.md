# OAP System API Reference

This document defines the internal logic interfaces and external execution endpoints for the Orbital Analysis Pro (OAP) modules.

## 1. Engine Module (`src/engine/`)

### `OrbitalState::propagate(&mut self, delta_t: f64)`
Propagates the satellite state vector forward in time using standard gravitational parameters.
* **Input:** `delta_t` (Time step delta in seconds).
* **Mutation:** Updates the internal XYZ position and velocity vectors.

### `calculate_j2_perturbation(pos: Vector3) -> Vector3`
Calculates the perturbing acceleration vector due to Earth's oblateness (J2 effect).
* **Input:** `pos` (Current 3D position vector).
* **Return:** A `Vector3` representing the acceleration modification required to correct the orbital trajectory.

## 2. Collision Module (`src/collision/`)

### `assess_threat(sat_pos: &Vector3, sat_vel: &Vector3, threat: &SpaceObject) -> ThreatAssessment`
Evaluates the mathematical probability of a conjunction event using relative spatial metrics.
* **Input:** Current satellite vectors and the tracked foreign object structure.
* **Return:** A `ThreatAssessment` struct containing `level` (Clear, Monitor, Critical), `tca_seconds`, and `miss_distance_meters`.

### `compute_evasion_maneuver(position: &Vector3, velocity: &Vector3) -> ManeuverVector`
Generates an optimal, fuel-efficient Delta-V burn vector utilizing cross-product normal formulation.
* **Input:** Current satellite position and velocity vectors.
* **Return:** `ManeuverVector` dictating the directional thrust magnitude and burn duration limits.

### `OapCryptoCore::verify_ally(&self, received_ping: u64, current_timestamp: u64) -> TargetIdentity`
Validates incoming radio transmissions against the internal Sovereign Swarm rolling code.
* **Input:** `received_ping` (Intercepted 64-bit integer), `current_timestamp` (Epoch time).
* **Return:** `TargetIdentity` enumeration (`VerifiedAlly`, `StandardInternational`, `UnknownDebris`).

### `compute_recovery_maneuver(current_pos: &Vector3, current_vel: &Vector3, reference_pos: &Vector3, reference_vel: &Vector3, config: &RecoveryConfig) -> ManeuverVector`
Calculates the required Delta-V to close the spatial and velocity gap between the satellite and its reference orbit.
* **Input:** Current state vectors, reference target vectors, and tuning parameters (`RecoveryConfig`).
* **Logic:** Utilizes PD (Proportional-Derivative) control logic to ensure zero-overshoot soft landings back to the orbital path.
* **Return:** `ManeuverVector` capped by the `max_delta_v` threshold to ensure fuel preservation.

## 3. Telemetry Module (`src/telemetry/`)

### `parse_incoming_data(raw_data: &[u8]) -> Option<TelemetryPacket>`
Safely decodes binary streams from ground stations.
* **Safety:** Returns `None` silently if the packet length is invalid, preventing buffer overflow vulnerabilities.

### `send_sos()`
Constructs and dispatches a high-priority hexadecimal distress signal (`0xFF, 0x53, 0x4F, 0x53, 0xAA`) directly to the hardware transmitter buffer. Triggered autonomously by the panic hook system.
