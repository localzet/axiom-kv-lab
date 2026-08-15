use anyhow::{bail, Result};

fn main() -> Result<()> {
    if std::env::args().nth(1).as_deref() != Some("about") {
        bail!("usage: axiom-kv-lab about; run `python modelcheck.py` for the v0.2 reference experiment");
    }
    println!("Axiom KV Lab v0.2: counterexample-driven architectural evolution experiment");
    Ok(())
}
