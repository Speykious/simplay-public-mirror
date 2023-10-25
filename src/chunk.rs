#![allow(dead_code)]

use std::sync::{Arc, Weak};
use bevy::prelude::*;
use hashbrown::HashMap;

use crate::block::*;
use crate::voxel::*;
use crate::world;
use crate::mesher;

// ==== DEBUG ====
const GREEDY_MESHING: bool = false;
// ===============

pub const CHUNK_SIZE: (u8, u8, u8) = (16, 16, 16);

#[derive(Debug)]
pub struct Chunk {
    blocks: HashMap<(i8, i8, i8), BlockType>, // The reason that I am using i8 instead of u8, is so I can read the blocks of neighboring chunks.
    cpos: (isize, isize, isize), // Chunk position.
    neighbors: HashMap<world::Direction, Weak<Self>>, // Neighbors.
}

impl Chunk {
    pub fn new(cpos: (isize, isize, isize)) -> Self {
        return Self {
            cpos,
            blocks: HashMap::new(),
            neighbors: HashMap::new(),
        };
    }

    // Set the neighbor.
    pub fn set_neighbor(&mut self, direction: world::Direction, chunk: Weak<Self>) {
        self.neighbors.insert(direction, chunk);
    }

    // Set the neighbor if it doesn't exist yet.
    pub fn add_neighbor(&mut self, direction: world::Direction, chunk: Weak<Self>) {
        if self.neighbors.contains_key(&direction) == false {
            self.set_neighbor(direction, chunk);
        }
    }

    pub fn get_neighbor(&self, direction: world::Direction) -> Option<Arc<Self>> {
        return match self.neighbors.get(&direction) {
            Some(s) => Some(s.upgrade().unwrap()),
            None => None,
        };
    }

    pub fn mesh(&self) -> Mesh {
        // Voxels store data like what sides need to be drawn.
        let mut voxels: Vec<Voxel> = Vec::new();

        // Loop through every block in the chunk.
        for x in 0..CHUNK_SIZE.0 {
            for y in 0..CHUNK_SIZE.1 {
                for z in 0..CHUNK_SIZE.2 {
                    let (ix, iy, iz) = (x as i8, y as i8, z as i8);

                    let block = self.get_block((ix, iy, iz));

                    let mut voxel_data = Voxel::new((x, y, z), block);

                    // Loop through all the neighboring blocks, and check if a face should be drawn.
                    for d in world::Direction::all() {
                        let (dx, dy, dz) = d.offset_with_position((x as isize, y as isize, z as isize)); // Returns isizes.
                        let (dx, dy, dz) = (dx as i8, dy as i8, dz as i8); // Convert to (i8, i8, i8).

                        let d_block = self.get_block((dx, dy, dz));

                        if Self::is_face(block, d_block) {
                            voxel_data.enable_side(d);
                        }
                    }

                    voxels.push(voxel_data);
                }
            }
        }

        let (mesh_data, indices) = mdi_from::voxel_array(&voxels);

        return mesher::create_mesh(&mesh_data, &indices);
    }

    // Used in self.mesh() to check whether a block needs a face or not.
    fn is_face(block: BlockType, d_block: BlockType) -> bool {
        if block.properties().transparent {
            if block == d_block {
                return false;
            }
        }

        if d_block.properties().transparent {
            return true;
        } else {
            return false;
        }
    }

    pub fn set_block_u8(&mut self, position: (u8, u8, u8), blocktype: BlockType) {
        self.set_block((position.0 as i8, position.1 as i8, position.2 as i8), blocktype);
    }

    pub fn set_block(&mut self, position: (i8, i8, i8), blocktype: BlockType) {
        match blocktype {
            BlockType::Air => self.blocks.remove(&position),
            _ => self.blocks.insert(position, blocktype),
        };
    }

    pub fn get_block_u8(&self, position: (u8, u8, u8)) -> BlockType {
        return self.get_block((position.0 as i8, position.1 as i8, position.2 as i8));
    }

    pub fn get_block(&self, position: (i8, i8, i8)) -> BlockType {
        if Self::position_overflow(position) == false {
            return match self.blocks.get(&position) {
                Some(s) => *s,
                None => BlockType::Air,
            };
        }

        else {
            let directions = Self::position_overflow_direction(position).unwrap();

            if directions.len() > 1 {
                return BlockType::Air;
            }

            let direction: world::Direction = directions[0];

            let neighbor = match self.get_neighbor(direction) {
                Some(s) => s,
                None => return BlockType::Air,
            };

            let wrap_pos = Self::wrap_position(position);

            let nblock = neighbor.get_block_u8(wrap_pos);

            return nblock;
        }
    }

    // Wrap an overflowing position to a regular position.
    pub fn wrap_position(position: (i8, i8, i8)) -> (u8, u8, u8) {
        let x: u8 = position.0.rem_euclid(CHUNK_SIZE.0 as i8) as u8;
        let y: u8 = position.1.rem_euclid(CHUNK_SIZE.1 as i8) as u8;
        let z: u8 = position.2.rem_euclid(CHUNK_SIZE.2 as i8) as u8;

        return (x, y, z);
    }

    // What direction does a position overflow in?
    pub fn position_overflow_direction(position: (i8, i8, i8)) -> Option<Vec<world::Direction>> {
        if Self::position_overflow(position) == false {
            return None;
        }

        let mut directions: Vec<world::Direction> = Vec::new();

        if position.0 < 0 {
            directions.push(world::Direction::West);
        }

        else if position.0 > (CHUNK_SIZE.0 - 1) as i8 {
            directions.push(world::Direction::East);
        }

        if position.1 < 0 {
            directions.push(world::Direction::Down);
        }

        else if position.1 > (CHUNK_SIZE.1 - 1) as i8 {
            directions.push(world::Direction::Up);
        }

        if position.2 < 0 {
            directions.push(world::Direction::North);
        }

        else if position.2 > (CHUNK_SIZE.2 - 1) as i8 {
            directions.push(world::Direction::South);
        }

        return Some(directions);
    }

    // Is a position outside of a chunk?
    pub fn position_overflow(position: (i8, i8, i8)) -> bool {
        let xyz_array = [position.0, position.1, position.2];
        let chunk_size_array = [CHUNK_SIZE.0 as i8, CHUNK_SIZE.1 as i8, CHUNK_SIZE.2 as i8];

        for i in 0..3 {
            if xyz_array[i] < 0 || xyz_array[i] > chunk_size_array[i] - 1 {
                return true;
            }
        }

        return false;
    }
}
