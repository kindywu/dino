use anyhow::{Error, Result};
use rquickjs::{async_with, AsyncContext, AsyncRuntime, Context, Function, Promise, Runtime};

fn log(msg: String) {
    println!("{msg}");
}

#[tokio::main]
async fn main() -> Result<()> {
    let runtime = Runtime::new()?;
    let context = Context::full(&runtime)?;

    context.with(|ctx| {
        let global = ctx.globals();
        let fun = Function::new(ctx.clone(), log)?.with_name("log")?;
        global.set("log", fun)?;

        ctx.eval(r#"log("Hello, World!")"#)?;

        let result: String = ctx.eval_file("examples/rquickjs.js").unwrap();
        // Print the result
        println!("{}", result);
        Ok::<_, Error>(())
    })?;

    let rt = AsyncRuntime::new().unwrap();
    let ctx = AsyncContext::full(&rt).await.unwrap();
    async_with!(ctx=>|ctx|{
       let func:Function = ctx.eval(r#"async function say_hello(name) {return "hello " + name} say_hello"#)?;
       println!("{:?}", func);
       let p:Promise = func.call(("world",))?;
       println!("{:?}", p);
       let result:String = p.finish()?;
       println!("{}", result);
       Ok::<_, Error>(())
    })
    .await?;

    Ok(())
}
