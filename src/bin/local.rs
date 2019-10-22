use async_std::task;
use clap::{App, Arg};
use log::error;
use miniproxy::config::LocalConfig;
use miniproxy::daemon::set_daemon;
use miniproxy::local::run_local;
const LOCAL_NAME: &'static str = "minilocal";

fn main() {
    env_logger::init();
    let matches = App::new(LOCAL_NAME)
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("host").short("h").long("host").value_name("HOST").help("host"))
        .arg(Arg::with_name("port").short("p").long("port").value_name("PORT").help("port"))
        .arg(
            Arg::with_name("server")
                .short("s")
                .long("server")
                .value_name("SERVER")
                .help("server address"),
        )
        .arg(Arg::with_name("daemon").short("d").help("daemonize"))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG")
                .help("config path"),
        )
        .get_matches();

    let daemonize = matches.is_present("daemon");
    let mut config = match matches.value_of("config") {
        Some(path) => match LocalConfig::load_from_file(path) {
            Ok(config) => config,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        },
        None => LocalConfig::default(),
    };

    if let Some(host) = matches.value_of("host") {
        config.host = Some(host.to_string());
    }

    if let Some(port) = matches.value_of("port") {
        config.port = match port.parse::<u16>() {
            Ok(port) => Some(port),
            Err(e) => {
                error!("invalid port {:?}", e);
                return;
            }
        }
    }

    if let Some(server) = matches.value_of("server") {
        config.server = Some(server.to_string());
    }

    if config.server.is_none() {
        error!("server address required");
        return;
    }

    if daemonize {
        set_daemon(LOCAL_NAME);
    }

    if let Err(e) = task::block_on(run_local(config)) {
        error!("{:?}", e);
    }
}
