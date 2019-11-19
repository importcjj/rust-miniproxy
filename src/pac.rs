use crate::Result;
use async_std::fs;
use async_std::net::TcpStream;
use async_std::prelude::*;
use log::info;

pub async fn serve_pac_file(mut stream: TcpStream) -> Result<()> {
    info!("serve pac file");

    let file_contents = fs::read("proxy.pac").await?;

    stream.write_all("HTTP/1.1 200 OK\r\n".as_bytes()).await?;
    let header_content_length = format!("Content-Length: {}\r\n", file_contents.len());
    stream.write_all(header_content_length.as_bytes()).await?;
    stream.write_all("Server: minilocal".as_bytes()).await?;
    stream.write_all("Connection: close".as_bytes()).await?;
    stream
        .write_all("Content-Type: application/x-ns-proxy-autoconfig\r\n".as_bytes())
        .await?;
    stream.write_all("\r\n".as_bytes()).await?;
    stream.write_all(&file_contents).await?;

    Ok(())
}
