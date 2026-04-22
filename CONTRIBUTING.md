# Contributing to OpenOrbit 🚀

First off, thank you for considering contributing to **OAP (Orbital Analysis Pro)**! We truly appreciate your help in keeping Earth's orbits safe and sustainable.

As an aerospace project, we prioritize **precision, safety, and clean code.**

## How Can You Help?
- **Bug Reports:** Found an anomaly in the J2 perturbation calculation or radar detection? Open an Issue.
- **Feature Requests:** Want to add solar radiation pressure models or atmospheric drag? Let's discuss it in the Discussions tab.
- **Code Contributions:** Check the "help wanted" labels in the Issues section and submit a Pull Request.

## Development Workflow
Since we use **Rust**, please ensure your development environment is properly set up:

1. **Format Your Code:** Before committing, run this command to keep the codebase clean and standard:
   ```bash
   cargo fmt
   ```
2. **Run Tests**: We have a strict "No Red Tests" policy. Ensure all integration and unit tests pass before submitting a PR:
    ```bash
    cargo test
    ```
3. **Branching**: Use descriptive branch names (e.g., `feat/lunar-gravity-model` or `fix/radar-glitch`).

## License
By contributing to OpenOrbit, you agree that your contributions will be licensed under the **Apache License 2.0**.