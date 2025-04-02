use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use walkdir::WalkDir;

// 定义命令行参数
#[derive(Parser)]
#[command(name = "search_tool", about = "A tool to search keywords in text files")]
struct Args {
    /// 搜索关键词
    #[arg(short, long)]
    keyword: String,

    /// 搜索目录路径
    #[arg(short, long)]
    path: String,

    /// 是否忽略大小写
    #[arg(short = 'i', long, default_value_t = false)]
    ignore_case: bool,
}

// 搜索结果结构体
#[derive(Debug)]
struct SearchResult {
    file_path: String,
    matches: Vec<(usize, String)>, // (行号, 匹配行内容)
}

// 搜索单个文件的函数
fn search_file(
    path: PathBuf,
    keyword: String,
    ignore_case: bool,
    tx: Sender<SearchResult>,
) {
    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => return, // 文件读取失败，跳过
    };

    let lines: Vec<&str> = content.lines().collect();
    let mut matches = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let line_to_check = if ignore_case {
            line.to_lowercase()
        } else {
            line.to_string()
        };
        let keyword_to_check = if ignore_case {
            keyword.to_lowercase()
        } else {
            keyword.clone()
        };

        if line_to_check.contains(&keyword_to_check) {
            matches.push((i + 1, line.to_string())); // 记录行号（从1开始）和行内容
        }
    }

    if !matches.is_empty() {
        tx.send(SearchResult {
            file_path: path.to_string_lossy().to_string(),
            matches,
        })
        .unwrap();
    }
}

fn main() {
    // 解析命令行参数
    let args = Args::parse();

    // 创建消息通道
    let (tx, rx): (Sender<SearchResult>, Receiver<SearchResult>) = mpsc::channel();

    // 遍历目录并启动并发搜索
    let mut handles = Vec::new();
    for entry in WalkDir::new(&args.path)
        .into_iter()
        .filter_map(|e| e.ok()) // 忽略遍历中的错误
    {
        if entry.path().is_file()
            && entry.path().extension().map_or(false, |ext| ext == "txt")
        {
            let tx = tx.clone();
            let keyword = args.keyword.clone();
            let ignore_case = args.ignore_case;
            let path = entry.path().to_path_buf();

            // 为每个文件创建一个线程
            let handle = thread::spawn(move || {
                search_file(path, keyword, ignore_case, tx);
            });
            handles.push(handle);
        }
    }

    // 关闭主线程的发送端
    drop(tx);

    // 收集搜索结果
    let mut results = Vec::new();
    while let Ok(result) = rx.recv() {
        results.push(result);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出结果
    if results.is_empty() {
        println!("未找到匹配项。");
    } else {
        println!("搜索结果：");
        for result in &results {
            println!("文件: {}", result.file_path);
            for (line_num, line) in &result.matches {
                println!("  行 {}: {}", line_num, line);
            }
        }
        println!(
            "总结：找到 {} 个文件，匹配 {} 行。",
            results.len(),
            results.iter().map(|r| r.matches.len()).sum::<usize>()
        );
    }
}