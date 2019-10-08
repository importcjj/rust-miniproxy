use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use tokio::runtime::Runtime;

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:7788").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            reader.copy(&mut writer).await.unwrap();
        });
    }
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(run_server());
}
