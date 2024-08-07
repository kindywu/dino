use std::collections::BTreeSet;

use anyhow::Result;
use glob::glob;

fn main() -> Result<()> {
    let files: BTreeSet<_> = glob("/root/workspace/rust/dino/demo/**/*.ts")?
        .filter_map(|f| f.ok())
        .filter(|f| !f.starts_with("/root/workspace/rust/dino/demo/build/"))
        .collect();

    for file in files {
        println!("{file:?}");
    }
    Ok(())
}
