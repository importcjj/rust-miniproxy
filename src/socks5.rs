use async_std::net::TcpStream;
use async_std::net::ToSocketAddrs;
use async_std::prelude::*;

use crate::ciper::CiperTcpStream;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use log::info;
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn serve_socks5(mut stream: CiperTcpStream) -> Result<()> {
    let mut buf = vec![0; 257];
    // SOCK5 协议详见 https://zh.wikipedia.org/wiki/SOCKS#SOCKS5

    // VER	NMETHODS	METHODS
    // 1	1	        1-255
    // let _n = stream.read(&mut buf).await?;
    let _n = stream.read(&mut buf).await?;

    // VER	METHOD
    // 1	1
    stream.write_all(&[0x05_u8, 0x00_u8]).await?;

    // VER	CMD	RSV	    ATYP	DST.ADDR	DST.PORT
    // 1	1	0x00	1	    动态	     2

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;

    match buf[1] {
        // 0x01表示CONNECT请求
        0x01 => (),
        0x02 => (),
        0x03 => (),
        _ => return Ok(()),
    }

    let port = Cursor::new(&buf[n - 2..n]).read_u16::<BigEndian>().unwrap();

    let addr = match buf[3] {
        0x01 => SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(buf[4], buf[5], buf[6], buf[7])),
            port,
        ),
        0x03 => {
            let domain = format!("{}:{}", String::from_utf8_lossy(&buf[5..n - 2]), port);
            let mut addrs = domain.to_socket_addrs().await?;
            addrs.next().unwrap()
        }
        0x04 => SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(
                Cursor::new(&buf[4..6]).read_u16::<BigEndian>().unwrap(),
                Cursor::new(&buf[6..8]).read_u16::<BigEndian>().unwrap(),
                Cursor::new(&buf[8..10]).read_u16::<BigEndian>().unwrap(),
                Cursor::new(&buf[10..12]).read_u16::<BigEndian>().unwrap(),
                Cursor::new(&buf[12..14]).read_u16::<BigEndian>().unwrap(),
                Cursor::new(&buf[14..16]).read_u16::<BigEndian>().unwrap(),
                Cursor::new(&buf[16..18]).read_u16::<BigEndian>().unwrap(),
                Cursor::new(&buf[18..20]).read_u16::<BigEndian>().unwrap(),
            )),
            port,
        ),
        _ => return Ok(()),
    };

    // VER	REP	RSV	    ATYP	BND.ADDR	BND.PORT
    // 1	1	0x00	1	    动态	    2

    stream.write_all(&[5, 0, 0, 1, 0, 0, 0, 0, 0, 0]).await?;

    // start to proxy.
    info!("connecting {:?}", addr);
    let target = TcpStream::connect(addr.clone()).await?;

    let (lr, lw) = &mut (&stream, &stream);
    let (tr, tw) = &mut (&target, &target);

    let copy_a = async_std::io::copy(lr, tw);
    let copy_b = async_std::io::copy(tr, lw);

    async_std::future::select!(copy_a, copy_b).await?;

    // 这里如果使用futures::select好像有问题
    // 所以使用async_std::future::select
    // select!(copy_a, copy_b).await?;
    // match r1 {
    //     Ok(n) => info!("Send {} bytes to {:?}", n, addr),
    //     Err(e) => error!("Failed to send to {:?}: {:?}", addr, e),
    // }

    // match r2 {
    //     Ok(n) => info!("Receive {} bytes from {:?}", n, addr),
    //     Err(e) => error!("Failed to send to minilocal: {:?}", e),
    // }

    Ok(())
}

pub async fn req_socks5(mut stream: CiperTcpStream, path: &str) -> Result<CiperTcpStream> {
    stream.write_all(&[0x05, 0x01, 0x00]).await?;
    stream.read_exact(&mut [0; 2]).await?;
    let mut data = vec![5, 1, 0];

    match path.parse::<SocketAddr>() {
        // IPV4
        Ok(SocketAddr::V4(v4)) => {
            data.push(0x01);
            data.extend_from_slice(&v4.ip().octets());
            data.write_u16::<BigEndian>(v4.port()).unwrap();
        }
        // IPV6
        Ok(SocketAddr::V6(v6)) => {
            data.push(0x05);
            data.extend_from_slice(&v6.ip().octets());
            data.write_u16::<BigEndian>(v6.port()).unwrap();
        }
        // domain
        Err(_) => {
            data.push(0x03);
            let mut parts = path.split(":");
            let domain = parts.next().unwrap();
            let port: u16 = parts.next().unwrap_or("80").parse().unwrap();
            data.push(domain.as_bytes().len() as u8);
            data.extend_from_slice(domain.as_bytes());
            data.write_u16::<BigEndian>(port).unwrap();
        }
    }
    stream.write_all(&data).await?;
    stream.read(&mut [0; 1024]).await?;

    Ok(stream)
}
