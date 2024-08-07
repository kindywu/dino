use std::env;

use clap::Parser;

use super::{build_project, CmdExector};

#[derive(Debug, Parser)]
pub struct BuildOpts {}

impl CmdExector for BuildOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let path = env::current_dir()?;
        build_project(&path)?;
        Ok(())
    }
}
