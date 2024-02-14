use clap::Args;

#[derive(Args, Debug, Clone)]
#[command(
    about = "Display only the splash screen",
    args_conflicts_with_subcommands = true
)]
pub struct SplashCommand {}

impl SplashCommand {
    pub async fn run(&self) {}
}
