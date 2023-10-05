#![allow(dead_code)]

use std::io;
use bevy::prelude::*;
use crate::places;
use crate::log;
use crate::log::macro_deps::*;

/// Read all the resource packs, then build the game's assets from the data.
pub fn build_assets() -> Result<(), io::Error> {
    log::info!("Building game assets...");

    return Ok(()); // todo
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
    return true; // todo
}
