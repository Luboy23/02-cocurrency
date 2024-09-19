use std::net::SocketAddr;

use tokio::{io::AsyncWriteExt, net::TcpListener};
use anyhow::Result;
use tracing::{info, warn};

const BUF_SIZE: usize  =4096; 

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::fmt::init(); 
    // build listener

    let addr = "0.0.0.0:6379";

    let listener = TcpListener::bind(addr).await?;
    info!("Dredis: listening on: {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", addr);
        tokio::spawn(async move {
           if let Err(e) = process_redis_connection(stream, addr).await{
            warn!("Error processing connection with {} : {:?}",addr, e);
           }
        });
    }
} 

async fn process_redis_connection(mut stream: tokio::net::TcpStream, addr: SocketAddr) -> Result<()>{
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
        Ok(0) => break,
        Ok(n) => {
            info!("read {} bytes", n);
            let line = String::from_utf8_lossy(&buf);
            info!("{:?}", line);
            stream.write_all(b"+OK\r\n").await?;
        } 
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            continue;
        }
        Err(e) => {
            return Err(e.into());
            } 
        }
    }
    warn!("Connection {} addr closed", addr);
    Ok(())
}