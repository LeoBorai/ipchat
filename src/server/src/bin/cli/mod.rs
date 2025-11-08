mod command;

use self::command::Command;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<()> {
        match self.command {
            Command::Start(cmd) => cmd.exec().await,
        }
    }
}
