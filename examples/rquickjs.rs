use anyhow::Result;
use rquickjs::{Context, Runtime};

fn main() -> Result<()> {
    // Create a new QuickJS runtime and context
    let runtime = Runtime::new()?;
    let context = Context::full(&runtime)?;

    // Create a JavaScript code to execute
    // let code = r#"
    //     function greet(name) {
    //         return `Hello, ${name}!`;
    //     }
    //     greet("World");
    // "#;

    context.with(|ctx| {
        // Evaluate the JavaScript code
        let result: String = ctx.eval_file("examples/rquickjs.js").unwrap();
        // Print the result
        println!("{}", result);
    });

    Ok(())
}
