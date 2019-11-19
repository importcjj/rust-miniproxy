use async_std::future::select;
use async_std::net::ToSocketAddrs;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use log::{debug, error, info};

use crate::ciper::CiperTcpStream;
use crate::config::LocalConfig;
use crate::pac::serve_pac_file;
use crate::password::decode_password;
use crate::socks5::req_socks5;
use crate::spawn_and_log_err;
use crate::Result;

pub async fn run_local(config: LocalConfig) -> Result<()> {
    let addr = format!("{}:{}", config.host.unwrap(), config.port.unwrap());
    let remote_addr = config.server.unwrap();
    let password = config.password.unwrap();
    info!("MINILOCAL listening on {}", addr);
    info!("Serve [ HTTP | HTTPS | SOCKS5 ]");
    info!("PAC url http://{}/pac", addr);

    let password = decode_password(&password)?;
    let server = TcpListener::bind(addr).await?;
    while let Some(stream) = server.incoming().next().await {
        let stream = stream?;
        spawn_and_log_err(serve_conn(remote_addr.clone(), stream, password.clone()));
    }
    Ok(())
}

async fn serve_conn(
    remote_addr: impl ToSocketAddrs,
    mut stream: TcpStream,
    password: Vec<u8>,
) -> Result<()> {
    let mut buf = vec![0_u8; 1024];
    let n = stream.read(&mut buf).await?;

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let server_stream = TcpStream::connect(remote_addr).await?;
    let mut server_stream = CiperTcpStream::new(server_stream, password);

    match req.parse(&buf[0..n]) {
        Ok(_) => {
            let mut host: Option<&str> = None;
            debug!("req {:?}", req);
            for h in req.headers {
                if h.name == "Host" {
                    host = Some(std::str::from_utf8(h.value)?);
                }
            }
            let host = match host {
                Some(h) => h,
                None => {
                    error!("invalid request");
                    return Ok(());
                }
            };

            // Serve pac file
            if let Some(path) = req.path {
                if path == "/pac" {
                    return serve_pac_file(stream).await;
                }
            }

            // Do nothing
            if host.contains("127.0.0.1") {
                return Ok(());
            }

            info!("{}", host);

            server_stream = req_socks5(server_stream, host).await?;
            match req.method {
                Some("CONNECT") => {
                    stream
                        .write_all("HTTP/1.1 200 Tunnel established\r\n\r\n".as_bytes())
                        .await?;
                }
                _ => {
                    server_stream.write_all(&buf[..n]).await?;
                }
            }
        }
        // 解析失败 则直接理解为socks5代理
        Err(_) => {
            server_stream.write_all(&buf[..n]).await?;
        }
    }

    let (lr, lw) = &mut (&stream, &stream);
    let (tr, tw) = &mut (&server_stream, &server_stream);

    let copy_a = async_std::io::copy(lr, tw);
    let copy_b = async_std::io::copy(tr, lw);
    select!(copy_a, copy_b).await?;

    Ok(())
}
