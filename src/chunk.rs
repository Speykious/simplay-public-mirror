#![allow(dead_code)]

use bevy::prelude::*;
use hashbrown::HashMap;

use crate::block::*;
use crate::voxel::*;
use crate::world;

// ==== DEBUG ====
const CULL_FACES: bool = false;
const GREEDY_MESHING: bool = false;
// ===============

pub const CHUNK_SIZE: (u8, u8, u8) = (16, 16, 16);

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
        // let mut mesh_info: Vec<([f32; 3], [f32; 3], [f32; 2])> = Vec::new();
        // let mut indices: Vec<u32> = Vec::new();

        let mut voxels: Vec<Voxel> = Vec::new();

        for x in 0..CHUNK_SIZE.0 {
            for y in 0..CHUNK_SIZE.0 {
                for z in 0..CHUNK_SIZE.0 {
                    let (ux, uy, uz) = (x, y, z);
                    let (x, y, z) = (x as i8, y as i8, z as i8); // Basically converts (x, y, z) from u8, to i8.

                    let mut voxel = Voxel::new((ux, uy, uz), self.get_block((x, y, z)));

                    for d in world::Direction::all().iter() {
                        let (dx, dy, dz) = d.offset_with_position((x as isize, y as isize, z as isize));
                        let (dx, dy, dz) = (dx as i8, dy as i8, dz as i8);

                        let d_block = self.get_block((dx, dy, dz));
                        let d_block_properties = d_block.properties();

                        if d_block_properties.transparent {
                            voxel.enable_side(*d);
                        }
                    }

                    voxels.push(voxel);
                }
            }
        }

        return mesh_from_voxels(&voxels);
    }

    pub fn set_block(&mut self, position: (i8, i8, i8), blocktype: BlockType) {
        match blocktype {
            BlockType::Air => self.blocks.remove(&position),
            _ => self.blocks.insert(position, blocktype),
        };
    }

    pub fn get_block(&mut self, position: (i8, i8, i8)) -> BlockType {
        return match self.blocks.get(&position) {
            Some(s) => *s,
            None => BlockType::Air,
        };
    }
}
