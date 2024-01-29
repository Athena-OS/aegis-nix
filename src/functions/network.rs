use crate::internal::*;

pub fn set_hostname(hostname: &str) {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "hostname =.*",
            &(format!("hostname = \"{}\";", hostname)),
        ),
        "Set Hostname",
    );
}


pub fn enable_ipv6() {
    files_eval(
        files::sed_file(
            "/mnt/etc/nixos/configuration.nix",
            "networking.enableIPv6 =.*",
            "networking.enableIPv6 = true;",
        ),
        "enable ipv6",
    );
}