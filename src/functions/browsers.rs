use crate::args::BrowserSetup;
use crate::internal::*;

pub fn install_browser_setup(browser_setup: BrowserSetup) {
    log::debug!("Installing {:?}", browser_setup);
    match browser_setup {
        BrowserSetup::Firefox => install_firefox(),
        BrowserSetup::None => log::debug!("No browser setup selected"),
    }
}

fn install_firefox() {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "browser =.*",
            "browser = \"firefox\"",
        ),
        "Set Firefox",
    );
}