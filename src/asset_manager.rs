#![allow(dead_code)]

use std::io;
use std::sync::mpsc;
use clap::*;
use serde::{Serialize, Deserialize};
use colored::Colorize;
use hashbrown::HashMap;
use image::{DynamicImage, ImageBuffer, Rgba};
use rayon::prelude::*;
use crate::places;
use crate::log;
use crate::log::macro_deps::*;
use crate::filesystem::*;
use crate::hash;
use crate::cli;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Subcommand, ValueEnum)]
pub enum AssetCheckBuildBehavior {
    Yes,
    IfNeeded,
    No,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockAtlasEntries {
    entries: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssetLinks {
    links: Vec<(Path, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackOrder {
    order: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub struct AtlasUVMapElement {
    pub corner: (u32, u32), // Texture top-left corner.
    pub size: (u32, u32), // Texture size.
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct BlockAtlasInfo {
    pub size: (u32, u32), // Atlas size.
    pub uv_map: HashMap<String, AtlasUVMapElement>,
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

// Build the block texture atlas.
fn build_block_atlas_texture() -> Result<(), io::Error> {
    let texture_files: Vec<Path> = directory::list_items(&places::assets().add_str("textures/block"))?
        .into_iter()
        .filter(|x| x.path_type() == PathType::File && x.to_string().ends_with(".png"))
        .collect();

    let mut column_map: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut column_size_map: HashMap<u32, u32> = HashMap::new();

    for i in 0..texture_files.len() {
        let texture = open_image(&texture_files[i])?;

        let width: u32 = texture.width();
        let height: u32 = texture.height();

        if column_map.contains_key(&width) == false {
            column_map.insert(width, Vec::new());
            column_size_map.insert(width, 0);
        }

        let mut index_array: Vec<usize> = column_map.get(&width).unwrap().iter().map(|x| *x).collect(); // Guarenteed value.
        let mut column_size = *column_size_map.get(&width).unwrap(); // Guarenteed value.

        index_array.push(i);
        column_size += height;

        column_map.insert(width, index_array);
        column_size_map.insert(width, column_size);
    }

    let mut atlas_size: (u32, u32) = (0, 0);

    atlas_size.1 = *column_size_map.values().max().unwrap();

    for i in column_size_map.keys() {
        atlas_size.0 += i;
    }

    // <Texture Name, (Top-Left Corner Pixel Coordinates, Texture Size In Pixels)>
    let mut uv_map: HashMap<String, AtlasUVMapElement> = HashMap::new();

    let mut pixel_map: HashMap<(u32, u32), Rgba<u8>> = HashMap::new();

    let mut pixel_offset: (u32, u32) = (0, 0);

    for c in column_map.keys() {
        for i in column_map.get(c).unwrap().iter() {
            let texture: DynamicImage = open_image(&texture_files[*i])?;

            let texture_name: String = texture_files[*i].basename().replace(".png", "");

            let width: u32 = texture.width();
            let height: u32 = texture.height();

            uv_map.insert(texture_name, AtlasUVMapElement { corner: pixel_offset, size: (width, height) });

            for x in 0..width {
                for y in 0..height {
                    pixel_map.insert((x + pixel_offset.0, y + pixel_offset.1), *texture.clone().into_rgba8().get_pixel(x, y));
                }
            }

            pixel_offset.1 += height;
        }

        pixel_offset.1 = 0;
        pixel_offset.0 += c;
    }

    let atlas = ImageBuffer::from_fn(atlas_size.0, atlas_size.1, |x, y| { *pixel_map.get(&(x, y)).unwrap_or(&Rgba::from([0, 0, 0, 0])) });

    match atlas.save(places::custom_built_assets().add_str("block_atlas.png").to_string()) {
        Ok(_) => (),
        Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Failed to save block atlas!")),
    };

    file::write(match toml::to_string(&BlockAtlasInfo { size: atlas.dimensions(), uv_map }) {
        Ok(o) => o,
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to serialize atlas UV map!"));
        },
    }.as_str(), &places::custom_built_assets().add_str("block_atlas.toml"))?;

    return Ok(());
}

/// Get textures in a directory. Vec<(Image, LocalPathName)>
pub fn get_textures_in(path: &Path) -> Result<(Vec<DynamicImage>, Vec<String>), io::Error> {
    let files: Vec<Path> = directory::list_items(path)?
        .into_iter()
        .filter(|x| x.path_type() == PathType::File && x.to_string().ends_with(".png"))
        .collect();

    let local: Vec<String> = files.iter()
        .map(|x| x.to_string().replace(&format!("{}{sc}", path.to_string(), sc = Path::split_char()), ""))
        .collect();

    let mut images: Vec<DynamicImage> = Vec::new();

    for i in files.iter() {
        images.push(open_image(i)?);
    }

    return Ok((images, local));
}

// Open an image.
fn open_image(path: &Path) -> Result<DynamicImage, io::Error> {
    return match image::open(path.to_string()) {
        Ok(o) => Ok(o),
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to open image!"))
        },
    };
}

// Build all the user's asset packs into one singular asset pack.
fn build_unified_asset_links() -> Result<(), io::Error> {
    let order: PackOrder = match toml::from_str(file::read(&Path::new(&format!("{}/order.toml", places::asset_packs().to_string())))?.as_str()) {
        Ok(o) => o,
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to parse TOML for pack order!"))
        },
    };

    let mut map: HashMap<String, (Path, String)> = HashMap::new();

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

        let pack_info: PackInfo = match toml::from_str(file::read(&pack_info_path)?.as_str()) {
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

        let assets = directory::list_items_recursive(&Path::new(&format!("{}/assets", pack_path.to_string())))?;

        let assets: Vec<Path> = assets.into_iter()
            .filter(|x| x.path_type() == PathType::File)
            .collect();

        for a in assets {
            let key = a.to_string().replace(format!("{}{sc}assets{sc}", pack_path.to_string(), sc = Path::split_char()).as_str(), "");

            if map.contains_key(&key) == false {
                map.insert(key, (a, i.to_string()));
            }
        }
    }

    let mut link_array: Vec<(Path, String)> = Vec::new();

    for l in map.into_values() {
        link_array.push(l);
    }

    file::write(match toml::to_string(&AssetLinks { links: link_array }) {
        Ok(o) => o,
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to save map to links file."));
        },
    }.as_str(), &Path::new(&format!("{}/links.toml", places::unified_asset_links().to_string())))?;

    return Ok(());
}

// Construct all the unzipped asset packs.
fn construct_unzipped_asset_packs() -> Result<(), io::Error> {
    let asset_packs: Vec<Path> = directory::list_items(&places::asset_packs())?.into_iter()
        .filter(|x| x.path_type() == PathType::Directory || (x.path_type() == PathType::File && x.to_string().ends_with(".zip")))
        .collect();

    for i in asset_packs.iter() {
        if i.path_type() == PathType::File {
            let target_path = Path::new(&format!("{}/{}", places::unzipped_asset_packs_cache().to_string(), i.basename().replace(".zip", "")));
            
            archive::zip::extract(i, &target_path, false)?;
        }

        else if i.path_type() == PathType::Directory {
            fs_action::copy(i, &places::unzipped_asset_packs_cache())?;
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

    log::generic!("Copying linked assets to asset cache...");

    let links: AssetLinks = match toml::from_str(file::read(&Path::new(&format!("{}/links.toml", places::unified_asset_links().to_string())))?.as_str()) {
        Ok(o) => o,
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to parse asset links TOML file."));
        },
    };

    for (i, j) in links.links.iter() {
        let parent_path = i.parent_path();

        let creation_path = Path::new(&parent_path.to_string().replace(&format!("{}/{}/assets/", places::unzipped_asset_packs_cache().to_string(), j), ""));
        let creation_path = Path::new(&format!("{}/{}", places::assets().to_string(), creation_path.to_string()));

        directory::create(&creation_path)?;

        fs_action::mv(i, &creation_path)?;
    }

    // Build block atlas.
    log::generic!("Building block atlas...");

    build_block_atlas_texture()?;

    // Record checksum.
    log::generic!("Saving built checksum...");

    match file::write(asset_packs_checksum()?.as_str(), &checksum_file()) {
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

    match args.build_assets {
        Some(s) => {
            match s {
                AssetCheckBuildBehavior::Yes => return true,
                AssetCheckBuildBehavior::No => return false,
                AssetCheckBuildBehavior::IfNeeded => (),
            };
        },
        None => (),
    };

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
    return match file::read(&checksum_file()) {
        Ok(o) => Ok(o),
        Err(e) => Err(e),
    };
}

fn asset_packs_checksum_file() -> Path {
    return Path::new(format!("{}/asset_packs_checksum", places::assets().to_string()).as_str());
}

fn asset_packs_checksum() -> Result<String, io::Error> {
    if asset_packs_checksum_file().exists() {
        let sum = file::read(&asset_packs_checksum_file())?;

        return Ok(sum.trim().to_string());
    }

    log::info!("Getting all asset pack file checksums, this may take a while...");

    let files = match directory::list_items_recursive(&places::asset_packs()) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let files: Vec<Path> = files.into_iter()
        .filter(|x| x.path_type() == PathType::File)
        .collect();

    let (tx, rx) = mpsc::channel();

    files.par_iter().for_each_with(tx, |tx, i| tx.send(hash::sha256::file(&i).unwrap()).unwrap());

    let mut files_checksum_vec: Vec<String> = rx.iter().collect();
    files_checksum_vec.sort();

    let mut files_checksum = String::new();

    for i in files_checksum_vec.iter() {
        files_checksum.push_str(i);
        files_checksum.push_str(" ");
    }

    file::write(files_checksum.trim(), &asset_packs_checksum_file())?;

    return Ok(files_checksum.trim().to_string());
}

/// Remove the old asset pack cache checksum file. (Do this if the asset packs were changed.)
pub fn refresh_asset_packs_checksum() -> Result<(), io::Error> {
    if asset_packs_checksum_file().exists() {
        fs_action::delete(&asset_packs_checksum_file())?;
    }

    return Ok(());
}
