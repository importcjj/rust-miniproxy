use async_std::io;
use async_std::io::{Read, Write};
use async_std::net::TcpStream;
use async_std::pin::Pin;
use async_std::task::{Context, Poll};
#[cfg(feature = "gkd")]
use gkd::Connection;
use log::debug;
pub struct CiperTcpStream {
    #[cfg(feature = "gkd")]
    stream: Connection,
    #[cfg(not(feature = "gkd"))]
    stream: TcpStream,
    decode_password: Vec<u8>,
    encode_password: Vec<u8>,
}

impl CiperTcpStream {
    #[cfg(feature = "gkd")]
    pub fn new(stream: Connection, encode_password: Vec<u8>) -> CiperTcpStream {
        let mut decode_password = vec![0; 256];
        for (i, b) in encode_password.iter().enumerate() {
            decode_password[*b as usize] = i as u8;
        }
        Self {
            stream,
            encode_password,
            decode_password,
        }
    }
    #[cfg(not(feature = "gkd"))]
    pub fn new(stream: TcpStream, encode_password: Vec<u8>) -> CiperTcpStream {
        let mut decode_password = vec![0; 256];
        for (i, b) in encode_password.iter().enumerate() {
            decode_password[*b as usize] = i as u8;
        }
        Self {
            stream,
            encode_password,
            decode_password,
        }
    }
}

impl Read for CiperTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_read(cx, buf)
    }
}

impl Read for &CiperTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        debug!("read");
        match Pin::new(&mut &self.stream).poll_read(cx, buf) {
            ok @ Poll::Ready(Ok(_)) => {
                for b in buf {
                    *b = self.decode_password[*b as usize];
                }
                ok
            }
            r @ _ => r,
        }
    }
}

impl Write for CiperTcpStream {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self).poll_close(cx)
    }
}

impl Write for &CiperTcpStream {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        debug!("write");
        let buf: Vec<u8> = buf
            .iter()
            .map(|b| self.encode_password[*b as usize])
            .collect();
        Pin::new(&mut &self.stream).poll_write(cx, &buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Pin::new(&mut &self.stream).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Pin::new(&mut &self.stream).poll_close(cx)
    }
}
