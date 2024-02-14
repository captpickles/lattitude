mod calibrate;
mod clear;
mod run;
mod splash;
mod unbox;

use crate::cli::splash::SplashCommand;
use calibrate::CalibrateCommand;
use clap::{Args, Parser, Subcommand};
use clear::ClearCommand;
use run::RunCommand;
use unbox::UnboxCommand;

#[derive(Debug, Clone, Parser)]
#[command(
author,
version = env ! ("CARGO_PKG_VERSION"),
about = "L'åttitüdé",
long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Clear(ClearCommand),
    Unbox(UnboxCommand),
    Splash(SplashCommand),
    Run(RunCommand),
    Calibrate(CalibrateCommand),
}

impl Command {
    pub async fn run(&self) {
        match self {
            Command::Clear(inner) => inner.run().await,
            Command::Unbox(inner) => inner.run().await,
            Command::Splash(inner) => inner.run().await,
            Command::Run(inner) => inner.run().await,
            Command::Calibrate(inner) => inner.run().await,
        }
    }
}
