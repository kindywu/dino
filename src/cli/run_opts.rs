use std::{env, fs, path::Path};

use super::{build_project, CmdExector};
use anyhow::Result;
use clap::Parser;
use rquickjs::{Context, Function, Object, Runtime};

#[derive(Debug, Parser)]
pub struct RunOpts {}

impl CmdExector for RunOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let path = env::current_dir()?;
        run_project(&path)?;
        Ok(())
    }
}

fn run_project(path: &Path) -> Result<()> {
    let file = build_project(&path)?;
    let module = fs::read_to_string(file)?;

    let js_worker = JsWorker::try_new(&module)?;
    js_worker.run("await handlers.hello()")?;

    Ok(())
}

struct JsWorker {
    ctx: Context,
}

impl JsWorker {
    pub fn try_new(module: &str) -> Result<Self> {
        let rt = Runtime::new()?;
        let ctx = Context::full(&rt)?;

        ctx.with(|ctx| {
            let global = ctx.globals();
            let ret: Object = ctx.eval(module)?;
            global.set("handlers", ret)?;
            // setup print function
            let fun = Function::new(ctx.clone(), print)?.with_name("print")?;
            global.set("print", fun)?;

            Ok::<_, anyhow::Error>(())
        })?;

        Ok(Self { ctx })
    }

    pub fn run(&self, code: &str) -> anyhow::Result<()> {
        self.ctx.with(|ctx| {
            ctx.eval_promise(code)?.finish()?;
            Ok::<_, anyhow::Error>(())
        })?;

        Ok(())
    }
}

fn print(msg: String) {
    println!("{msg}");
}

#[cfg(test)]
mod tests {

    use std::env;

    use anyhow::Result;

    use crate::cli::run_opts::run_project;

    use super::JsWorker;

    #[tokio::test]
    async fn run_project_should_work() -> Result<()> {
        let demo_path = env::current_dir()?.join("demo");
        run_project(&demo_path)?;
        Ok(())
    }

    #[test]
    fn js_worker_should_run() {
        let code = r#"
    (function(){async function hello(){print("hello world");return"hello";}return{hello:hello};})();
    "#;
        let worker = JsWorker::try_new(code).unwrap();
        worker.run("await handlers.hello()").unwrap();
    }
}
