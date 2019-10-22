use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;

use crate::ciper;
use crate::config::ServerConfig;
use crate::socks5::serve_socks5;
use crate::{spawn_and_log_err, Result};
use log::info;

pub async fn run_server(config: ServerConfig) -> Result<()> {
    let addr = format!("{}:{}", config.host.unwrap(), config.port.unwrap());
    info!("server listening on {}...", addr);
    let server = TcpListener::bind(addr).await?;
    while let Some(stream) = server.incoming().next().await {
        let stream = stream?;
        // println!("Accepting {}", stream.peer_addr()?);
        spawn_and_log_err(serve_conn(stream));
    }

    Ok(())
}

async fn serve_conn(stream: TcpStream) -> Result<()> {
    let stream = ciper::CiperTcpStream(stream);
    serve_socks5(stream).await?;
    Ok(())
}
