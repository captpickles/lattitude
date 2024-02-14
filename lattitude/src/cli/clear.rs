use clap::Args;

#[derive(Args, Debug, Clone)]
#[command(about = "Clear the screen", args_conflicts_with_subcommands = true)]
pub struct ClearCommand {}

impl ClearCommand {
    pub async fn run(&self) {}
}
