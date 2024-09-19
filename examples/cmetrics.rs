use anyhow::Result; // 引入 Result 类型，方便错误处理
use concurrency::CmapMetrics; // 引入 CmapMetrics 结构体，用于计数和度量
use rand::Rng; // 引入随机数生成器
use std::{thread, time::Duration}; // 引入线程和时间相关的功能

const N: usize = 2; // 定义工作线程的数量
const M: usize = 4; // 定义请求线程的数量

fn main() -> Result<()> {
    // 初始化 CmapMetrics 实例
    let metrics = CmapMetrics::new();

    // 打印初始的计数器状态
    println!("{}", metrics);

    // 启动 N 个工作线程
    for idx in 0..N {
        task_worker(idx, metrics.clone())?; // 克隆 metrics，并启动工作线程
    }

    // 启动 M 个请求线程
    for _ in 0..M {
        request_worker(metrics.clone())?; // 克隆 metrics，并启动请求线程
    }

    // 主线程循环，每 2 秒打印一次计数器状态
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics); // 打印当前的计数器状态
    }
}

// 工作线程函数
fn task_worker(idx: usize, metrics: CmapMetrics) -> Result<()> {
    // 使用 thread::spawn 启动一个新的线程
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng(); // 创建一个随机数生成器

            // 随机休眠 100 毫秒到 5000 毫秒之间
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            // 增加工作线程的计数器
            metrics.inc(format!("call.thread.worker.{}", idx)).unwrap(); // 增加计数，处理可能的错误
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(()) // 返回结果
    });

    Ok(()) // 返回成功
}

// 请求线程函数
fn request_worker(metrics: CmapMetrics) -> Result<()> {
    // 使用 thread::spawn 启动一个新的线程
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng(); // 创建一个随机数生成器
            // 随机休眠 50 毫秒到 800 毫秒之间
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            // 随机选择一个页面 1 到 4
            let page = rng.gen_range(1..5);
            // 增加请求页面的计数器
            metrics.inc(format!("req.page.{}", page))?; // 增加计数，处理可能的错误
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(()) // 返回结果
    });

    Ok(()) // 返回成功
}
