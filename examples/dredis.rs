use tokio::net::TcpListener;
use anyhow::Result;
use tracing::info;

const BUF_SIZE: usize  =4096; 

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::fmt::init();
    // build listener

    let addr = "0.0.0:6379";

    let listener = TcpListener::bind(addr).await?;
    info!("Dredis: listening on: {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);
        tokio::spawn(async move {
            process_redis_connection(stream).await;
        });
    }
    Ok(())
} 

async fn process_redis_connection(stream: tokio::net::TcpStream) -> Result<()>{
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read(&mut buf) {
        
        }
    }
    
Ok(())

}