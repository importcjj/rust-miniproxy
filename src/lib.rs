pub mod ciper;
pub mod local;
pub mod server;
mod socks5;

use async_std::future::Future;
use async_std::task;
use std::panic::UnwindSafe;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn spawn_and_log_err<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("conn err: {:?}", e);
        }
    })
}
