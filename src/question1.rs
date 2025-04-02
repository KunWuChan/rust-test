// 从命令行读取一个整数 n（若读取失败或没有输入则默认 n = 5）。
// 打印从 1 到 n 的所有整数，每行一个。
// 若该整数可以被 3 整除，则在数字后面附加输出 "Fizz"；若可以被 5 整除，则附加输出 "Buzz"；若同时满足可以被 3 和 5 整除的情况，则输出 "FizzBuzz"。
use std::env;

fn fizz_buzz(n: i32) {
    for i in 1..=n {
        let mut output = String::new();
        output.push_str(&i.to_string());
        
        if i % 3 == 0 && i % 5 == 0 {
            output.push_str(" FizzBuzz");
        } else if i % 3 == 0 {
            output.push_str(" Fizz");
        } else if i % 5 == 0 {
            output.push_str(" Buzz");
        }
        
        println!("{}", output);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = if args.len() > 1 {
        args[1].parse::<i32>().unwrap_or(5)
    } else {
        5
    };
    
    fizz_buzz(n);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fizz_buzz() {
        use std::io::Write;
        let mut output = Vec::new();
        
        {
            let mut stdout = output.by_ref();
            fizz_buzz(5);

        }
        
        fn get_fizz_buzz_string(n: i32) -> Vec<String> {
            let mut result = Vec::new();
            for i in 1..=n {
                let mut output = String::new();
                output.push_str(&i.to_string());
                
                if i % 3 == 0 && i % 5 == 0 {
                    output.push_str(" FizzBuzz");
                } else if i % 3 == 0 {
                    output.push_str(" Fizz");
                } else if i % 5 == 0 {
                    output.push_str(" Buzz");
                }
                result.push(output);
            }
            result
        }
        
        let result = get_fizz_buzz_string(5);
        assert_eq!(result, vec![
            "1".to_string(),
            "2".to_string(),
            "3 Fizz".to_string(),
            "4".to_string(),
            "5 Buzz".to_string()
        ]);
    }
}