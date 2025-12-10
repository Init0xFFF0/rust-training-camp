use anyhow::Result;
use clap::Parser;
use mini_grep::{Args, run};
fn main() -> Result<()> {
    let args = Args::parse();
    run(args)?;
    Ok(())
}
