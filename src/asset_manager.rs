#![allow(dead_code)]

use std::io;
use clap::Parser;
use serde::{Serialize, Deserialize};
use colored::Colorize;
use hashbrown::HashMap;
use crate::places;
use crate::log;
use crate::log::macro_deps::*;
use crate::filesystem::*;
use crate::hash;
use crate::cli;

#[derive(Debug, Serialize, Deserialize)]
struct AssetLinks {
    links: HashMap<String, Path>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackOrder {
    order: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
struct PackInfo {
    display_name: String,
    description: String,
    authors: Vec<String>,
    licenses: Vec<String>,
    pack_format: i32,
}

impl PackInfo {
    fn default() -> Self {
        return Self {
            display_name: String::from("< Unnamed Asset Pack >"),
            description: String::from("< No Description >"),
            authors: Vec::new(),
            licenses: Vec::new(),
            pack_format: -1, // -1 = undefined
        };
    }
}

// Build all the user's asset packs into one singular asset pack.
fn build_unified_asset_links() -> Result<(), io::Error> {
    let order: PackOrder = match toml::from_str(file::read(Path::new(&format!("{}/order.toml", places::asset_packs().to_string())))?.as_str()) {
        Ok(o) => o,
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to parse TOML for pack order!"))
        },
    };

    let mut map: HashMap<String, Path> = HashMap::new();

    for i in order.order.iter().rev() {
        let pack_path = Path::new(&format!("{}/{}", places::unzipped_asset_packs_cache().to_string(), i));
        let pack_info_path = Path::new(&format!("{}/pack.toml", pack_path.to_string()));

        if pack_path.exists() == false {
            log::error!("Pack non-existant: {} (Skipping...)", i);

            continue;
        }

        if pack_info_path.exists() == false {
            log::error!("No pack.toml in: {} (Skipping...)", i);

            continue;
        }

        let pack_info: PackInfo = match toml::from_str(file::read(pack_info_path)?.as_str()) {
            Ok(o) => o,
            Err(e) => {
                log::generic!("Failed to parse PackInfo from pack.toml: {:?}", e);

                return Err(io::Error::new(io::ErrorKind::Other, "Failed to parse pack info TOML!"))
            },
        };

        log::generic!("Processing pack: {} : {}", pack_info.display_name.bright_cyan().bold(), pack_info.description.bright_green().bold());

        if Path::new(&format!("{}/assets", pack_path.to_string())).exists() == false {
            log::error!("Missing 'assets' directory: {} (Skipping...)", pack_info.display_name);

            continue;
        }

        let assets = directory::list_items_recursive(Path::new(&format!("{}/assets", pack_path.to_string())))?;

        let assets: Vec<Path> = assets.into_iter()
            .filter(|x| x.path_type() == PathType::File)
            .collect();

        for a in assets {
            if map.contains_key(&a.basename()) == false {
                map.insert(a.basename(), a);
            }
        }
    }

    file::write(match toml::to_string(&map) {
        Ok(o) => o,
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to save map to links file."));
        },
    }.as_str(), Path::new(&format!("{}/links.toml", places::unified_asset_links().to_string())))?;

    return Ok(());
}

// Construct all the unzipped asset packs.
fn construct_unzipped_asset_packs() -> Result<(), io::Error> {
    let asset_packs: Vec<Path> = directory::list_items(places::asset_packs())?.into_iter()
        .filter(|x| x.path_type() == PathType::Directory || (x.path_type() == PathType::File && x.to_string().ends_with(".zip")))
        .collect();

    for i in asset_packs.iter() {
        log::debug!("{}", i.to_string());

        if i.path_type() == PathType::File {
            let target_path = Path::new(&format!("{}/{}", places::unzipped_asset_packs_cache().to_string(), i.basename().replace(".zip", "")));
            
            archive::zip::extract(i.clone(), target_path, false)?;
        }

        else if i.path_type() == PathType::Directory {
            fs_action::copy(i.clone(), places::unzipped_asset_packs_cache())?;
        }
    }

    return Ok(());
}

/// Read all the built asset packs, then build the game's assets from the data.
pub fn build_assets() -> Result<(), io::Error> {
    log::info!("Unzipping all asset packs...");

    construct_unzipped_asset_packs()?;

    log::info!("Building unified asset links...");

    build_unified_asset_links()?;

    log::info!("Building game assets...");

    // TODO: Block Atlas Generation

    // Record checksum.
    log::generic!("Saving built checksum...");

    match file::write(asset_packs_checksum()?.as_str(), checksum_file()) {
        Ok(_) => (),
        Err(e) => {
            log::error!("Failed to write to cache checksum!");
            
            return Err(e);
        },
    };

    // Yay! The code made it!
    return Ok(());
}

/// If the assets need to be built again, build them.
pub fn build_assets_if_needed() -> Result<(), io::Error> {
    if is_build_needed() {
        return build_assets();
    }

    log::info!("No need to build game assets again... skipping...");

    return Ok(());
}

/// Checks to see whether the builded assets are outdated, and need to be built again.
pub fn is_build_needed() -> bool {
    let args = cli::Cli::parse();

    if args.rebuild_assets {
        return true;
    }

    if checksum_file().exists() == false {
        return true;
    }

    if asset_packs_checksum().unwrap() == cache_checksum().unwrap() {
        return false;
    }

    else {
        return true;
    }
}

fn checksum_file() -> Path {
    return Path::new(format!("{}/checksum", places::assets().to_string()).as_str());
}

fn cache_checksum() -> Result<String, io::Error> {
    return match file::read(checksum_file()) {
        Ok(o) => Ok(o),
        Err(e) => Err(e),
    };
}

fn asset_packs_checksum_file() -> Path {
    return Path::new(format!("{}/asset_packs_checksum", places::assets().to_string()).as_str());
}

fn asset_packs_checksum() -> Result<String, io::Error> {
    if asset_packs_checksum_file().exists() {
        let sum = file::read(asset_packs_checksum_file())?;

        return Ok(sum.trim().to_string());
    }

    log::info!("Getting all asset pack file checksums, this may take a while...");

    let files = match directory::list_items_recursive(places::asset_packs()) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let files: Vec<Path> = files.into_iter()
        .filter(|x| x.path_type() == PathType::File)
        .collect();

    let mut files_checksum_vec: Vec<String> = Vec::new();

    for i in files.iter() {
        files_checksum_vec.push(match hash::file(i.clone()) {
            Ok(o) => o,
            Err(e) => return Err(e),
        });
    }

    files_checksum_vec.sort();

    let mut files_checksum = String::new();

    for i in files_checksum_vec.iter() {
        files_checksum.push_str(i);
        files_checksum.push_str(" ");
    }

    file::write(files_checksum.trim(), asset_packs_checksum_file())?;

    return Ok(files_checksum.trim().to_string());
}

/// Remove the old asset pack cache checksum file. (Do this if the asset packs were changed.)
pub fn refresh_asset_packs_checksum() -> Result<(), io::Error> {
    if asset_packs_checksum_file().exists() {
        fs_action::delete(asset_packs_checksum_file())?;
    }

    return Ok(());
}
