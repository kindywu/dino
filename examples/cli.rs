use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}

#[derive(Debug, Parser)]
#[command(name = "dino", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "init", about = "Init dino project")]
    Init(InitOpts),
    #[command(name = "build", about = "Build dino project")]
    Build(BuildOpts),
    #[command(name = "run", about = "Run user's dino project")]
    Run(RunOpts),
}

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

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    opts.cmd.execute().await?;
    Ok(())
}
