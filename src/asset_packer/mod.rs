pub mod packer;

use std::env;
use bevy::prelude::*;
use crate::places;

pub struct AssetPackerPlugin;

impl Plugin for AssetPackerPlugin {
    fn build(&self, app: &mut App) {
        env::set_var("BEVY_ASSET_ROOT", places::assets().to_string());
    }
}
