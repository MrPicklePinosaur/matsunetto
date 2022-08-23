use pino_argparse::{Cli, Command, Flag, FlagParse};

use crate::api::pull_metrics;

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
    println!("{:?}", pull_metrics()?);
    Ok(())
}
