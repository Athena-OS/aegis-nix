use std::process::{Command};

pub fn install() {

    Command::new("nixos-install")
        .args(["--no-root-passwd"])
        .output()
        .expect("Failed to install the system!");
}