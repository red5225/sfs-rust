# SFS Rust — Flight Test

An original 2D spaceflight simulator prototype written in Rust. It keeps the accessible side-on rocket-flight feel while using a custom physics loop for gravity, atmospheric drag, thrust, propellant use, and landing checks.

## Controls
- **Space** — full engine throttle
- **A / D** — rotate the rocket
- **P** — pause / resume
- **R** — reset the flight
- **1 / 2 / 3 / 4** — 1× / 2× / 5× / 10× time warp
- **H** — hide / show flight computer

## Physics in this prototype
- Newtonian point-mass gravity: \(g=GM/r^2\)
- Exponential atmosphere density approximation
- Quadratic aerodynamic drag
- Rocket-equation propellant flow using engine specific impulse
- Semi-implicit Euler integration with 5 ms maximum simulation steps during normal time warp

This is an early playable foundation, not yet a complete SFS recreation. The next milestones are a part-based VAB, staging, multiple engines/tanks, terrain and collision geometry, multiple celestial bodies, patched-conic/orbit prediction, save slots, and a more rigorous integrator/aerodynamic model.

## Run locally
Install Rust from https://rustup.rs, then:

```sh
cargo run --release
```

## GitHub builds
Every push to `main` builds release binaries for Windows x64, Linux x64, and Intel macOS x64. The workflow uploads each build as an Actions artifact. Push a tag like `v0.1.0` to also create a GitHub Release with the binaries attached.
