use async_std::task;
use myproxy::local::run_local;
use myproxy::Result;

fn main() -> Result<()> {
    task::block_on(run_local("127.0.0.1:9998"))
}
