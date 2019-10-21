use log::{error, info, warn};

#[cfg(target_os = "windows")]
pub fn set_daemon(name: &str) {
    warn!("can't be daemonized on windows yet!");
}

#[cfg(not(target_os = "windows"))]
pub fn set_daemon(name: &str) {
    use daemonize::Daemonize;
    use std::fs::File;

    let stdout = File::create(format!("/tmp/{}.out", name)).unwrap();
    let stderr = File::create(format!("/tmp/{}.err", name)).unwrap();

    let daemonize = Daemonize::new()
        .pid_file(format!("/tmp/{}.pid", name)) // Every method except `new` and `start`
        .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .group("daemon") // Group name
        .group(2) // or group id.
        .umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => info!("Success, daemonized"),
        Err(e) => error!("Error, {}", e),
    }
}
