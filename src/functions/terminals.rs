use crate::args::TerminalSetup;
use crate::internal::*;

pub fn install_terminal_setup(terminal_setup: TerminalSetup) {
    log::debug!("Installing {:?}", terminal_setup);
    match terminal_setup {
        TerminalSetup::Alacritty => install_alacritty(),
        TerminalSetup::Kitty => install_kitty(),
        TerminalSetup::None => log::debug!("No terminal setup selected"),
    }
}

fn install_alacritty() {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "terminal =.*",
            "terminal = \"alacritty\"",
        ),
        "Set Alacritty",
    );
}

fn install_kitty() {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "terminal =.*",
            "terminal = \"kitty\"",
        ),
        "Set Kitty",
    );
}