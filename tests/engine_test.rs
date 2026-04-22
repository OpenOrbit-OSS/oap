// OAP/tests/engine_test.rs
// Integration tests for the orbital mechanics and physics engine.

use oap::engine::orbital_mechanics::{OrbitalState, Vector3, EARTH_RADIUS};

#[test]
fn test_altitude_calculation() {
    // Setup a satellite exactly 500km above the Earth's surface along the X-axis
    let state = OrbitalState {
        position: Vector3 {
            x: EARTH_RADIUS + 500_000.0,
            y: 0.0,
            z: 0.0,
        },
        velocity: Vector3 {
            x: 0.0,
            y: 7600.0,
            z: 0.0,
        },
    };

    let altitude = state.get_altitude();

    // Assert altitude is exactly 500,000 meters
    assert!(
        (altitude - 500_000.0).abs() < 1e-5,
        "Altitude calculation drifted!"
    );
}

#[test]
fn test_orbital_propagation_determinism() {
    let mut state = OrbitalState {
        position: Vector3 {
            x: EARTH_RADIUS + 400_000.0,
            y: 0.0,
            z: 0.0,
        },
        velocity: Vector3 {
            x: 0.0,
            y: 7660.0,
            z: 0.0,
        },
    };

    let initial_x = state.position.x;

    // Propagate forward by 1 second
    state.propagate(1.0);

    // X position should decrease slightly as the orbit curves,
    // Y position should increase based on velocity.
    assert!(
        state.position.x < initial_x,
        "Gravity failed to pull the satellite!"
    );
    assert!(
        state.position.y > 0.0,
        "Tangential velocity failed to update position!"
    );
}
