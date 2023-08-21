#![allow(dead_code)]

use std::collections::HashMap;
use bevy::prelude::*;
use crate::chunk::*;
use crate::voxel::*;
use crate::perlin;

pub fn chunk_manager_system(mut cmds: Commands, mut cm: ResMut<ChunkManager>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    for cx in 0..8 {
        for cy in 0..1 {
            for cz in 0..8 {
                if cm.chunks.chunk_exists(cx, cy, cz) {
                    break;
                }

                let mut chunk = cm.load_chunk(cx, cy, cz);

                for x in 0..CHUNK_SIZE.0 {
                    for y in 0..CHUNK_SIZE.1 {
                        for z in 0..CHUNK_SIZE.2 {
                            let (gx, gy, gz) = chunk.pos_local_to_global(x, y, z);
                            
                            let v = perlin::noise_3d(gx as f32, gy as f32, gz as f32);

                            if v > 10.0 {
                                chunk.set_block((x as u8, y as u8, z as u8), Block::Debug);
                            }
                        }
                    }
                }

                let chunk_mesh = chunk.draw();

                cmds.spawn(
                    PbrBundle {
                        mesh: meshes.add(chunk_mesh),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(1.0, 0.2, 1.0),
                            // double_sided: true,
                            // cull_mode: None,
                            ..default()
                        }),
                        transform: Transform::from_xyz(cx as f32 * CHUNK_SIZE.0 as f32, cy as f32 * CHUNK_SIZE.1 as f32, cz as f32 * CHUNK_SIZE.2 as f32),
                        ..default()
                    }
                );
            }
        }
    }
}

#[derive(Debug)]
pub struct ChunkTracker {
    map: HashMap<(isize, isize, isize), Chunk>,
}

impl ChunkTracker {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    pub fn insert(&mut self, c: (isize, isize, isize), b: Chunk) {
        self.map.insert(c, b);
    }

    pub fn get(&mut self, c: (isize, isize, isize)) -> Chunk {
        return match self.map.get(&c) {
            Some(s) => s.clone(),
            None => {
                let chunk = Chunk::new(c.0, c.1, c.2);

                self.insert(c, chunk);

                return self.get(c);
            },
        };
    }

    pub fn chunk_exists(&self, x: isize, y: isize, z: isize) -> bool {
        return match self.map.get(&(x, y, z)) {
            Some(_s) => true,
            None => false,
        };
    }
}

#[derive(Debug, Resource)]
pub struct ChunkManager {
    chunks: ChunkTracker,
}

impl ChunkManager {
    pub fn new() -> Self {
        return Self {
            chunks: ChunkTracker::new(),
        };
    }

    pub fn set_block(&mut self, pos: (isize, isize, isize), block: Block) {
        let local_pos = Chunk::pos_global_to_local(pos.0, pos.1, pos.2);
        let chunk_pos = Chunk::pos_global_to_chunk(pos.0, pos.1, pos.2);

        self.chunks.get(chunk_pos).set_block(local_pos, block);
    }

    pub fn load_chunk(&mut self, x: isize, y: isize, z: isize) -> Chunk {
        return self.chunks.get((x, y, z));
    }
}
