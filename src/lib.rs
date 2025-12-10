use anyhow::{Context, Result};
use clap::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;
#[derive(Parser, Debug)]
pub struct Args {
    pub pattern: String,
    pub path: PathBuf,
    #[arg(short = 'i', long)]
    pub ignore_case: bool,
    #[arg(short = 'n', long)]
    pub show_line_numbers: bool,
}

pub fn run(args: Args) -> Result<()> {
    let contents = read_to_string(&args.path)
        .with_context(|| format!("can not read to file{:?}", args.path))?;

    let pattern_lower = args.pattern.to_lowercase();

    for (index, line) in contents.lines().enumerate() {
        let hit: bool = if args.ignore_case {
            line.to_lowercase().contains(&pattern_lower)
        } else {
            line.contains(&args.pattern)
        };
        if hit {
            if args.show_line_numbers {
                println!("{},{}", index + 1, line);
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
