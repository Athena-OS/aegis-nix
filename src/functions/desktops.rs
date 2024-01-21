use crate::args::DesktopSetup;
use crate::internal::{files, files_eval};

pub fn install_desktop_setup(desktop_setup: DesktopSetup) {
    log::debug!("Installing {:?}", desktop_setup);
    match desktop_setup {
        DesktopSetup::Gnome => install_gnome(),
        DesktopSetup::XfceRefined => install_xfce_refined(),
        DesktopSetup::XfcePicom => install_xfce_picom(),
        DesktopSetup::None => log::debug!("No desktop setup selected"),
    }
}

fn install_xfce_refined() {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "desktop =.*",
            "desktop = \"xfce\"",
        ),
        "Set XFCE",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/home-manager/desktops/xfce/default.nix",
            "athena.desktops.xfce.refined =.*",
            "athena.desktops.xfce.refined = true;",
        ),
        "Set XFCE Refined",
    );
}

fn install_xfce_picom() {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "desktop =.*",
            "desktop = \"xfce\"",
        ),
        "Set XFCE",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/home-manager/desktops/xfce/default.nix",
            "athena.desktops.xfce.refined =.*",
            "athena.desktops.xfce.refined = false;",
        ),
        "Set XFCE Picom",
    );
}

fn install_gnome() {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "desktop =.*",
            "desktop = \"gnome\"",
        ),
        "Set GNOME",
    );
}