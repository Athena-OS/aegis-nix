use std::process::{Command, Stdio};

pub fn install() {
    // https://stackoverflow.com/questions/31992237/how-would-you-stream-output-from-a-process
    let mut install_cmd =
        Command::new("nixos-install")
        .args(&["--no-root-passwd"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start nixos-install");

    let status = install_cmd.wait();
    match status {
        Ok(exit_status) => {
            match exit_status.code() {
                Some(code) => println!("Exited with status: {}", code),
                None => println!("Process terminated without an exit code."),
            }
        }
        Err(err) => eprintln!("Failed to wait for process: {}", err),
    }
}