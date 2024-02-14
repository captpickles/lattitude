use clap::Args;

#[derive(Args, Debug, Clone)]
#[command(
    about = "Display the unboxing screen",
    args_conflicts_with_subcommands = true
)]
pub struct UnboxCommand {}

impl UnboxCommand {
    pub async fn run(&self) {}
}
