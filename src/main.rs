use anyhow::{Error, Result};
use dino::run_bundle;
use rquickjs::{Context, Object, Runtime};

fn main() -> Result<()> {
    let bundle = run_bundle("fixtures/main.ts", &Default::default())?;
    println!("{:#}", bundle);

    let runtime = Runtime::new()?;
    let context = Context::full(&runtime)?;
    context.with(|ctx| {
        let global = ctx.globals();
        let ret: Object = ctx.eval(bundle)?;
        global.set("handlers", ret)?;

        let ret: String = ctx.eval_promise(r#"handlers.hello();"#)?.finish()?;
        println!("{ret:?}");

        Ok::<_, Error>(())
    })?;

    Ok(())
}
