use async_std::task;
use miniproxy::local::run_local;
use miniproxy::Result;

fn main() -> Result<()> {
    env_logger::init();
    task::block_on(run_local("0.0.0.0:9998"))
}
