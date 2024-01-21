use crate::internal::exec::*;
use crate::internal::*;
use std::path::PathBuf;

pub fn install_nix_config() {
    std::fs::create_dir_all("/mnt/etc").unwrap();
    log::info!("Generate hardware configuration.");
    exec_eval(
        exec(
            "nixos-generate-config",
            vec![
                String::from("--root"),
                String::from("/mnt"),
            ],
        ),
        "Run nixos-generate-config",
    );
    log::info!("Download latest Athena OS configuration.");
    exec_eval(
        exec(
            "curl",
            vec![
                String::from("-o"),
                String::from("/tmp/athena-nix.zip"),
                String::from("https://codeload.github.com/Athena-OS/athena-nix/zip/refs/heads/main"),
            ],
        ),
        "Getting latest Athena OS configuration.",
    );
    exec_eval(
        exec(
            "unzip",
            vec![
                String::from("/tmp/athena-nix.zip"),
                String::from("-d"),
                String::from("/tmp/"),
            ],
        ),
        "Extract Athena OS configuration archive.",
    );
    log::info!("Install Athena OS configuration.");
    exec_eval(
        exec(
            "cp",
            vec![
                String::from("-rf"),
                String::from("/tmp/athena-nix-main/nixos/home-manager"),
                String::from("/tmp/athena-nix-main/nixos/hosts"),
                String::from("/tmp/athena-nix-main/nixos/modules"),
                String::from("/tmp/athena-nix-main/nixos/pkgs"),
                String::from("/tmp/athena-nix-main/nixos/users"),
                String::from("/tmp/athena-nix-main/nixos/configuration.nix"),
                String::from("/mnt/etc/nixos/"),
            ],
        ),
        "Move Athena OS configuration to /mnt/etc/nixos/.",
    );
}

pub fn install_bootloader_efi() {
    log::info!("Set EFI Bootloader.");
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "systemd",
            "systemd",
        ),
        "Setting EFI bootloader",
    );
}

pub fn install_bootloader_legacy(device: PathBuf) {
    if !device.exists() {
        crash(format!("The device {device:?} does not exist"), 1);
    }
    let device = device.to_string_lossy().to_string();
    log::info!("Legacy bootloader installing at {}", device);
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/modules/boot/grub/default.nix",
            "/dev/sda",
            &device,
        ),
        "Setting Legacy bootloader",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "systemd",
            "grub",
        ),
        "Setting Legacy bootloader",
    );
}