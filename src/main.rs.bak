use std::error::Error;
use std::io::{self, Read, Write};
use std::process::ExitCode;
use zed_serial_monitor::{connect_port, list_serial_ports};

const DEFAULT_BAUD_RATE: u32 = 115_200;

struct Options {
    port: Option<String>,
    baud_rate: Option<u32>,
    list_only: bool,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("serial-monitor: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let options = parse_options(std::env::args().skip(1))?;
    if options.list_only {
        print_ports(&list_serial_ports()?);
        return Ok(());
    }

    let ports = list_serial_ports()?;
    let port_name = match options.port.as_deref() {
        Some(port_name) => port_name.to_owned(),
        None => select_port(&ports)?,
    };
    let baud_rate = match options.baud_rate {
        Some(baud_rate) => baud_rate,
        None if options.port.is_some() => DEFAULT_BAUD_RATE,
        None => select_baud_rate()?,
    };

    let mut port = connect_port(&port_name, baud_rate)?;
    eprintln!("Connected to {port_name} at {baud_rate} baud. Press Ctrl+C to stop.");

    let stdout = io::stdout();
    let mut output = stdout.lock();
    let mut buffer = [0_u8; 1024];
    loop {
        match port.read(&mut buffer) {
            Ok(0) => continue,
            Ok(bytes_read) => {
                output.write_all(&buffer[..bytes_read])?;
                output.flush()?;
            }
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock
                ) => {}
            Err(error) => return Err(error.into()),
        }
    }
}

fn parse_options(args: impl IntoIterator<Item = String>) -> Result<Options, String> {
    let mut options = Options {
        port: None,
        baud_rate: None,
        list_only: false,
    };
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--list" => options.list_only = true,
            "--port" | "-p" => {
                options.port = Some(args.next().ok_or("--port requires a port name")?);
            }
            "--baud" | "-b" => {
                let value = args.next().ok_or("--baud requires a number")?;
                let baud_rate = value
                    .parse::<u32>()
                    .map_err(|_| format!("invalid baud rate: {value}"))?;
                if baud_rate == 0 {
                    return Err("baud rate must be greater than zero".into());
                }
                options.baud_rate = Some(baud_rate);
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                return Err(format!(
                    "unknown argument: {arg}\n\nRun with --help for usage."
                ))
            }
        }
    }

    if options.list_only && (options.port.is_some() || options.baud_rate.is_some()) {
        return Err("--list cannot be combined with --port or --baud".into());
    }
    Ok(options)
}

fn print_help() {
    println!(
        "Serial Monitor for Zed\n\n\
         Usage:\n\
         [1mzed-serial-monitor[0m                 Choose a port and baud rate interactively\n\
         [1mzed-serial-monitor --list[0m          List detected serial ports\n\
         [1mzed-serial-monitor -p PORT -b RATE[0m Connect directly\n\n\
         Options:\n\
         -p, --port PORT   Serial port name\n\
         -b, --baud RATE   Baud rate (default: 115200)\n\
         -h, --help        Show this help"
    );
}

fn print_ports(ports: &[serialport::SerialPortInfo]) {
    if ports.is_empty() {
        println!("No serial ports found.");
        return;
    }

    println!("Available serial ports:");
    for port in ports {
        println!("  {} ({:?})", port.port_name, port.port_type);
    }
}

fn select_port(ports: &[serialport::SerialPortInfo]) -> Result<String, Box<dyn Error>> {
    if ports.is_empty() {
        return Err("no serial ports found; connect a device and try again".into());
    }

    println!("Available serial ports:");
    for (index, port) in ports.iter().enumerate() {
        println!("  {}) {} ({:?})", index + 1, port.port_name, port.port_type);
    }

    loop {
        let selection = prompt("Select a port by number or name: ")?;
        if let Ok(index) = selection.parse::<usize>() {
            if let Some(port) = index.checked_sub(1).and_then(|index| ports.get(index)) {
                return Ok(port.port_name.clone());
            }
        }
        if ports.iter().any(|port| port.port_name == selection) {
            return Ok(selection);
        }
        eprintln!("No listed port matches {selection:?}; try again.");
    }
}

fn select_baud_rate() -> Result<u32, Box<dyn Error>> {
    loop {
        let selection = prompt("Baud rate [115200]: ")?;
        if selection.is_empty() {
            return Ok(DEFAULT_BAUD_RATE);
        }
        match selection.parse::<u32>() {
            Ok(baud_rate) if baud_rate > 0 => return Ok(baud_rate),
            _ => eprintln!("Enter a positive integer baud rate."),
        }
    }
}

fn prompt(message: &str) -> io::Result<String> {
    print!("{message}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::parse_options;

    #[test]
    fn parses_port_and_baud() {
        let options = parse_options([
            "--port".into(),
            "/dev/ttyUSB0".into(),
            "-b".into(),
            "9600".into(),
        ])
        .expect("valid options should parse");
        assert_eq!(options.port.as_deref(), Some("/dev/ttyUSB0"));
        assert_eq!(options.baud_rate, Some(9600));
        assert!(!options.list_only);
    }

    #[test]
    fn rejects_zero_baud_rate() {
        assert!(parse_options(["--baud".into(), "0".into()]).is_err());
    }

    #[test]
    fn rejects_port_options_with_list_mode() {
        assert!(parse_options(["--list".into(), "--port".into(), "ttyUSB0".into()]).is_err());
    }
}
