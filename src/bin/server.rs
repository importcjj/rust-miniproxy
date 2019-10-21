use async_std::task;
use clap::{App, Arg};
use log::error;
use miniproxy::config::ServerConfig;
use miniproxy::daemon::set_daemon;
use miniproxy::server::run_server;

const SERVER_NAME: &'static str = "miniserver";

fn main() {
    env_logger::init();
    let matches = App::new(SERVER_NAME)
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("host").short("h").long("host").help("host"))
        .arg(Arg::with_name("port").short("p").long("port").help("port"))
        .arg(Arg::with_name("daemon").short("d").help("daemonize"))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("config path"),
        )
        .get_matches();

    let daemonize = matches.is_present("daemon");
    let mut config = match matches.value_of("config") {
        Some(ref path) => match ServerConfig::load_from_file(path) {
            Ok(config) => config,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        },
        None => ServerConfig::default(),
    };

    if let Some(host) = matches.value_of("host") {
        config.host = Some(host.to_string());
    }

    if let Some(port) = matches.value_of("port") {
        config.port = match port.parse::<u16>() {
            Ok(port) => Some(port),
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        }
    }

    if daemonize {
        set_daemon(SERVER_NAME);
    }

    if let Err(e) = task::block_on(run_server(config)) {
        error!("{:?}", e);
    }
}
