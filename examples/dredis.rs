use std::net::SocketAddr; // 引入 SocketAddr 类型，用于处理套接字地址

use tokio::{io::AsyncWriteExt, net::TcpListener}; // 引入 Tokio 的异步写入扩展和 TCP 监听器
use anyhow::Result; // 引入 Result 类型，方便错误处理
use tracing::{info, warn}; // 引入 tracing 库，用于日志记录

const BUF_SIZE: usize = 4096; // 定义缓冲区大小

#[tokio::main] // 标记为异步主函数
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init(); // 初始化 tracing 订阅者以输出日志

    // 定义监听的地址和端口
    let addr = "0.0.0.0:6379"; // 监听所有网络接口上的 6379 端口

    // 创建 TCP 监听器，绑定到指定的地址
    let listener = TcpListener::bind(addr).await?;
    info!("Dredis: listening on: {}", addr); // 记录监听信息

    // 无限循环，接受连接
    loop {
        // 接受一个传入的连接
        let (stream, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", addr); // 记录已接受连接的信息
        
        // 为每个连接生成一个新的异步任务
        tokio::spawn(async move {
            // 处理连接，并在出现错误时记录警告
            if let Err(e) = process_redis_connection(stream, addr).await {
                warn!("Error processing connection with {} : {:?}", addr, e);
            }
        });
    }
} 

// 处理与客户端的 Redis 连接
async fn process_redis_connection(mut stream: tokio::net::TcpStream, addr: SocketAddr) -> Result<()> {
    loop {
        // 等待流可读
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE); // 创建缓冲区

        // 尝试从流中读取数据到缓冲区
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break, // 如果读取返回 0，表示连接已关闭，退出循环
            Ok(n) => {
                info!("read {} bytes", n); // 记录读取的字节数
                let line = String::from_utf8_lossy(&buf); // 将字节转换为字符串
                info!("{:?}", line); // 记录读取的内容
                stream.write_all(b"+OK\r\n").await?; // 发送简单的响应给客户端
            }
            // 如果没有数据可读，继续循环
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            // 处理其他错误
            Err(e) => {
                return Err(e.into()); // 返回错误
            } 
        }
    }
    warn!("Connection {} addr closed", addr); // 记录连接关闭的信息
    Ok(()) // 返回成功
}
