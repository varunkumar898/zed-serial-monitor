use zed_extension_api as zed;

struct ZedSerialMonitor;

impl zed::Extension for ZedSerialMonitor {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(ZedSerialMonitor);
