use crate::args::ShellSetup;
use crate::internal::*;

pub fn install_shell_setup(shell_setup: ShellSetup) {
    log::debug!("Installing {:?}", shell_setup);
    match shell_setup {
        ShellSetup::Bash => install_bash(),
        ShellSetup::Fish => install_fish(),
        ShellSetup::Zsh => install_zsh(),
        ShellSetup::None => log::debug!("No shell setup selected"),
    }
}

fn install_bash() {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "shell =.*",
            "shell = \"bash\"",
        ),
        "Set Bash",
    );
}

fn install_fish() {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "shell =.*",
            "shell = \"fish\"",
        ),
        "Set Fish",
    );
}

fn install_zsh() {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "shell =.*",
            "shell = \"zsh\"",
        ),
        "Set Zsh",
    );
}