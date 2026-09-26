#![forbid(unsafe_code)]

use std::time::Duration;

pub fn list_serial_ports() -> Result<Vec<serialport::SerialPortInfo>, serialport::Error> {
    serialport::available_ports()
}

pub fn connect_port(
    port_name: &str,
    baud_rate: u32,
) -> Result<Box<dyn serialport::SerialPort>, serialport::Error> {
    serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(100))
        .open()
}

#[cfg(test)]
mod tests {
    #[test]
    fn serial_port_listing_is_callable_without_hardware() {
        let result = super::list_serial_ports();
        assert!(result.is_ok(), "port discovery failed: {result:?}");
    }
}
