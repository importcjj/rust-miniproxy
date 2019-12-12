use crate::ciper::CiperTcpStream;
use crate::config::ServerConfig;
use crate::password::decode_password;
use crate::socks5::serve_socks5;
use crate::{spawn_and_log_err, Result};
use async_std::net::ToSocketAddrs;
use log::info;

pub async fn run_server(config: ServerConfig) -> Result<()> {
    let addr = format!("{}:{}", config.host.unwrap(), config.port.unwrap());
    let password = config.password.unwrap();
    info!("server listening on {}...", addr);
    info!("{}", password);

    let password = decode_password(&password)?;

    serve(addr, password).await
}

#[cfg(feature = "gkd")]
async fn serve<A: ToSocketAddrs>(addr: A, ciper_password: Vec<u8>) -> Result<()> {
    info!("powered by gkd-rs");
    use gkd::Server;

    let server = Server::bind(addr).await?;
    while let Some((stream, _)) = server.accept().await {
        let stream = CiperTcpStream::new(stream, ciper_password.clone());
        spawn_and_log_err(serve_socks5(stream));
    }
    Ok(())
}

#[cfg(not(feature = "gkd"))]
async fn serve<A: ToSocketAddrs>(addr: A, ciper_password: Vec<u8>) -> Result<()> {
    use async_std::net::TcpListener;
    use async_std::prelude::*;

    let server = TcpListener::bind(addr).await?;
    while let Some(stream) = server.incoming().next().await {
        let stream = stream?;
        let stream = CiperTcpStream::new(stream, ciper_password.clone());
        spawn_and_log_err(serve_socks5(stream));
    }

    Ok(())
}
