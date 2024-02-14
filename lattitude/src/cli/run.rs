use clap::Args;

#[derive(Args, Debug, Clone)]
#[command(about = "Run Låttitüdé", args_conflicts_with_subcommands = true)]
pub struct RunCommand {}

impl RunCommand {
    pub async fn run(&self) {}
}
