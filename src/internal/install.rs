use log::{error, info, warn};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

pub fn install() {
        
    let mut install_cmd = Command::new("nixos-install")
        .arg("--no-root-passwd")
        .stdout(Stdio::piped()) // Capture stdout
        .stderr(Stdio::piped()) // Capture stderr
        .spawn()
        .expect("Failed to start nixos-install");

    let stdout_handle = install_cmd.stdout.take().unwrap();
    let stderr_handle = install_cmd.stderr.take().unwrap();

    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout_handle);
        for line in reader.lines() {
            let line = line.expect("Failed to read stdout");
            info!("{}", line);
        }
    });

    let exit_status = install_cmd.wait().expect("Failed to wait for the package manager");

    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr_handle);
        for line in reader.lines() {
            let line = line.expect("Failed to read stderr");
            let exit_code = exit_status.code().unwrap_or(-1);
            if exit_code == 0 {
                warn!(
                    "WARN (exit code {}): {}",
                    exit_code,
                    line
                );
            }
            else {
                error!(
                    "ERROR (exit code {}): {}",
                    exit_code,
                    line
                );
            }
        }
    });

    // Wait for the stdout and stderr threads to finish
    stdout_thread.join().expect("stdout thread panicked");
    stderr_thread.join().expect("stderr thread panicked");

    if !exit_status.success() {
        // Handle the error here, e.g., by logging it
        error!("The installer failed with exit code: {}", exit_status.code().unwrap_or(-1));
    }
}