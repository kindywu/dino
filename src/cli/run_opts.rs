use clap::Parser;

use super::CmdExector;

#[derive(Debug, Parser)]
pub struct RunOpts {
    #[arg(short, long)]
    pub name: String,
}

impl CmdExector for RunOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("run {}", self.name);
        Ok(())
    }
}
