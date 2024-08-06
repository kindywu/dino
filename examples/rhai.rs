use rhai::{Engine, EvalAltResult};

fn main() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    engine.eval_file("examples/fibonacci.rhai".into())?;

    Ok(())
}
