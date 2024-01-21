use crate::internal::*;
use std::fs::{self, OpenOptions};
use std::io::{self, Write, Error, ErrorKind};
use regex::Regex;

pub fn copy_file(path: &str, destpath: &str) {
    let return_code = std::fs::copy(path, destpath);
    match return_code {
        Ok(_) => {
            log::info!("Copy {} to {}", path, destpath);
        }
        Err(e) => {
            crash(
                format!("Copy {} to {}: Failed with error {}", path, destpath, e),
                1,
            );
        }
    }
}

pub fn remove_file(path: &str) {
    let returncode = std::fs::remove_file(path);
    match returncode {
        Ok(_) => {
            log::info!("Remove {}", path);
        }
        Err(e) => {
            crash(format!("Remove {}: Failed with error {}", path, e), 1);
        }
    }
}

pub fn sed_file(path: &str, find: &str, replace: &str) -> io::Result<()> {
    log::info!("Sed '{}' to '{}' in file {}", find, replace, path);
    let contents = fs::read_to_string(path)?;
    let regex = Regex::new(find).map_err(|e| Error::new(ErrorKind::InvalidInput, e.to_string()))?;
    let new_contents = regex.replace_all(&contents, replace);
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
    file.write_all(new_contents.as_bytes())?;
    Ok(())
}

pub fn create_directory(path: &str) -> std::io::Result<()> { // Create all missing dirs in the specified path
    std::fs::create_dir_all(path)
}
