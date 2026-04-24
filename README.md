# Orbital Analysis Pro (OAP)
**High-Performance Satellite Monitoring and Autonomous Collision Avoidance System**
**Version:** 1.1.0 (Sovereign Command - AOR INTEGRATED)

## 1. Overview
Orbital Analysis Pro (OAP) is a specialized, sovereign-grade software suite designed for real-time satellite trajectory propagation and autonomous conjunction assessment. Developed with a strict focus on high-fidelity orbital mechanics, cryptographic security, and minimal computational overhead, OAP serves as an onboard autonomous guardian for national satellite assets.

The system is highly optimized for deployment on resource-constrained On-Board Computers (OBCs) and Ground Control Stations, providing mission-critical telemetry and collision mitigation with deterministic microsecond performance.

## 2. Key Features
* **High-Fidelity Physics Engine:** Implements Newton's Law of Universal Gravitation combined with J2 Perturbation models to ensure ultra-precise state vector propagation, compensating for Earth's oblateness.
* **Autonomous Collision Avoidance:** Utilizes Time of Closest Approach (TCA) algorithms to predict and execute fuel-efficient evasive maneuvers autonomously.
* **Sovereign Swarm Protocol:** Employs Time-based XOR Hashing (Rolling Code) for secure inter-satellite handshake protocols, ensuring resilience against foreign signal spoofing and facilitating swarm intelligence negotiation.
* **Aerospace-Grade Failsafes:** Features a native panic-handling architecture that logs diagnostic data to a black box and transmits high-gain SOS signals upon detection of logic anomalies or radiation-induced bit flips.
* **Tactical Command Dashboard:** WebGL-accelerated, responsive 3D visualization and equatorial radar tracking for real-time situational awareness at Ground Control.
* **Adaptive Orbit Recovery (AOR):** Autonomously guides the satellite back to its reference orbital trajectory post-evasion using an integrated Proportional-Derivative (PD) controller, featuring a strict fuel-clamping mechanism to prevent propellant exhaustion.
* **FDIR (Fault Detection, Isolation, and Recovery):** Continuous onboard diagnostic system (running at 60Hz) that monitors memory integrity, orbital altitude (LEO hard-deck), and fuel anomalies. It automatically engages Safe Mode and hardware isolation to prevent catastrophic failures and preserve battery life.

## 3. System Architecture
The project follows a strict modular design ensuring scalability, memory safety, and domain isolation:
* `src/engine/`: Core astrodynamics, deterministic propagation, and gravitational anomaly modeling.
* `src/collision/`: Threat evaluation, radar spatial filtering, evasive maneuver planning, and cryptographic swarm coordination.
* `src/telemetry/`: Hardware-level binary command encoding, asynchronous data parsing, and emergency transmission.
* `dashboard_ui/`: Glassmorphism-styled, responsive mission control interface utilizing Three.js and Canvas 2D.
* `tests/`: High-coverage, extreme-scenario integration and unit testing suite.

## 4. Installation and Deployment

### 4.1 Prerequisites
* **Rust Toolchain:** Latest stable version, installed via `rustup`.
* **Build Targets:** Configured for `aarch64-unknown-linux-gnu` or custom embedded targets.
* **Control Interface:** Modern web browser with WebGL 2.0 support.

### 4.2 Building from Source
Navigate to the project root and compile the core engine with maximum optimizations:
```bash
cd OAP
cargo build --release
```

## 5. System Execution
To initialize the core simulation and verify system integrity against extreme orbital scenarios, execute:
```bash
cargo test --test extreme_simulation_test
cargo test --test coordination_test
cargo run --release
```

## 6. Automation Scripts
We provide several utility scripts to streamline development and mission control operations. All scripts are located in the `scripts/` directory and should be executed from the project root.

### 6.1 Diagnostic suite
Before pushing any code or initiating a mission, run the diagnostic suite to ensure code quality and memory safety:
```bash
./scripts/run_diagnostics.sh
```
*This script checks the Rust toolchain, runs unit tests for orbital mechanics, and performs a deep linting check.*

### 6.2 Launching the Dashboard
Since the dashboard is a sovereign-grade interface, we recommend running it via a local web server to bypass browser CORS restrictions and ensure smooth 3D rendering:
```bash
./scripts/launch_dashboard.sh
```
Once executed, access the visualizer at: `http://localhost:8000`

**Note**: Ensure you have granted execution permissions to the scripts: `chmod +x scripts/*.sh`

## 7. Licensing
This project is licensed under the **Apache License, Version 2.0** (the "License").
You may obtain a copy of the License at:

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

---

*© Copyright 2026 OpenOrbit. All rights reserved.*
