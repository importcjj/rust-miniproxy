use async_std::future::select;
use async_std::net::ToSocketAddrs;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;

use crate::ciper;
use crate::socks5::req_socks5;
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
    let server_stream = TcpStream::connect("127.0.0.1:9999").await?;
    let mut server_stream = ciper::CiperTcpStream(server_stream);

    match req.parse(&buf[0..n]) {
        Ok(_) => {
            let mut path: Option<&str> = None;
            for h in req.headers {
                if h.name == "Host" {
                    path = Some(std::str::from_utf8(h.value)?);
                }
            }
            let path = path.unwrap_or("example.com");
            server_stream = req_socks5(server_stream, path).await?;
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
        Err(err) => {
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
