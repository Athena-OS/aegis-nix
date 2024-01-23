use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn install(cores: String, jobs: String, keep: bool) {
    // The init logging is called at the beginning of main.rs

    let mut install_args = vec![
        "--no-root-password",
        "--no-bootloader",
        "--cores",
        &cores,
        "--max-jobs",
        &jobs,
    ];

    if keep {
        install_args.push("--keep-going");
    }

    let mut install_cmd = Command::new("nixos-install")
        .args(&install_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start nixos-install");

    let stdout_handle = install_cmd.stdout.take().expect("Failed to open stdout pipe");
    std::thread::spawn(move || {
        let reader = BufReader::new(stdout_handle);
        for line in reader.lines() {
            if let Ok(line) = line {            
                log::info!("{}", line);
            }
        }
    });

    let stderr_handle = install_cmd.stderr.take().expect("Failed to open stderr pipe");
    std::thread::spawn(move || {
        let reader = BufReader::new(stderr_handle);
        for line in reader.lines() {
            if let Ok(line) = line {
                // info!() instead of error!() because nix commands output work in stderr instead of stdout because stderr is not buffered
                log::info!("{}", line);
            }
        }
    });

    let status = install_cmd.wait();
    match status {
        Ok(exit_status) => match exit_status.code() {
            Some(code) => log::info!("Exited with status: {}", code),
            None => log::info!("Process terminated without an exit code."),
        },
        Err(err) => log::error!("Failed to wait for process: {}", err),
    }
}