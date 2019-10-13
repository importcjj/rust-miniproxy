use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::net::ToSocketAddrs;

use crate::spawn_and_log_err;
use crate::Result;

pub async fn run_local(addr: impl ToSocketAddrs) -> Result<()> {
    let server = TcpListener::bind(addr).await?;
    
    while let Some(stream) = server.incoming().next().await {
        let stream = stream?;
        spawn_and_log_err(serve_conn(stream));
    }
    Ok(())
}


async fn serve_conn(mut stream: TcpStream) -> Result<()> {
    let mut buf = vec![0_u8; 1024];
    let n = stream.read(&mut buf).await?;

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    match req.parse(&buf[0..n]) {
        Ok(_) => {
            println!("{:?}", req);
        }
        // 解析失败 则直接理解为socks5代理
        Err(err) => println!("failed to parse {}", err)
    }
    

    
    Ok(())
}