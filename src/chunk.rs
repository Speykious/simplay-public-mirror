#![allow(dead_code)]

use std::collections::HashMap;
use bevy::prelude::*;
use crate::voxel::*;
use crate::world::{Axis, AxisCoord};
use crate::library::*;

// ==== DEBUG SETTINGS ====
const CULL_FACES: bool = true;
const HIDE_EDGE_FACES: bool = true;
const GREEDY_MESHING: bool = true;
// ========================

pub const CHUNK_SIZE: (u8, u8, u8) = (16, 16, 16);

#[derive(Debug, Clone)]
pub struct ChunkHashMap {
    map: HashMap<(u8, u8, u8), Block>,
}

impl ChunkHashMap {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    pub fn insert(&mut self, c: (u8, u8, u8), b: Block) {
        self.map.insert(c, b);
    }

    pub fn get(&self, c: (u8, u8, u8)) -> Block {
        return match self.map.get(&c) {
            Some(s) => *s,
            None => Block::Air,
        };
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub position: (isize, isize, isize),
    pub blocks: ChunkHashMap,
    pub mesh: Option<Mesh>,
}

impl Chunk {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        return Chunk {
            position: (x, y, z),
            blocks: Self::init_blocks(),
            mesh: None,
        };
    }

    pub fn draw(&self) -> Mesh {
        let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for i in self.blocks.map.iter() {
            let (x, y, z) = (i.0.0, i.0.1, i.0.2);

            let block = self.blocks.get((x, y, z));
            let sides = VoxelSide::vec_from_axis_vec(&self.visible_sides(x, y, z), (x, y, z));

            for j in sides.iter() {
                vertices.extend(j.vertices.clone());

                indices = combine_indices(&vec![indices, j.indices.clone()]);
            }
        }

        return create_mesh(&vertices, &indices);
    }

    // fn visible_sides_special_cases(&self, x: i8, y: i8, z: i8) -> Option<Vec<Axis>> {
    //     let mut sides: Vec<Axis> = Vec::new();

    //     if HIDE_EDGE_FACES == false {
    //         if x == 0 {
    //             sides.push(Axis::West);
    //         }

    //         if y == 0 {
    //             sides.push(Axis::Down);
    //         }

    //         if z == 0 {
    //             sides.push(Axis::North);
    //         }
    //     }

    //     if sides.len() == 0 {
    //         return None;
    //     } else {
    //         return Some(sides);
    //     }
    // }

    pub fn visible_sides(&self, x: u8, y: u8, z: u8) -> Vec<Axis> {
        let axis_list = Axis::vec_all();

        if CULL_FACES == false {
            return axis_list;
        }

        let mut vsides: Vec<Axis> = Vec::new();

        for i in Axis::vec_all() {
            // match self.visible_sides_special_cases(x as i8, y as i8, z as i8) {
            //     Some(s) => return s,
            //     None => {},
            // };

            let (ox, oy, oz) = i.coord_offset_from(x.into(), y.into(), z.into());

            let p = self.get_block(ox as u8, oy as u8, oz as u8).properties();

            if p.transparent == true {
                vsides.push(i);
            }
        }

        return vsides;
    }

    pub fn set_block(&mut self, pos: (u8, u8, u8), type_id: Block) {
        self.blocks.insert(pos, type_id);
    }

    pub fn get_block(&self, x: u8, y: u8, z: u8) -> Block {
        return self.blocks.get((x, y, z));
    }

    pub fn size() -> usize {
        return CHUNK_SIZE.0 as usize * CHUNK_SIZE.1 as usize * CHUNK_SIZE.2 as usize;
    }

    pub fn linearize(x: u8, y: u8, z: u8) -> usize {
        return z as usize * CHUNK_SIZE.0 as usize * CHUNK_SIZE.1 as usize + y as usize * CHUNK_SIZE.0 as usize + x as usize;
    }

    pub fn delinearize(index: usize) -> (u8, u8, u8) {
        let z = index / (CHUNK_SIZE.0 as usize * CHUNK_SIZE.1 as usize);
        let index = index % (CHUNK_SIZE.0 as usize * CHUNK_SIZE.1 as usize);
        let y = index / CHUNK_SIZE.0 as usize;
        let x = index % CHUNK_SIZE.0 as usize;

        return (x as u8, y as u8, z as u8);
    }

    fn init_blocks() -> ChunkHashMap {
        let mut blocks = ChunkHashMap::new();

        // for x in 0..CHUNK_SIZE.0 {
        //     for y in 0..CHUNK_SIZE.1 {
        //         for z in 0..CHUNK_SIZE.2 {
        //             blocks.insert((x, y, z), Block::Air);
        //         }
        //     }
        // }

        return blocks;
    }

    pub fn pos_local_to_global_single(&self, s: u8, coord_type: AxisCoord) -> isize {
        let csize = match coord_type {
            AxisCoord::X => CHUNK_SIZE.0,
            AxisCoord::Y => CHUNK_SIZE.1,
            AxisCoord::Z => CHUNK_SIZE.2,
        };

        let cpos = match coord_type {
            AxisCoord::X => self.position.0,
            AxisCoord::Y => self.position.1,
            AxisCoord::Z => self.position.2,
        };

        let global_s: isize = (cpos * csize as isize) + s as isize;

        return global_s;
    }

    pub fn pos_local_to_global(&mut self, x: u8, y: u8, z: u8) -> (isize, isize, isize) {
        let global_pos: (isize, isize, isize) = (self.pos_local_to_global_single(x, AxisCoord::X), self.pos_local_to_global_single(y, AxisCoord::Y), self.pos_local_to_global_single(z, AxisCoord::Z));

        return global_pos;
    }

    pub fn pos_global_to_chunk_single(s: isize, coord_type: AxisCoord) -> isize {
        let csize = match coord_type {
            AxisCoord::X => CHUNK_SIZE.0,
            AxisCoord::Y => CHUNK_SIZE.1,
            AxisCoord::Z => CHUNK_SIZE.2,
        };

        let chunk_s: isize = (s as f64 / csize as f64).ceil() as isize;

        return chunk_s;
    }

    pub fn pos_global_to_chunk(x: isize, y: isize, z: isize) -> (isize, isize, isize) {
        let chunk_pos: (isize, isize, isize) = (Self::pos_global_to_chunk_single(x, AxisCoord::X), Self::pos_global_to_chunk_single(y, AxisCoord::Y), Self::pos_global_to_chunk_single(z, AxisCoord::Z));

        return chunk_pos;
    }

    pub fn pos_chunk_to_global_single(s: isize, coord_type: AxisCoord) -> isize {
        let csize = match coord_type {
            AxisCoord::X => CHUNK_SIZE.0,
            AxisCoord::Y => CHUNK_SIZE.1,
            AxisCoord::Z => CHUNK_SIZE.2,
        };

        let global_s: isize = s * csize as isize;

        return global_s;
    }

    pub fn pos_chunk_to_global(x: isize, y: isize, z: isize) -> (isize, isize, isize) {
        let global_pos: (isize, isize, isize) = (Self::pos_chunk_to_global_single(x, AxisCoord::X), Self::pos_chunk_to_global_single(y, AxisCoord::Y), Self::pos_chunk_to_global_single(z, AxisCoord::Z));

        return global_pos;
    }

    pub fn pos_global_to_local_single(s: isize, coord_type: AxisCoord) -> u8 {
        let csize = match coord_type {
            AxisCoord::X => CHUNK_SIZE.0,
            AxisCoord::Y => CHUNK_SIZE.1,
            AxisCoord::Z => CHUNK_SIZE.2,
        };

        let local_s: u8 = (s % csize as isize) as u8;

        return local_s;
    }

    pub fn pos_global_to_local(x: isize, y: isize, z: isize) -> (u8, u8, u8) {
        let local_pos: (u8, u8, u8) = (Self::pos_global_to_local_single(x, AxisCoord::X), Self::pos_global_to_local_single(y, AxisCoord::Y), Self::pos_global_to_local_single(z, AxisCoord::Z));

        return local_pos;
    }
}
