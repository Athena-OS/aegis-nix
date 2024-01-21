mod args;
mod functions;
mod internal;
mod logging;

use crate::args::{BootloaderSubcommand, Command, Cli, UsersSubcommand};
use crate::functions::*;
use clap::Parser;

fn main() {
    human_panic::setup_panic!();
    let cli = Cli::parse();
    println!("verbose: {}", cli.verbose);
    let log_file_path = "/tmp/aegis";
    logging::init(cli.verbose, log_file_path);
    match cli.command {
        Command::Partition(args) => {
            let mut partitions = args.partitions;
            partition::partition(
                args.device,
                args.mode,
                args.efi,
                &mut partitions,
            );
        }
        Command::InstallBase => {
            base::install_nix_config();
        }
        Command::Bootloader { subcommand } => match subcommand {
            BootloaderSubcommand::GrubEfi { } => {
                base::install_bootloader_efi();
            }
            BootloaderSubcommand::GrubLegacy { device } => {
                base::install_bootloader_legacy(device);
            }
        }
        Command::Locale(args) => {
            locale::set_locale(args.locales.join(" ")); // locale.gen file comes grom glibc package that is in base group package
            locale::set_keyboard(&args.keyboard);
            locale::set_timezone(&args.timezone);
        }
        Command::Networking(args) => {
            network::set_hostname(&args.hostname);
        }
        Command::Users { subcommand } => match subcommand {
            UsersSubcommand::NewUser(args) => {
                users::new_user(
                    &args.username,
                    &args.password,
                    true,
                );
            }
            UsersSubcommand::RootPass { password } => {
                users::root_pass(&password);
            }
        },
        Command::Config { config } => {
            crate::internal::config::read_config(config);
        }
        Command::Desktops { desktop } => {
            desktops::install_desktop_setup(desktop);
        }
        Command::Themes { theme } => {
            themes::install_theme_setup(theme);
        }
        Command::DisplayManagers { displaymanager } => {
            displaymanagers::install_dm_setup(displaymanager);
        }
        Command::Shells { shell } => {
            shells::install_shell_setup(shell);
        }
        Command::Browsers { browser } => {
            browsers::install_browser_setup(browser);
        }
        Command::Terminals { terminal } => {
            terminals::install_terminal_setup(terminal);
        }
    }
}
