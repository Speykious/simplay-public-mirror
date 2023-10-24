#![allow(dead_code)]

use bevy::prelude::*;

use crate::chunk::*;
use crate::block::*;
use crate::places;

pub struct ChunkManagerPlugin;

impl Plugin for ChunkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, test_chunks);
    }
}

fn test_chunks(
    mut cmds: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: ResMut<AssetServer>,
) {
    for cx in 0..2 {
        for cy in 0..2 {
            for cz in 0..2 {
                let mut chunk = Chunk::new((cx, cy, cz));

                for x in 0..CHUNK_SIZE.0 {
                    for y in 0..CHUNK_SIZE.1 {
                        for z in 0..CHUNK_SIZE.2 {
                            chunk.set_block_u8((x, y, z), BlockType::Dirt);
                        }
                    }
                }

                cmds.spawn((
                    PbrBundle {
                        mesh: meshes.add(chunk.mesh()),
                        transform: Transform::from_xyz((cx * CHUNK_SIZE.0 as isize) as f32, (cy * CHUNK_SIZE.1 as isize) as f32, (cz * CHUNK_SIZE.2 as isize) as f32),
                        material: materials.add(StandardMaterial {
                            // base_color: Color::rgb(0.05, 0.5, 0.35),
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
