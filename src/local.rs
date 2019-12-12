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
use async_std::io::{Read, Write};
use futures::future::FutureExt;

pub async fn run_local(config: LocalConfig) -> Result<()> {
    let addr = format!("{}:{}", config.host.unwrap(), config.port.unwrap());
    let remote_addr = config.server.unwrap();
    let password = config.password.unwrap();
    info!("MINILOCAL listening on {}", addr);
    info!("Serve [ HTTP | HTTPS | SOCKS5 ]");
    info!("PAC url http://{}/pac", addr);

    let password = decode_password(&password)?;

    #[cfg(feature = "gkd")]
    let gkd_client = {
        info!("powered by gkd-rs");
        use gkd::Client;
        Client::connect(&remote_addr.clone(), 8).await?
    };

    let server = TcpListener::bind(addr).await?;
    while let Some(stream) = server.incoming().next().await {
        let stream = stream?;

        #[cfg(feature = "gkd")]
        let conn_to_server = gkd_client.get_connection().await?;
        #[cfg(not(feature = "gkd"))]
        let conn_to_server = TcpStream::connect(remote_addr.clone()).await?;

        let server_stream = CiperTcpStream::new(conn_to_server, password.clone());
        spawn_and_log_err(serve_conn(server_stream, stream));
    }
    Ok(())
}

async fn serve_conn<T>(
    mut server_stream: T,
    // conn_to_server: TcpStream,
    mut stream: TcpStream,
) -> Result<()>
where
    T: Read + Write + Unpin,
    for<'a> &'a T: Read + Write + Unpin,
{
    let mut buf = vec![0_u8; 1024];
    let n = stream.read(&mut buf).await?;

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

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
    let r = futures::select! {
        r1 = copy_a.fuse() => r1,
        r2 = copy_b.fuse() => r2
    };

    Ok(())
}
