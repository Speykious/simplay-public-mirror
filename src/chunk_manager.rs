#![allow(dead_code)]

use std::sync::{Arc, RwLock};
use std::rc::Rc;
use std::ops::Range;
use bevy::prelude::*;
use hashbrown::HashMap;

use crate::chunk::*;
use crate::block::*;
use crate::places;
use crate::world;

pub struct ChunkManagerPlugin;

impl Plugin for ChunkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::new());
        app.add_systems(Startup, test_chunks);
    }
}

#[derive(Resource)]
struct ChunkManager {
    chunks: HashMap<(isize, isize, isize), Arc<RwLock<Chunk>>>,
}

impl ChunkManager {
    fn new() -> Self {
        return Self {
            chunks: HashMap::new(),
        };
    }
}

fn test_chunks(
    mut cmds: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_manager: ResMut<ChunkManager>,
    asset_server: ResMut<AssetServer>,
) {
    let xyz_ranges: Rc<(Range<isize>, Range<isize>, Range<isize>)> = Rc::new((-1..2, -1..2, -1..2));

    for cx in xyz_ranges.0.clone() {
        for cy in xyz_ranges.1.clone() {
            for cz in xyz_ranges.2.clone() {
                let mut chunk = Chunk::new((cx, cy, cz));

                for x in 0..CHUNK_SIZE.0 {
                    for y in 0..CHUNK_SIZE.1 {
                        for z in 0..CHUNK_SIZE.2 {
                            chunk.set_block_u8((x, y, z), BlockType::Dirt);
                        }
                    }
                }

                chunk_manager.chunks.insert((cx, cy, cz), Arc::new(RwLock::new(chunk)));
            }
        }
    }

    for k in chunk_manager.chunks.keys() {
        let mut chunk = chunk_manager.chunks.get(k).unwrap().write().unwrap();

        for a in world::Direction::all() {
            let offset = a.offset_with_position(*k);

            if chunk_manager.chunks.contains_key(&offset) {
                let neighbor = chunk_manager.chunks.get(&offset).unwrap();

                chunk.add_neighbor(a, Arc::downgrade(neighbor));
            }
        }
    }

    for cx in xyz_ranges.0.clone() {
        for cy in xyz_ranges.1.clone() {
            for cz in xyz_ranges.2.clone() {
                let chunk = chunk_manager.chunks.get(&(cx, cy, cz)).unwrap().read().unwrap();

                cmds.spawn((
                    PbrBundle {
                        mesh: meshes.add(chunk.mesh()),
                        transform: Transform::from_xyz((cx * CHUNK_SIZE.0 as isize) as f32, (cy * CHUNK_SIZE.1 as isize) as f32, (cz * CHUNK_SIZE.2 as isize) as f32),
                        material: materials.add(StandardMaterial {
                            // base_color: Color::rgb(0.05, 0.5, 0.35), // The only reason this is still here, is because I think it is a cool color, and it is a secret comment!
                            base_color_texture: Some(asset_server.load(format!("{}/block_atlas.png", places::custom_built_assets().to_string()))),
                            // double_sided: true, // debug
                            // cull_mode: None, // debug
                            reflectance: 0.15,
                            perceptual_roughness: 0.6,
                            ..default()
                        }),
                        ..default()
                    }, Name::new(format!("Chunk ({}, {}, {})", cx, cy, cz))
                ));
            }
        }
    }
}
