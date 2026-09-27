# Serial Monitor for Zed

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

A small native serial monitor that runs in Zed's integrated terminal. It lists
serial ports, lets you choose a port and baud rate, and streams received bytes
to the terminal.

> This is not currently an installable Zed extension with a custom panel. Zed's
> extension API does not expose a general-purpose custom panel or click-event
> API. This project uses Zed project tasks to launch a native Rust program
> instead.

## Requirements

- Rust toolchain
- Linux: `libudev` development files for the `serialport` crate
- A serial device and permission to access it

## Run in Zed

Open this project in Zed, open the task picker (`task: spawn`), and choose
**Serial Monitor: connect**. Select the port and baud rate in the integrated
terminal. Press Ctrl+C to disconnect.

Choose **Serial Monitor: list ports** to show detected devices without opening
one. These project tasks are defined in `.zed/tasks.json`.

## Run from a terminal

Start an interactive session:

```sh
cargo run -p zed-serial-monitor-cli --release
```

List ports:

```sh
cargo run -p zed-serial-monitor-cli --release -- --list
```

Connect directly:

```sh
cargo run -p zed-serial-monitor-cli --release -- --port /dev/ttyUSB0 --baud 115200
```

On Windows, pass a port such as `COM3`; on macOS, use the device path shown by
`--list`.

## Development & Testing

```sh
# Format & Lint
cargo fmt --check
cargo clippy --package zed-serial-monitor-cli -- -D warnings
cargo clippy --target wasm32-wasip2 -- -D warnings

# Run CLI Unit Tests (4 unit tests)
cargo test --package zed-serial-monitor-cli

# Build Zed WASM Extension
cargo build --target wasm32-wasip2 --release
```

Real-device testing is still needed on Linux, macOS, and Windows. On Linux,
access may require membership in the `dialout` group or the group that owns the
device node.
