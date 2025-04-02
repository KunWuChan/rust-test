// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。
use std::env;
use std::fs;
use std::process;

fn process_file(path: &str) -> std::io::Result<()> {
    let content = fs::read_to_string(path)?;
    
    let char_count = content.chars().filter(|c| *c != '\n' && *c != '\r').count();
    
    let line_count = content.lines().count();

    fs::write("output.txt", format!("Characters: {}, Lines: {}", char_count, line_count))?;
    
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        args[1].clone()
    } else {
        eprintln!("错误：请提供文件路径作为参数");
        process::exit(1);
    };

    if let Err(e) = process_file(&path) {
        eprintln!("处理文件 {} 时出错：{}", path, e);
        process::exit(1);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_process_file_success() {
        fs::write("test_input.txt", "Hello\nWorld\n").expect("应能创建测试文件");
        
        process_file("test_input.txt").expect("应能成功处理文件");
        
        let output = fs::read_to_string("output.txt").expect("应能读取输出文件");
        assert_eq!(output, "Characters: 10, Lines: 2");
        
        fs::remove_file("test_input.txt").expect("应能清理输入文件");
        fs::remove_file("output.txt").expect("应能清理输出文件");
    }

    #[test]
    fn test_process_file_no_path() {
        let args: Vec<String> = vec!["program".to_string()];
        let mut output = Vec::new();
        let result = std::panic::catch_unwind(|| {
            let path = if args.len() > 1 {
                args[1].clone()
            } else {
                eprintln!("错误：请提供文件路径作为参数");
                process::exit(1);
            };
            process_file(&path).unwrap(); 
        });
        assert!(result.is_err()); 
    }

    #[test]
    fn test_process_file_nonexistent() {
        let result = process_file("nonexistent.txt");
        assert!(result.is_err());
    }
}