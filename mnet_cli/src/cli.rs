use mnet_types::DeviceState;
use pino_argparse::{Cli, Command, Flag, FlagParse};
use pino_utils::some_or_continue;

use crate::{
    api::pull_metrics,
    colors::{ansi_string, Color, Effect},
};

pub fn run_cli() {
    let cli = Cli {
        program_name: "mnet_cli",
        root_command: Command {
            command_name: "run",
            desc: "",
            handler: handle_run,
            flags: vec![],
        },
        ..Default::default()
    };
    let args = std::env::args().collect();
    cli.run(&args).unwrap();
}

fn handle_run(flagparse: FlagParse) -> Result<(), Box<dyn std::error::Error>> {
    let metrics = pull_metrics()?;
    for m in metrics {
        let info = some_or_continue!(m.metrics);
        let battery = format!("bat {}%", (info.battery * 100.) as i64);
        let mem = format!(
            "mem {}M",
            (info.memory.total - info.memory.free) / (1024 * 1024)
        );

        let output = format!("{} | {} | {}", m.name, battery, mem);
        let color = if m.state == DeviceState::Online {
            Color::Green
        } else {
            Color::StrongBlack
        };
        println!("{}", &ansi_string(&output, color, Effect::Bold));
    }
    Ok(())
}
