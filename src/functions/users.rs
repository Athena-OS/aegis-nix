use crate::internal::*;
use std::process::Command;

pub fn new_user(username: &str, password: &str, do_hash_pass: bool) {
    let mut _password = String::new();
    let config_path = "/mnt/etc/nixos/configuration.nix";
    
    let user_line = format!("username = \"{}\";", username);
    match replace_line_in_file(config_path, "username =", &user_line) {
        Ok(_) => {
            log::info!("Set username");
        }
        Err(e) => {
            log::error!("Set username ERROR: {}", e);
        }
    }
    
    if do_hash_pass {
        let hashed_pass = hash_pass(password).stdout;
        _password = String::from_utf8_lossy(&hashed_pass).into_owned();
    }
    else {
        _password = password.to_string();
    }
    
    let hash_line = format!("hashed = \"{}\";", _password);
    match replace_line_in_file(config_path, "hashed =", &hash_line) {
        Ok(_) => {
            log::info!("Set user password hash");
        }
        Err(e) => {
            log::error!("Set user password hash ERROR: {}", e);
        }
    }
}

pub fn hash_pass(password: &str) -> std::process::Output {
    let output = Command::new("openssl")
        .args(["passwd", "-6", password])
        .output()
        .expect("Failed to hash password");

    output
}

pub fn root_pass(root_pass: &str) {
    let config_path = "/mnt/etc/nixos/configuration.nix";
    
    let hash_line = format!("hashedRoot = \"{}\";", root_pass);
    match replace_line_in_file(config_path, "hashedRoot =", &hash_line) {
        Ok(_) => {
            log::info!("Set root password hash");
        }
        Err(e) => {
            log::error!("Set root password hash ERROR: {}", e);
        }
    }
}
