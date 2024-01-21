use crate::internal::*;
use std::process::Command;

pub fn new_user(username: &str, password: &str, do_hash_pass: bool) {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "username =.*",
            &(format!("username = \"{}\"", username)),
        ),
        "Set username",
    );
    if do_hash_pass {
        let hashed_pass = &*hash_pass(password).stdout;
        let _password = match std::str::from_utf8(hashed_pass) {
            Ok(v) => v,
            Err(e) => panic!("Failed to hash password, invalid UTF-8 sequence {}", e),
        };
    }
}

pub fn hash_pass(password: &str) -> std::process::Output {
    let output = Command::new("openssl")
        .args(["passwd", "-6", password])
        .output()
        .expect("Failed to hash password");
    
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "hashed =.*",
            &(format!("hashed = \"{}\"", String::from_utf8_lossy(&output.stdout).to_string())),
        ),
        "Set password hash",
    );
    output
}

pub fn root_pass(root_pass: &str) {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "hashedRoot =.*",
            &(format!("hashedRoot = \"{}\"", root_pass)),
        ),
        "Set root password hash",
    );
}
