use anyhow::{Error, Result};
use dino::run_bundle;
use rquickjs::{Context, Object, Runtime, String};

fn main() -> Result<()> {
    let bundle = run_bundle("fixtures/main.ts", &Default::default())?;
    println!("{:#}", bundle);

    let runtime = Runtime::new()?;
    let context = Context::full(&runtime)?;
    context.with(|ctx| {
        let global = ctx.globals();
        let ret: Object = ctx.eval(
            r#"
    (function(){async function hello(){return"hello";}return{hello:hello};})();
    "#,
        )?;
        global.set("handlers", ret)?;

        let ret: String = ctx.eval_promise(r#"await handlers.hello();"#)?.finish()?;
        println!("{ret:?}");

        Ok::<_, Error>(())
    })?;

    Ok(())
}
