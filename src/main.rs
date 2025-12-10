use anyhow::Result;
use clap::Parser; // 必须引入这个，否则不能调用 .parse()
use mini_grep::{Args, run}; // 引入你自己的结构体 Args 和函数 run
fn main() -> Result<()> {
    let args = Args::parse();
    run(args)
}
