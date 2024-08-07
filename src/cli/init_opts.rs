use clap::Parser;

use crate::CmdExector;

#[derive(Debug, Parser)]
pub struct InitOpts {
    #[arg(short, long)]
    pub name: String,
}

impl CmdExector for InitOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("init {}", self.name);
        Ok(())
    }
}
