use rhai::{Engine, EvalAltResult};

fn main() -> Result<(), Box<EvalAltResult>> {
    // Define external function
    fn compute_something(x: i64) -> bool {
        (x % 40) == 0
    }

    // Create scripting engine
    let mut engine = Engine::new();

    // Register external function as 'compute'
    engine.register_fn("compute", compute_something);

    engine.eval_file("examples/fibonacci.rhai".into())?;

    Ok(())
}
