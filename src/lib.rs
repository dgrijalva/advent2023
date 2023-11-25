mod client;
mod puzzle;

use clap::Parser;
pub use client::DownloadCommand;
use client::SubmitCommand;

#[derive(Parser, Debug, Clone)]
pub struct RootOpt {
    /// Year to run (default: 2023)
    #[arg(short, long, default_value_t = 2023)]
    pub year: u16,

    /// Day to run
    #[arg(short, long)]
    pub day: u8,

    /// Part to run
    #[arg(short, long)]
    pub part: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Clone, clap::Subcommand)]
enum Commands {
    Download(DownloadCommand),
    Submit(SubmitCommand),
}

impl RootOpt {
    pub fn run(&self) -> Result<(), anyhow::Error> {
        log::info!("Running day {} part {}", self.day, self.part);

        if let Some(cmd) = &self.command {
            return cmd.run(self);
        }

        todo!("Run the day");
        Ok(())
    }
}

impl Commands {
    pub fn run(&self, opt: &RootOpt) -> Result<(), anyhow::Error> {
        match self {
            Commands::Download(cmd) => cmd.run(opt),
            Commands::Submit(cmd) => cmd.run(opt),
        }
    }
}
