use anyhow::Context;
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
    pub show_line_number: bool,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    // 1. 读取文件内容
    let content =
        read_to_string(&args.path).with_context(|| format!("读取文件失败 {:?}", &args.path))?;

    // 2. 准备搜索模式和闭包：根据 ignore_case 决定是否转换为小写
    let search_pattern = if args.ignore_case {
        args.pattern.to_lowercase()
    } else {
        args.pattern
    };

    // 定义核心匹配逻辑的闭包，处理大小写不敏感的情况
    let pattern_match = |word: &str| -> bool {
        if args.ignore_case {
            word.to_lowercase().contains(&search_pattern)
        } else {
            word.contains(&search_pattern)
        }
    };

    let mut word_count: usize = 0;

    // 3. 使用迭代器链进行搜索、过滤和计数
    content
        .lines() // 迭代器：按行
        .enumerate() // 迭代器：(行号-1, 行内容)
        .for_each(|(index, line)| {
            // 迭代器：遍历每一行
            let line_number = index + 1;

            // 内部迭代：按空格分割单词，并过滤匹配的单词
            let matches_in_line: Vec<String> = line
                .split_whitespace()
                .filter(|word| pattern_match(word)) // 使用闭包过滤
                .map(|word| {
                    if args.show_line_number {
                        // 如果需要显示行号，返回格式化字符串
                        format!("{},{}", line_number, word)
                    } else {
                        // 否则只返回单词
                        word.to_string()
                    }
                })
                .collect(); // 收集本行所有匹配项

            // 打印结果并更新总计数
            for matched_item in matches_in_line.iter() {
                println!("{}", matched_item);
            }
            word_count += matches_in_line.len();
        });

    println!("--- 共找到 {} 个匹配单词 ---", word_count);
    Ok(())
}
