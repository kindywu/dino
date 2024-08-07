use anyhow::Result;

fn main() -> Result<()> {
    let lines = [b"a", b"b", b"c"];
    let mut hasher = blake3::Hasher::new();
    for line in lines {
        hasher.update(line);
    }
    let hash = hasher.finalize().to_string();
    println!("{hash}");
    Ok(())
}
