// 请从命令行读取一行字符串（例如 "apple banana pear banana apple banana"）。
// 使用空格进行拆分，统计每个单词出现的次数，并以从高到底的顺序输出。
// 如果出现次数相同，按单词本身的字典序从小到大排序输出。
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("请提供一行字符串作为输入，例如：cargo run apple banana pear");
        return;
    }
    let input = args[1..].join(" ");

    let words: Vec<&str> = input.split_whitespace().collect();

    let mut word_count = HashMap::new();
    for word in words {
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }

    let mut word_vec: Vec<(String, u32)> = word_count.into_iter().collect();
    word_vec.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| {
            a.0.cmp(&b.0)
        })
    });

    for (word, count) in word_vec {
        println!("{}: {}", word, count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_word_count() {
        let input = "apple banana pear banana apple banana".to_string();
        let words: Vec<&str> = input.split_whitespace().collect();

        let mut word_count = HashMap::new();
        for word in words {
            *word_count.entry(word.to_string()).or_insert(0) += 1;
        }

        let mut word_vec: Vec<(String, u32)> = word_count.into_iter().collect();
        word_vec.sort_by(|a, b| {
            b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
        });

        assert_eq!(word_vec, vec![
            ("banana".to_string(), 3),
            ("apple".to_string(), 2),
            ("pear".to_string(), 1),
        ]);
    }
}