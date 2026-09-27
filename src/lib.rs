use zed_extension_api::{self as zed, Command, ContextServerId, Project, Result};

struct SerialMonitorExtension;

impl zed::Extension for SerialMonitorExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        Ok(Command {
            command: "cargo".to_string(),
            args: vec![
                "run".to_string(),
                "-p".to_string(),
                "zed-serial-monitor-cli".to_string(),
                "--release".to_string(),
            ],
            env: vec![],
        })
    }
}

zed::register_extension!(SerialMonitorExtension);
