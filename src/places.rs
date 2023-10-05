#![allow(dead_code)]

use std::io;
use crate::dir;
use crate::filesystem::*;
use crate::log::*;

macro_rules! return_path {
    (
        $extend_path: expr,
        $base_path: expr
    ) => {
        return Path::new(&format!("{}/{}", $base_path, $extend_path));
    }
}

pub fn base() -> Path {
    return_path!("simplay", dir::config().to_string());
}

pub fn cache() -> Path {
    return_path!(".simplay_cache", dir::cache().to_string());
}

pub fn assets() -> Path {
    return_path!("assets", cache().to_string());
}

pub fn create_all_dirs() -> Result<(), io::Error> {
    let directories = vec![
        base(),
        cache(),
        assets(),
    ];

    info!("Setting up directories...");

    for i in directories.iter() {
        if i.exists() == false {
            match directory::create(i.clone()) {
                Ok(_) => {
                    generic!("Created directory: {}", i.to_string());
                },
                Err(e) => {
                    error!("Failed to create directory: {}", i.to_string());
                    return Err(e);
                },
            };
        }
    }

    return Ok(());
}
