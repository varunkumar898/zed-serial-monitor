# Development Guide

## Current design

This repository is a native Rust command-line serial monitor, launched as a
project task inside Zed. The monitor uses the `serialport` crate for discovery
and I/O. Serial reads use a 100 ms timeout so the process can respond to
terminal interruption without blocking on the device indefinitely. Received
bytes are written directly to the terminal, preserving the device's output.

The project is not a custom Zed panel extension. Zed's supported extension API
does not provide arbitrary panel registration, HTML rendering, or callbacks
such as the `register_panel` and `on_click` pseudocode sometimes shown in
examples. Zed tasks are the supported integration used here.

## Workspace architecture

This repository is structured as a dual-target Cargo workspace:
- **`zed-serial-monitor` (root)**: The Zed WASM extension target (`wasm32-wasip2`), registering with Zed's extension API.
- **`zed-serial-monitor-cli` (`crates/cli`)**: The native Rust CLI tool interfacing directly with host serial hardware (`serialport`), launched seamlessly via Zed tasks.

## Zed task workflow

`.zed/tasks.json` defines tasks to launch the monitor interactively and list
ports. Run `task: spawn` from the command palette, then select the desired
Serial Monitor task. The connect task runs `cargo run -p zed-serial-monitor-cli --release`, prompts for a
device and baud rate, and displays incoming data in Zed's integrated terminal.

For a direct connection, run a one-shot task or terminal command such as:

```sh
cargo run -p zed-serial-monitor-cli --release -- --port /dev/ttyUSB0 --baud 115200
```

## Validation

```sh
cargo fmt --check
cargo clippy --package zed-serial-monitor-cli -- -D warnings
cargo clippy --target wasm32-wasip2 -- -D warnings
cargo test --package zed-serial-monitor-cli
cargo build --target wasm32-wasip2 --release
cargo run -p zed-serial-monitor-cli -- --list
```

The automated tests cover CLI argument validation and port discovery. They do
not replace testing with a physical serial device. Hardware testing should
verify port opening, matching baud rates, live output, and unplug/disconnect
behavior on each target operating system.

## Linux permissions

If opening a device returns a permission error, check the device's owning group
and add the user to that group when appropriate (commonly `dialout`). Log out
and back in after changing group membership.
