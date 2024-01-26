use crate::args;
use crate::args::{DesktopSetup, ThemeSetup, DMSetup, ShellSetup, BrowserSetup, TerminalSetup, PartitionMode};
use crate::functions::*;
use crate::internal::*;
use serde::{Deserialize, Serialize};
use std::path::{PathBuf};


#[derive(Serialize, Deserialize)]
struct Config {
    partition: Partition,
    bootloader: Bootloader,
    locale: Locale,
    networking: Networking,
    users: Vec<Users>,
    rootpass: String,
    params: InstallParams,
    desktop: String,
    theme: String,
    displaymanager: String,
    browser: String,
    terminal: String,
    snapper: bool,
    flatpak: bool,
    zramd: bool,
    extra_packages: Vec<String>,
    kernel: String,
}

#[derive(Serialize, Deserialize)]
struct Partition {
    device: String,
    mode: PartitionMode,
    efi: bool,
    swap: bool,
    swap_size: String,
    partitions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Bootloader {
    r#type: String,
    location: String,
}

#[derive(Serialize, Deserialize)]
struct Locale {
    locale: Vec<String>,
    keymap: String,
    timezone: String,
}

#[derive(Serialize, Deserialize)]
struct Networking {
    hostname: String,
}

#[derive(Serialize, Deserialize)]
struct Users {
    name: String,
    password: String,
    hasroot: bool,
    shell: String,
}

#[derive(Serialize, Deserialize)]
struct InstallParams {
    cores: String,
    jobs: String,
    keep: bool,
}

pub fn read_config(configpath: PathBuf) {
    let data = std::fs::read_to_string(&configpath);
    match &data {
        Ok(_) => {
            log::debug!("[ \x1b[2;1;32mOK\x1b[0m ] Read config file {configpath:?}");
        }
        Err(e) => {
            crash(
                format!("Read config file {configpath:?}  ERROR: {}", e),
                e.raw_os_error().unwrap(),
            );
        }
    }
    let config: std::result::Result<Config, serde_json::Error> =
        serde_json::from_str(&data.unwrap());
    match &config {
        Ok(_) => {
            log::debug!("[ \x1b[2;1;32mOK\x1b[0m ] Parse config file {configpath:?}",);
        }
        Err(e) => {
            crash(format!("Parse config file {configpath:?}  ERROR: {}", e), 1);
        }
    }
    //////
    let config: Config = config.unwrap();
    log::info!("Block device to use : {}", config.partition.device);
    log::info!("Partitioning mode : {:?}", config.partition.mode);
    log::info!("Partitioning for EFI : {}", config.partition.efi);
    log::info!("Swap partition : {}", config.partition.swap);
    let mut partitions: Vec<args::Partition> = Vec::new();
    for partition in config.partition.partitions {
        partitions.push(args::Partition::new(
            partition.split(':').collect::<Vec<&str>>()[0].to_string(),
            partition.split(':').collect::<Vec<&str>>()[1].to_string(),
            partition.split(':').collect::<Vec<&str>>()[2].to_string(),
        ));
    }
    let device = PathBuf::from("/dev/").join(config.partition.device.as_str());
    partition::partition(
        device,
        config.partition.mode,
        config.partition.efi,
        config.partition.swap,
        config.partition.swap_size,
        &mut partitions,
    );
    println!();
    base::install_nix_config();
    println!();
    log::info!("Installing bootloader : {}", config.bootloader.r#type);
    log::info!("Installing bootloader to : {}", config.bootloader.location);
    if config.bootloader.r#type == "grub-efi" {
        base::install_bootloader_efi(PathBuf::from(config.bootloader.location));
    } else if config.bootloader.r#type == "grub-legacy" {
        base::install_bootloader_legacy(PathBuf::from(config.bootloader.location));
    }
    println!();
    // Set locales at the beginning to prevent some warning messages about "Setting locale failed"
    log::info!("Adding Locales : {:?}", config.locale.locale);
    locale::set_locale(config.locale.locale.join(" "));
    log::info!("Using keymap : {}", config.locale.keymap);
    locale::set_keyboard(config.locale.keymap.as_str());
    log::info!("Setting timezone : {}", config.locale.timezone);
    locale::set_timezone(config.locale.timezone.as_str());
    println!();
    log::info!("Hostname : {}", config.networking.hostname);
    network::set_hostname(config.networking.hostname.as_str());
    println!();
    println!("---------");
    log::info!("Installing desktop : {:?}", config.desktop);
    match config.desktop.to_lowercase().as_str() {
        "gnome" => { //Note that the value on this match statement must fit the name in desktops.py of aegis-gui (then they are lowercase transformed)
            desktops::install_desktop_setup(DesktopSetup::Gnome);
        },
        "xfce refined" => desktops::install_desktop_setup(DesktopSetup::XfceRefined),
        "xfce picom" => desktops::install_desktop_setup(DesktopSetup::XfcePicom),
        "none/diy" => desktops::install_desktop_setup(DesktopSetup::None),
        _ => log::info!("No desktop setup selected!"),
    }
    println!();
    log::info!("Installing theme : {:?}", config.theme);

    match config.theme.to_lowercase().as_str() {
        "graphite" => themes::install_theme_setup(ThemeSetup::Graphite), //Note that the value on this match statement must fit the name in themes.py of aegis-gui (then they are lowercase transformed)
        "sweet" => themes::install_theme_setup(ThemeSetup::Sweet),
        _ => log::info!("No theme setup selected!"),
    }
    println!();
    log::info!("Installing display manager : {:?}", config.displaymanager);
    match config.displaymanager.to_lowercase().as_str() {
        "gdm" => {
            displaymanagers::install_dm_setup(DMSetup::Gdm);
        },
        "lightdm neon" => {
            displaymanagers::install_dm_setup(DMSetup::LightDMNeon);
        },
        _ => log::info!("No display manager setup selected!"),
    }

    println!();
    log::info!("Installing browser : {:?}", config.browser);
    /*if let Some(browser) = &config.browser {
        browsers::install_browser_setup(*browser);
    }*/
    match config.browser.to_lowercase().as_str() {
        "firefox" => {
            browsers::install_browser_setup(BrowserSetup::Firefox);
        },
        _ => log::info!("No browser setup selected!"),
    }
    println!();
    // Terminal configuration //
    log::info!("Installing terminal : {:?}", config.terminal);
    match config.terminal.to_lowercase().as_str() {
        "alacritty" => {
            terminals::install_terminal_setup(TerminalSetup::Alacritty);
        },
        "kitty" => {
            terminals::install_terminal_setup(TerminalSetup::Kitty);
        },
        _ => log::info!("No terminal setup selected!"),
    }
    for i in 0..config.users.len() {
        log::info!("Creating user : {}", config.users[i].name);
        //log::info!("Setting use password : {}", config.users[i].password);
        log::info!("Enabling root for user : {}", config.users[i].hasroot);
        log::info!("Setting user shell : {}", config.users[i].shell);

        match config.users[i].shell.to_lowercase().as_str() {
            "bash" => shells::install_shell_setup(ShellSetup::Bash),
            "fish" => shells::install_shell_setup(ShellSetup::Fish),
            "zsh" => shells::install_shell_setup(ShellSetup::Zsh),
            _ => log::info!("No shell setup selected!"),
        }
        users::new_user(
            config.users[i].name.as_str(),
            config.users[i].password.as_str(),
            false,
        );
        println!("---------");
    }
    println!();
    //log::info!("Setting root password : {}", config.rootpass);
    users::root_pass(config.rootpass.as_str());
    println!();
    log::info!("Install Athena OS");
    install::install(config.params.cores, config.params.jobs, config.params.keep);
    println!();
    log::info!("Installation log file copied to /var/log/aegis.log");
    files_eval(files::create_directory("/mnt/var/log"), "create /mnt/var/log");
    files::copy_file("/tmp/aegis.log", "/mnt/var/log/aegis.log");
    if config.bootloader.r#type == "grub-efi" {
        partition::umount("/mnt/boot");
    }
    partition::umount("/mnt");
    log::info!("Installation finished! You may reboot now!");
}
