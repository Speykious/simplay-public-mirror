#![allow(dead_code)]

use bevy::prelude::*;
use hashbrown::HashMap;

use crate::block::*;
use crate::voxel::*;
use crate::world;
use crate::mesher;

// ==== DEBUG ====
const GREEDY_MESHING: bool = false;
const SHARE_VERTICES: bool = false;
// ===============

pub const CHUNK_SIZE: (u8, u8, u8) = (4, 4, 4);

pub struct Chunk {
    blocks: HashMap<(i8, i8, i8), BlockType>, // The reason that I am using i8 instead of u8, is so I can read the blocks of neighboring chunks.
    cpos: (isize, isize, isize), // Chunk position.
}

impl Chunk {
    pub fn new(cpos: (isize, isize, isize)) -> Self {
        return Self {
            cpos,
            blocks: HashMap::new(),
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

        let (mut mesh_data, mut indices) = mdi_from::voxel_array(&voxels);

        if SHARE_VERTICES {
            (mesh_data, indices) = mesher::optimize::share_vertices(&mesh_data, &indices);
        }

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

    pub fn set_block(&mut self, position: (i8, i8, i8), blocktype: BlockType) {
        // match blocktype {
        //     BlockType::Air => self.blocks.remove(&position),
        //     _ => self.blocks.insert(position, blocktype),
        // };

        self.blocks.insert(position, blocktype);
    }

    pub fn get_block(&self, position: (i8, i8, i8)) -> BlockType {
        return match self.blocks.get(&position) {
            Some(s) => *s,
            None => BlockType::Air,
        };
    }
}
