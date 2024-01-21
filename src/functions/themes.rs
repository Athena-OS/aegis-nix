use crate::args::ThemeSetup;
use crate::internal::*;

pub fn install_theme_setup(theme_setup: ThemeSetup) {
    log::debug!("Installing {:?}", theme_setup);
    match theme_setup {
        ThemeSetup::Graphite => install_graphite(),
        ThemeSetup::Sweet => install_sweet(),
        ThemeSetup::None => log::debug!("No theme setup selected"),
    }
}

fn install_graphite() {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "module-name =.*",
            "module-name = \"graphite\"",
        ),
        "Set Graphite Module",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "gtk-theme =.*",
            "gtk-theme = \"Graphite-Dark\"",
        ),
        "Set Graphite GTK",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "icon-theme =.*",
            "icon-theme = \"Tela-circle-black-dark\"",
        ),
        "Set Graphite Icon Theme",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "cursor-theme =.*",
            "cursor-theme = \"Bibata-Modern-Ice\"",
        ),
        "Set Graphite Cursor Theme",
    );
}

fn install_sweet() {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "module-name =.*",
            "module-name = \"sweet\"",
        ),
        "Set Sweet Module",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "gtk-theme =.*",
            "gtk-theme = \"Sweet-Dark-v40\"",
        ),
        "Set Sweet GTK",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "icon-theme =.*",
            "icon-theme = \"Tela-circle-black-dark\"",
        ),
        "Set Sweet Icon Theme",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "cursor-theme =.*",
            "cursor-theme = \"Bibata-Modern-Ice\"",
        ),
        "Set Sweet Cursor Theme",
    );
}