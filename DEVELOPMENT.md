# Serial Monitor for Zed

A lightweight serial monitor panel for the Zed editor. Read microcontroller output directly without leaving your editor.

## What it does

- **Auto-detect COM/serial ports** (Linux `/dev/ttyUSB*`, macOS `/dev/tty.usbserial*`, Windows `COM*`)
- **Live serial output** in a dedicated panel
- **Port selector** — switch between ports without restarting
- **Baud rate control** — 9600, 57600, 115200, 230400, 460800, 921600
- **Clear output** button
- **Connect/disconnect** controls
- **No external dependencies** — uses native serialport API

Perfect for:
- Debugging ESP32 / ESP8266 code
- Monitoring STM32 serial output
- Reading sensor data in real-time
- Any embedded project that talks over UART

## Install

1. **Clone this repo into your Zed extensions folder:**
   ```bash
   git clone https://github.com/yourusername/zed-serial-monitor ~/.config/zed/extensions/zed-serial-monitor
   ```
   (Exact path depends on your OS—see below)

2. **Restart Zed**, or reload extensions via Command Palette: `zed: reload extensions`

3. **Open the Serial Monitor:**
   - Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`)
   - Run: `serial_monitor: toggle`
   - Or use the default shortcut: (configurable in `keybindings.json`)

## Usage

1. **Plug in your microcontroller** (STM32, ESP32, Arduino, etc.)
2. **Open Serial Monitor** → automatically lists available ports
3. **Select your port** from the dropdown
4. **Pick baud rate** (default 115200)
5. **Click Connect** → live output starts flowing
6. **Click Clear** to empty the buffer
7. **Click Disconnect** to stop reading

## Keybindings

Add to your Zed `keybindings.json` to set custom shortcuts:

```json
{
  "bindings": {
    "ctrl-alt-s": "serial_monitor: toggle",
    "ctrl-alt-c": "serial_monitor: clear"
  }
}
```

## Platform-specific install paths

**macOS:**
```bash
~/.config/zed/extensions/
```

**Linux:**
```bash
~/.config/zed/extensions/
```

**Windows:**
```bash
%APPDATA%\Zed\extensions\
```

## Development

### Prerequisites
- Rust 1.70+
- Zed development headers

### Build locally

```bash
cargo build --release
```

### Load into Zed for testing
1. Place this folder in your extensions directory (above)
2. Run in Zed: `zed: reload extensions`

### Debug

Enable logging by setting:
```bash
RUST_LOG=debug zed
```

## Features (current)

- ✅ Port detection
- ✅ Basic connect/disconnect
- ✅ Live serial output display
- ✅ Baud rate selector
- ✅ Clear buffer button
- ✅ Multi-platform support

## Planned

- 🔲 Autoscroll with pause button
- 🔲 Line filtering / regex search
- 🔲 Timestamp injection
- 🔲 Save/export output to file
- 🔲 Syntax highlighting for known formats (JSON, hex dumps)
- 🔲 Data rate visualization
- 🔲 Character encoding selector (UTF-8, Latin-1, etc.)

## Troubleshooting

**Port not showing up?**
- Check if your device drivers are installed (STM32, CH340, etc.)
- Try: `ls /dev/tty*` (macOS/Linux) or Device Manager (Windows)
- Unplug and replug the device

**Permission denied on Linux?**
```bash
# Add your user to the dialout group
sudo usermod -a -G dialout $USER
newgrp dialout
```

**Garbage characters?**
- Verify baud rate matches your device configuration
- Check USB cable quality (bad cables cause data corruption)

**Panel doesn't open?**
- Run `zed: reload extensions` and try again
- Check logs: `RUST_LOG=debug zed`

## Contributing

Pull requests welcome! Areas we need help with:
- Windows serial port detection edge cases
- macOS Bluetooth serial ports
- Better error messages
- UI polish

## License

MIT — use freely, modify, share.

## Support this project

If this saves you time, consider:
- Leaving a ⭐ on GitHub
- Filing detailed bug reports
- Submitting feature requests
- Contributing code improvements

## FAQ

**Q: Can I log output to a file?**
A: Not yet—but you can copy the buffer and paste it. File export coming soon.

**Q: Will this slow down Zed?**
A: No. Serial reading happens in a background thread with minimal CPU usage.

**Q: Can I use this with Bluetooth serial ports?**
A: macOS Bluetooth should work automatically. Linux support is planned.

**Q: Why not just use picocom/miniterm/screen?**
A: You can! This just keeps you in your editor without context-switching. Pick what works for your workflow.

## Author

Made for embedded developers who want to stay focused in one place.

---

**Feedback?** Open an issue or start a discussion!
