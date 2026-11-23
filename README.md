# SFS Rust — Flight Test

An original 2D spaceflight simulator prototype written in Rust. It aims to retain the approachable side-on rocket-flight feel while developing a more detailed simulation model. This is an early prototype, not a complete recreation of Spaceflight Simulator.

## Current prototype features

- Newtonian point-mass gravity using \(g=GM/r^2\)
- Exponential approximation for atmospheric density
- Quadratic aerodynamic drag
- Engine thrust and propellant consumption using specific impulse
- Rocket rotation, flight telemetry, pause and time warp
- Basic surface landing / crash checks

## Controls

| Key | Action |
|---|---|
| Space | Full engine throttle |
| A / D | Rotate rocket |
| P | Pause / resume |
| R | Reset flight |
| 1 / 2 / 3 / 4 | 1x / 2x / 5x / 10x time warp |
| H | Hide / show flight computer |

## Build it yourself (no GitHub Actions)

GitHub Actions are not used by this repository. You compile the game on your own computer with Cargo. Build on the operating system you want to run the game on.

### 1. Install prerequisites

**macOS (including Intel Macs running Monterey):**

Install Apple's Command Line Tools (provides the compiler and SDK headers):

```sh
xcode-select --install
```

Install Rust and Cargo:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When the installer finishes, close and reopen Terminal, then run:

```sh
rustup update stable
rustup default stable
rustc --version
cargo --version
```

**Windows:**

1. Install the Visual Studio Build Tools with the **Desktop development with C++** workload.
2. Install Rust from https://rustup.rs (choose the default MSVC toolchain).
3. Open a new PowerShell window.

**Ubuntu / Debian Linux:**

```sh
sudo apt update
sudo apt install -y build-essential pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After Rust installation, restart the terminal or load Cargo's environment:

```sh
. "$HOME/.cargo/env"
rustup update stable
```

### 2. Download the source

If Git is installed:

```sh
git clone https://github.com/red5225/sfs-rust.git
cd sfs-rust
```

Or download the ZIP from the repository page, extract it, and open Terminal / PowerShell in the extracted `sfs-rust` folder.

### 3. Check and compile

Run these commands from the project folder:

```sh
cargo check
cargo build --release
```

The first build downloads the Rust dependencies. The optimized executable is placed here:

- macOS / Linux: `target/release/sfs-rust`
- Windows: `target/release/sfs-rust.exe`

### 4. Run the game

macOS / Linux:

```sh
./target/release/sfs-rust
```

Windows PowerShell:

```powershell
.\target\release\sfs-rust.exe
```

You can also compile and run in one command from the project folder:

```sh
cargo run --release
```

### 5. Useful Cargo commands

```sh
cargo fmt
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo clean
```

- `cargo fmt` formats the Rust source.
- `cargo clippy` runs Rust's linter.
- `cargo test` runs the test suite (tests will grow as the project develops).
- `cargo clean` deletes generated build files; the next build will take longer.

### 6. Build for another operating system

The simplest and most reliable approach is to run `cargo build --release` on the target operating system. A macOS build is a macOS executable, a Windows build is an `.exe`, and a Linux build is a Linux executable. Cross-compiling requires the target platform's linker, SDK, and system libraries; it is not covered by a one-command universal cross-build setup.

## Development roadmap

1. Rocket construction editor with attachable tanks, engines, separators and fairings.
2. Staging and multi-part vehicle simulation.
3. Better numerical integration and deterministic fixed-step physics.
4. Multiple planets and moons with patched-conic / N-body options.
5. Atmosphere profiles, lift, drag, heating and terrain collisions.
6. Orbit prediction, map view, saves and mission progression.

## License

MIT. This is an independent project and is not affiliated with or endorsed by the developers of Spaceflight Simulator. It does not include SFS proprietary source code or assets.
