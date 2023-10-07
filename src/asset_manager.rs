#![allow(dead_code)]

use std::io;
use crate::places;
use crate::log;
use crate::log::macro_deps::*;
use crate::filesystem::*;
use crate::hash;

/// Read all the resource packs, then build the game's assets from the data.
pub fn build_assets() -> Result<(), io::Error> {
    log::info!("Building game assets...");

    // TODO: Block Atlas Generation

    // Record checksum.
    log::generic!("Saving built checksum...");

    match file::write(match asset_packs_checksum() {
        Ok(o) => o,
        Err(e) => return Err(e),
    }.as_str(), checksum_file()) {
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

fn asset_packs_checksum() -> Result<String, io::Error> {
    let packs = match directory::list_items(places::built_asset_packs()) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let mut packs_checksum_vec: Vec<String> = Vec::new();

    for i in packs.iter() {
        packs_checksum_vec.push(match hash::file(i.clone()) {
            Ok(o) => o,
            Err(e) => return Err(e),
        });
    }

    packs_checksum_vec.sort();

    let mut packs_checksum = String::new();

    for i in packs_checksum_vec.iter() {
        packs_checksum.push_str(i);
        packs_checksum.push_str(" ");
    }

    return Ok(packs_checksum.trim().to_string());
}
