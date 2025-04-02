// 使用多线程并行计算某个函数的值或模拟并发任务。
// 需要创建 3 个线程同时进行下载，并在下载完成后将结果（例如“URL + 下载完成”）
// 通过消息通道（std::sync::mpsc）发送回主线程。主线程依次接收并打印结果。
use std::sync::mpsc;       
use std::thread;   
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let urls = vec!["https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe",
                    "https://github.com/rust-lang/rustc-pr-tracking/archive/refs/heads/master.zip",
                    "https://github.com/rust-lang/rustup-components-history/archive/refs/heads/master.zip"];

    for url in urls {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            tx_clone.send(format!("{} 下载完成", url)).unwrap();
        });
    }

    for _ in 0..3 {
        match rx.recv() {
            Ok(msg) => println!("{}", msg),
            Err(e) => println!("接收错误: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn test_downloads() {
        let (tx, rx) = mpsc::channel();
        let urls = vec![
            ("https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe", 1),
            ("https://github.com/rust-lang/rustc-pr-tracking/archive/refs/heads/master.zip", 2),
            ( "https://github.com/rust-lang/rustup-components-history/archive/refs/heads/master.zip", 3),
        ];

        for (url, dur) in urls {
            let tx = tx.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(dur));
                tx.send(format!("{} download complete", url)).unwrap();
            });
        }
        drop(tx);

        let mut received = vec![];
        while let Ok(msg) = rx.recv() {
            received.push(msg);
        }

        let expected = vec![
            "http://example1.com download complete",
            "http://example2.com download complete",
            "http://example3.com download complete",
        ];
        let mut received_sorted = received.clone();
        received_sorted.sort();
        let mut expected_sorted = expected.clone();
        expected_sorted.sort();
        assert_eq!(received_sorted, expected_sorted);
    }
}