use anyhow::{Error, Result};
use rquickjs::{Context, Function, Runtime};

fn log(msg: String) {
    println!("{msg}");
}

fn main() -> Result<()> {
    let runtime = Runtime::new()?;
    let context = Context::full(&runtime)?;

    context.with(|ctx| {
        let global = ctx.globals();
        let fun = Function::new(ctx.clone(), log)?.with_name("log")?;
        global.set("log", fun)?;

        ctx.eval(r#"log("Hello, World!")"#)?;

        // Evaluate the JavaScript code
        let result: String = ctx.eval_file("examples/rquickjs.js").unwrap();
        // Print the result
        println!("{}", result);
        Ok::<_, Error>(())
    })?;

    Ok(())
}
