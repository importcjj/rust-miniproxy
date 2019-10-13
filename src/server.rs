use async_std::net::ToSocketAddrs;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;

use crate::socks5::serve_socks5;
use crate::{spawn_and_log_err, Result};

pub async fn run_server(addr: impl ToSocketAddrs) -> Result<()> {
    let server = TcpListener::bind(addr).await?;
    while let Some(stream) = server.incoming().next().await {
        let stream = stream?;
        // println!("Accepting {}", stream.peer_addr()?);
        spawn_and_log_err(serve_conn(stream));
    }

    Ok(())
}

async fn serve_conn(stream: TcpStream) -> Result<()> {
    serve_socks5(stream).await?;
    Ok(())
}
