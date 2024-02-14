use clap::Args;

#[derive(Args, Debug, Clone)]
#[command(
    about = "Calibrate the touch screen",
    args_conflicts_with_subcommands = true
)]
pub struct CalibrateCommand {}

impl CalibrateCommand {
    pub async fn run(&self) {}
}
