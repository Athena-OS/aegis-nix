use crate::args;
use crate::args::{DesktopSetup, ThemeSetup, DMSetup, ShellSetup, BrowserSetup, TerminalSetup, PartitionMode};
use crate::functions::*;
use crate::internal::*;
use serde::{Deserialize, Serialize};
use std::path::{PathBuf};
use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};


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
    ipv6: bool,
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
        "cinnamon" => desktops::install_desktop_setup(DesktopSetup::Cinnamon),
        "mate" => desktops::install_desktop_setup(DesktopSetup::Mate),
        "xfce refined" => desktops::install_desktop_setup(DesktopSetup::XfceRefined),
        "xfce picom" => desktops::install_desktop_setup(DesktopSetup::XfcePicom),
        "none/diy" => desktops::install_desktop_setup(DesktopSetup::None),
        _ => log::info!("No desktop setup selected!"),
    }
    println!();
    log::info!("Installing theme : {:?}", config.theme);

    match config.theme.to_lowercase().as_str() {
        "akame" => themes::install_theme_setup(ThemeSetup::Akame),
        "cyborg" => themes::install_theme_setup(ThemeSetup::Cyborg),
        "graphite" => themes::install_theme_setup(ThemeSetup::Graphite), //Note that the value on this match statement must fit the name in themes.py of aegis-gui (then they are lowercase transformed)
        "hackthebox" => themes::install_theme_setup(ThemeSetup::HackTheBox),
        "samurai" => themes::install_theme_setup(ThemeSetup::Samurai),
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
    // Misc Settings
    println!();
    log::info!("Enabling ipv6 : {}", config.networking.ipv6);
    if config.networking.ipv6 {
        network::enable_ipv6();
    }
    log::info!("Installing flatpak : {}", config.flatpak);
    if config.flatpak {
        base::install_flatpak();
    }
    println!();
    println!("---------");
    log::info!("Enabling zramd : {}", config.zramd);
    if config.zramd {
        base::install_zram();
    }
    // Users
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
    let exit_code = install::install(config.params.cores, config.params.jobs, config.params.keep);
    println!();
    log::info!("Installation log file copied to /var/log/aegis.log");
    files_eval(files::create_directory("/mnt/var/log"), "create /mnt/var/log");
    files::copy_file("/tmp/aegis.log", "/mnt/var/log/aegis.log");
    if config.bootloader.r#type == "grub-efi" {
        partition::umount("/mnt/boot");
    }
    partition::umount("/mnt/home");
    partition::umount("/mnt");
    if exit_code == 0 {
        log::info!("Installation finished! You may reboot now!");
    }
    else {
        log::error!("Installation failed. Exit code: {}", exit_code);
        if prompt_user_for_logs() {
            run_logs_command();
        }
    }
}

// Prompt the user to generate logs and return true if the answer is 'Y'
fn prompt_user_for_logs() -> bool {
    println!("\nDo you want to generate logs of the failed installation to share to Athena OS Team? (Y/N)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read user input.");

    // Trim input, convert to lowercase, and check if it equals 'y'
    input.trim().to_lowercase() == "y"
}

// Run the command to send logs to termbin.com
fn run_logs_command() {
    // Create a new command to run the specified shell command
    let mut logs_command = Command::new("sh")
        .args(&["-c", "cat /tmp/aegis.log | nc termbin.com 9999"])
        .stdout(Stdio::piped())  // Redirect standard output to a pipe
        .stderr(Stdio::piped())  // Redirect standard error to a pipe
        .spawn()  // Start the command as a new process
        .expect("Failed to start logs command.");  // Handle any errors during command startup

    let stdout_handle = logs_command.stdout.take().expect("Failed to open stdout pipe.");
    let stdout_thread = std::thread::spawn(move || {
        let reader = BufReader::new(stdout_handle);
        for line in reader.lines() {
            if let Ok(line) = line {            
                log::info!("{}", line);
            }
        }
    });

    let stderr_handle = logs_command.stderr.take().expect("Failed to open stderr pipe.");
    let stderr_thread = std::thread::spawn(move || {
        let reader = BufReader::new(stderr_handle);
        for line in reader.lines() {
            if let Ok(line) = line {
                log::error!("{}", line);
            }
        }
    });

    // Wait for the logs command to complete and log its exit status
    let logs_status = logs_command.wait();
    match logs_status {
        Ok(exit_status) => match exit_status.code() {
            Some(code) => {
                if code == 0 {
                    log::info!("Log URL generation completed.");
                } else {
                    log::error!("Error on generating log URL. Exit code: {}", code);
                }
            }
            None => log::info!("Logs command terminated without an exit code."),
        },
        Err(err) => log::error!("Failed to wait for logs command: {}", err),
    }

    // Wait for the threads capturing output to finish before returning
    stdout_thread.join().expect("Failed to join stdout thread.");
    stderr_thread.join().expect("Failed to join stderr thread.");
}