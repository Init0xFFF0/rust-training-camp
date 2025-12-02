use anyhow::{Context, Result};
use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(version,about,long_about = None)]
struct Grep {
    pattern: String,

    path: PathBuf,
}
fn main() -> Result<()> {
    let grep = Grep::parse();

    let contents = fs::read_to_string(&grep.path)
        .with_context(|| format!("could not read file `{}`", grep.path.display()))?;
    // println!("read fille:{:?}", contents);

    for line in contents.lines() {
        if line.contains(&grep.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
    // println!("Searching for: {}", grep.pattern);
    // println!("In file: {}", grep.path.display());
}
