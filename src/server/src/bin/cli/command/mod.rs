mod start;

use clap::Parser;

use self::start::StartCmd;

#[derive(Debug, Parser)]
#[command(
    name = "ipchat",
    about = "IPChat Server Command Line Interface",
    author = "Leo Borai <estebanborai@gmail.com>",
    max_term_width = 100,
    next_line_help = true
)]
pub enum Command {
    /// Starts an IPChat server instance
    Start(StartCmd),
}
