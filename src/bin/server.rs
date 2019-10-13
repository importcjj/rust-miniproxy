use async_std::task;
use myproxy::server::run_server;
use myproxy::Result;

fn main() -> Result<()> {
    task::block_on(run_server("127.0.0.1:9999"))
}
