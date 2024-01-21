use crate::internal::*;
use std::process::Command;

pub fn new_user(username: &str, password: &str, do_hash_pass: bool) {
    let mut _password = String::new();

    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "username =.*",
            &(format!("username = \"{}\";", username)),
        ),
        "Set username",
    );
    if do_hash_pass {
        let hashed_pass = hash_pass(password).stdout;
        _password = String::from_utf8_lossy(&hashed_pass).into_owned();
    }
    else {
        _password = password.to_string();
    }
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "hashed =.*",
            &(format!("hashed = \"{}\";", _password)),
        ),
        "Set password hash",
    );
}

pub fn hash_pass(password: &str) -> std::process::Output {
    let output = Command::new("openssl")
        .args(["passwd", "-6", password])
        .output()
        .expect("Failed to hash password");

    output
}

pub fn root_pass(root_pass: &str) {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "hashedRoot =.*",
            &(format!("hashedRoot = \"{}\";", root_pass)),
        ),
        "Set root password hash",
    );
}
