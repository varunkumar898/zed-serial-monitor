use zed_extension_api::{self as zed, SlashCommand, SlashCommandOutput, Worktree};

struct SerialMonitorExtension;

impl zed::Extension for SerialMonitorExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "serial-monitor" => Ok(SlashCommandOutput {
                text: "Serial Monitor for Zed\n\nTo start the serial monitor, open the Command Palette and select 'task: spawn' -> 'Serial Monitor: connect'.".to_string(),
                sections: vec![],
            }),
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }
}

zed::register_extension!(SerialMonitorExtension);
