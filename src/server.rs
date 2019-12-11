use crate::ciper::CiperTcpStream;
use crate::config::ServerConfig;
use crate::password::decode_password;
use crate::socks5::serve_socks5;
use crate::{spawn_and_log_err, Result};
use async_std::io::{Read, Write};
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use log::info;

pub async fn run_server(config: ServerConfig) -> Result<()> {
    let addr = format!("{}:{}", config.host.unwrap(), config.port.unwrap());
    let password = config.password.unwrap();
    info!("server listening on {}...", addr);
    info!("{}", password);

    let password = decode_password(&password)?;
    let server = TcpListener::bind(addr).await?;
    while let Some(stream) = server.incoming().next().await {
        let stream = stream?;
        let stream = CiperTcpStream::new(stream, password.clone());
        spawn_and_log_err(serve_conn(stream));
    }

    Ok(())
}

async fn serve_conn(stream: CiperTcpStream<TcpStream>) -> Result<()> {
    serve_socks5(stream).await?;
    Ok(())
}
