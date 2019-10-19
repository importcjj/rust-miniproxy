use async_std::task;
use myproxy::local::run_local;
use myproxy::Result;

fn main() -> Result<()> {
    task::block_on(run_local("0.0.0.0:9998"))
}
