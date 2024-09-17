use anyhow::{anyhow, Result}; // 引入 Result 类型和 anyhow 宏，用于处理错误
use std::{thread, sync::mpsc, time::Duration}; // 引入线程、消息传递和时间模块

const NUM_PRODUCERS: usize = 4; // 定义常量，表示生产者线程的数量

#[allow(dead_code)] // 允许编译器忽略未使用的代码
#[derive(Debug)] // 自动为 Msg 结构体实现 Debug trait，以便可以打印
struct Msg {
    idx: usize,   // 消息的索引
    value: usize, // 消息的值
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(); // 创建一个消息通道，用于线程之间的通信

    // 创建生产者线程
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone(); // 克隆发送者，以便多个生产者使用相同的发送者
        thread::spawn(move || producer(i, tx)); // 启动生产者线程
    }

    drop(tx); // 关闭原始发送者，防止在生产者线程结束后仍然尝试发送消息

    // 创建消费者线程
    let consumer = thread::spawn(move || {
        for msg in rx { // 从接收者中接收消息
            println!("consumer: {:?}", msg); // 打印接收到的消息
        }
        println!("consumer exit"); // 消费者线程结束时的提示
        23 // 返回值
    });

    // 等待消费者线程结束，并处理可能的错误
    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?; // 使用 map_err 将错误转换为 anyhow 错误类型

    println!("secret: {}", secret); // 打印消费者线程返回的值
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>(); // 生成一个随机值
        tx.send(Msg::new(idx, value))?; // 发送消息
        let sleep_time = rand::random::<u8>() as u64 * 10; // 生成随机的睡眠时间
        thread::sleep(Duration::from_millis(sleep_time)); // 线程休眠
        // 随机退出生产者线程
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", idx); // 打印生产者线程退出的提示
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value } // 创建新的 Msg 实例
    }
}
