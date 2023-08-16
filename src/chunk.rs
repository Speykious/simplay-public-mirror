#![allow(dead_code)]

use std::collections::HashMap;
use bevy::prelude::*;
use crate::voxel::*;
use crate::world::Axis;
use crate::library::*;

// ==== DEBUG SETTINGS ====
const CULL_FACES: bool = true;
const HIDE_EDGE_FACES: bool = true;
// ========================

pub const CHUNK_SIZE: (u8, u8, u8) = (16, 16, 16);

#[derive(Debug)]
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
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub position: Vec3,
    pub blocks: ChunkHashMap,
    pub mesh: Option<Mesh>,
}

impl Chunk {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        return Chunk {
            position: Vec3::new(x as f32, y as f32, z as f32),
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

    fn visible_sides_special_cases(&self, x: i8, y: i8, z: i8) -> Option<Vec<Axis>> {
        let mut sides: Vec<Axis> = Vec::new();

        if HIDE_EDGE_FACES == false {
            if x == 0 {
                sides.push(Axis::West);
            }

            if y == 0 {
                sides.push(Axis::Down);
            }

            if z == 0 {
                sides.push(Axis::North);
            }
        }

        if sides.len() == 0 {
            return None;
        } else {
            return Some(sides);
        }
    }

    pub fn visible_sides(&self, x: u8, y: u8, z: u8) -> Vec<Axis> {
        let axis_list = Axis::vec_all();

        if CULL_FACES == false {
            return axis_list;
        }

        let mut vsides: Vec<Axis> = Vec::new();

        for i in Axis::vec_all() {
            match self.visible_sides_special_cases(x as i8, y as i8, z as i8) {
                Some(s) => return s,
                None => {},
            };

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
}
