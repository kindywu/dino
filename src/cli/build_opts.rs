use clap::Parser;

use super::CmdExector;

#[derive(Debug, Parser)]
pub struct BuildOpts {
    #[arg(short, long)]
    pub name: String,
}

impl CmdExector for BuildOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("build {}", self.name);
        Ok(())
    }
}
