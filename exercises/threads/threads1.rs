use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![]; // 创建一个存储线程句柄的向量
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now(); // 记录线程开始时间
            thread::sleep(Duration::from_millis(250)); // 线程休眠 250 毫秒
            println!("thread {} is complete", i); // 打印线程完成信息
            start.elapsed().as_millis() // 返回线程运行的时间（毫秒）
        }));
    }

    let mut results: Vec<u128> = vec![]; // 创建一个存储线程结果的向量
    for handle in handles {
        // 使用 handle.join() 等待线程完成并收集返回值
        let result = handle.join().expect("Thread panicked");
        results.push(result); // 将结果添加到 results 向量中
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!"); // 如果所有线程未完成，则触发 panic
    }

    println!(); // 打印一个空行
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result); // 打印每个线程的执行时间
    }
}