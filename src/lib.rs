pub mod ciper;
pub mod config;
pub mod daemon;
pub mod local;
mod pac;
pub mod password;
pub mod server;
mod socks5;

use async_std::future::Future;
use async_std::task;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn spawn_and_log_err<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            log::error!("conn err: {:?}", e);
        }
    })
}
