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

/// Base directory for configurations and data.
pub fn base() -> Path {
    return_path!("simplay", dir::config().to_string());
}

/// Cache directory for the game.
pub fn cache() -> Path {
    return_path!("simplay", dir::cache().to_string());
}

/// Built assets.
pub fn assets() -> Path {
    return_path!("assets", cache().to_string());
}

/// User installed asset packs.
pub fn asset_packs() -> Path {
    return_path!("asset_packs", base().to_string());
}

/// Links to assets used for building built assets.
pub fn unified_asset_links() -> Path {
    return_path!("unified_asset_links", cache().to_string());
}

/// All the user installed asset packs in directory form.
pub fn unzipped_asset_packs_cache() -> Path {
    return_path!("unzipped_assets", cache().to_string());
}

// Create all the directories.
pub fn create_all_dirs() -> Result<(), io::Error> {
    let directories = vec![
        base(),
        asset_packs(),

        cache(),
        assets(),
        unified_asset_links(),
        unzipped_asset_packs_cache(),
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

// Delete all temporary the directories.
pub fn delete_temp_dirs() -> Result<(), io::Error> {
    let directories = vec![
        unified_asset_links(),
        unzipped_asset_packs_cache(),
    ];

    info!("Deleting temporary directories...");

    for i in directories.iter() {
        if i.exists() == true {
            match fs_action::delete(i.clone()) {
                Ok(_) => {
                    generic!("Deleted directory: {}", i.to_string());
                },
                Err(e) => {
                    error!("Failed to delete directory: {}", i.to_string());
                    return Err(e);
                },
            };
        }
    }

    return Ok(());
}
