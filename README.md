# Vexide Odometry & Localization Template

A clean, hardware-independent foundation for odometry and localization on VEX V5 robots using [Vexide](https://vexide.dev/).

This template is intentionally designed as a **scalable starting point** for teams that want to move beyond basic dead-reckoning. It emphasizes separation of concerns, testability on a normal PC, and a clear path toward more advanced filters (EKF, particle filter, sensor fusion, etc.).

---

## Design Goals

- **Hardware independent** – Core math and localization logic have zero dependency on VEX hardware.
- **Testable on a PC** – Run and debug the entire pipeline on your computer with fake sensors before deploying to the brain.
- **Separation of concerns** – Geometry, odometry sources, robot state, timing, and visualization are cleanly separated.
- **Scalable** – Start with simple dead-reckoning and later drop in an EKF, particle filter, GPS, distance sensors, or vision without rewriting the foundation.
- **Cross-platform** – Same codebase runs on the V5 brain (Vexide) and on the host (std) via Cargo features.

---

## Features

- Full SE(2) geometry library (`Pose2D`, `Transform2D`, `Twist2D`, `Rotation2D`, `Translation2D`)
- Pluggable `Odometry` trait – swap real sensors or simulated sources with no changes to the rest of the system
- Background localization loop (Vexide task on the brain, `std::thread` on the host)
- Shared robot state protected by mutexes
- Cross-platform monotonic clock abstraction
- Simple trajectory visualizer (SVG) for host-side testing
- Feature-flagged builds (`vex` vs `host`)

---

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Vexide](https://vexide.dev/) / `cargo-v5` for deploying to a V5 Brain
- Git

### Download
1. Download the ZIP file [here](https://github.com/brayden-hayes/Graveyard-Template/releases)
2. Navigate to where the folder was downloaded, right click, and press extract all.
3. Open VSCode, go to File \ Open Folder \  YOUR_FILE_PATH
4. Read through the comments in the library or the rest of the documentation here.

---

## Project Structure (typical)

```
src/
├── geometry/          # Pure SE(2) math (no hardware, no I/O)
│   ├── pose2d.rs
│   ├── transform2d.rs
│   ├── twist2d.rs
│   ├── rotation2d.rs
│   └── translation2d.rs
├── robot/
│   ├── robot.rs       # Robot struct + Compete impl + localization loop
│   └── odometry.rs    # Odometry trait + example implementations
├── utils/
│   ├── clock.rs       # Cross-platform monotonic clock
│   └── visualizer.rs  # SVG trajectory output for testing
└── main.rs
```

---

## Quick Start

### Running on the V5 Brain

```bash
cargo v5 run
```

Uses the default `vex` feature. Use your normal Vexide workflow if it differs.

### Testing on a PC

```bash
cargo test --features host --no-default-features
```

Or, if you only want to test the localization module:

```bash
cargo test test_localization --features host --no-default-features
```

---

## Core Concepts

### 1. Geometry (SE(2))

All poses and motions are expressed using a small, self-contained SE(2) library:

- `Pose2D` – position + heading
- `Twist2D` – body-frame velocity / incremental motion (`dx`, `dy`, `dθ`)
- `Twist2D::exp()` – converts a twist into a rigid transform (the heart of the prediction step)
- `Transform2D` – rigid body transform

This keeps the math consistent and makes it easy to later add proper Lie-group Jacobians for an EKF.

### 2. Odometry Trait

```rust
pub trait Odometry: Send + Sync {
    fn forward_distance(&mut self) -> f64;
    fn sideways_distance(&mut self) -> f64;
    fn heading(&mut self) -> f64;
}
```

Any sensor source that can provide these three values can be used. Real dead wheels + IMU, a simulated source, logged data, etc. are all just different implementations of the same trait.

### 3. Robot State & Localization Loop

`RobotState` owns the current pose and the previous sensor readings.

The localization loop (running in the background) continuously:

1. Reads the latest odometry measurements
2. Computes the SE(2) delta since the last cycle
3. Applies the delta to the pose with `transform_by`

The pose is protected by a mutex so both the background loop and the rest of the program can access it safely.

### 4. Cross-Platform Clock

A small `Clock` trait + `GlobalClock` alias provides monotonic time on both the brain (`user_uptime`) and the host (`std::time::Instant`). This is used for timestamps, `dt` calculation, and simulated sensors.

### 5. Host-Side Visualization

When running tests on a PC you can collect poses and write a simple SVG:

```rust
write_trajectory_svg(&poses, "trajectory.svg").unwrap();
```

Open the file in any browser to see the path, start/end markers, and final heading.

---

## Extending the Template

This project is deliberately minimal so teams can grow it in the direction they need:

| Goal | Suggested next step |
|------|---------------------|
| Better dead-reckoning | Add track-width / wheel-diameter calibration |
| Absolute corrections | Fuse GPS or distance sensors (EKF / PF) |
| Vision | Add AprilTag / AI Vision measurement models |
| Logging & replay | Serialize sensor packets + poses for offline analysis |
| Simulation | Drive the same `Odometry` trait from a physics sim |

Because the odometry source, the state estimator, and the geometry are separated, you can replace or upgrade any piece without touching the others.

---

## Feature Flags

| Feature | Purpose |
|---------|---------|
| `vex` | Build for the V5 brain (default) |
| `host` | Build for PC testing / CI |

Example:

```bash
cargo test --features host --no-default-features
```

---

## Philosophy

Most VEX localization code starts simple and then becomes a tangled mess of sensor reads, magic numbers, and blocking loops. This template tries to avoid that by enforcing a few rules from day one:

1. **Pure geometry first** – no hardware types in the math layer.
2. **Sensors behind a trait** – the rest of the system never knows whether the data is real or fake.
3. **Background localization** – the control code should only *read* a pose, never block on sensors.
4. **Host-first development** – if it doesn’t work on the PC with simulated data, it won’t magically work on the field.

The result is a foundation that stays understandable even as the localization system grows more sophisticated.

---

## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

---
