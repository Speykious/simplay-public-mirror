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
    enabled_meshes: HashMap<((u8, u8, u8), Axis), bool>,
    mesh_sizes: HashMap<((u8, u8, u8), Axis), (u8, u8, u8)>,
}

impl ChunkHashMap {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
            enabled_meshes: HashMap::new(),
            mesh_sizes: HashMap::new(),
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

    pub fn enable_disable_mesh(&mut self, pos: (u8, u8, u8), side: Axis, enable: bool) {
        self.enabled_meshes.insert((pos, side), enable);
    }

    pub fn disable_mesh(&mut self, pos: (u8, u8, u8), side: Axis) {
        self.enable_disable_mesh(pos, side, false);
    }

    pub fn enable_mesh(&mut self, pos: (u8, u8, u8), side: Axis) {
        self.enable_disable_mesh(pos, side, true);
    }

    pub fn mesh_enabled(&self, pos: (u8, u8, u8), side: Axis) -> bool {
        return match self.enabled_meshes.get(&(pos, side)) {
            Some(s) => *s,
            None => true,
        };
    }

    pub fn set_face_size(&mut self, pos: (u8, u8, u8), side: Axis, size: (u8, u8, u8)) {
        self.mesh_sizes.insert((pos, side), size);
    }

    pub fn get_face_size(&self, pos: (u8, u8, u8), side: Axis) -> (u8, u8, u8) {
        return match self.mesh_sizes.get(&(pos, side)) {
            Some(s) => *s,
            None => (1, 1, 1),
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

    pub fn draw(&mut self) -> Mesh {
        let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        self.greedy_mesher();

        for i in self.clone().blocks.map.iter() {
            let (x, y, z) = (i.0.0, i.0.1, i.0.2);

            let vinfo = self.visible_info(x, y, z);

            let mut vsizes: Vec<(u8, u8, u8)> = Vec::new();

            for i in Axis::vec_all() {
                vsizes.push(self.blocks.get_face_size((x, y, z), i));
            }

            let block = self.blocks.get((x, y, z));
            let sides = VoxelSide::vec_from_axis_vec(&vinfo.sides, &vsizes, (x, y, z));

            for j in sides.iter() {
                if self.blocks.mesh_enabled((x, y, z), j.side) {
                    vertices.extend(j.vertices.clone());
                    indices = combine_indices(&vec![indices, j.indices.clone()]);
                }
            }
        }

        return create_mesh(&vertices, &indices);
    }

    fn greedy_mesher(&mut self) {
        if GREEDY_MESHING == false {
            return;
        }

        for a in Axis::vec_all() {
            let gmt = a.greedy_mesh_traverse_coords();

            if gmt.contains(&AxisCoord::X) && gmt.contains(&AxisCoord::Y) {
                for z in [0, CHUNK_SIZE.2 - 1] {
                    for y in 0..CHUNK_SIZE.1 {
                        for x in 1..CHUNK_SIZE.0 {
                            if self.get_block(x, y, z) != self.get_block(x - 1, y, z) {
                                continue;
                            }

                            let fs1 = self.blocks.get_face_size((x, y, z), a);
                            let fs2 = self.blocks.get_face_size((x - 1, y, z), a);

                            self.blocks.set_face_size((x, y, z), a, (fs1.0 + fs2.0, fs1.1, fs1.2));
                            self.blocks.disable_mesh((x - 1, y, z), a);
                        }
                    }

                    for x in 0..CHUNK_SIZE.0 {
                        for y in 1..CHUNK_SIZE.1 {
                            if self.blocks.mesh_enabled((x, y, z), a) == false {
                                continue;
                            }

                            if self.blocks.mesh_enabled((x, y - 1, z), a) == false {
                                continue;
                            }

                            if self.get_block(x, y, z) != self.get_block(x, y - 1, z) {
                                continue;
                            }

                            let fs1 = self.blocks.get_face_size((x, y, z), a);
                            let fs2 = self.blocks.get_face_size((x, y - 1, z), a);

                            if fs1.0 != fs2.0 {
                                continue;
                            }

                            self.blocks.set_face_size((x, y, z), a, (fs1.0, fs1.1 + fs2.1, fs1.2));
                            self.blocks.disable_mesh((x, y - 1, z), a);
                        }
                    }
                }
            }
        }
    }

    // This function is for seeing whether a block should have
    // certain faces, or if the mesh is disabled, or anything like that.
    fn visible_info(&mut self, x: u8, y: u8, z: u8) -> VisibleInfo {
        let axis_list = Axis::vec_all();
        let size_list: Vec<(u8, u8, u8)> = vec![
            (1, 1, 1),
            (1, 1, 1),
            (1, 1, 1),
            (1, 1, 1),
            (1, 1, 1),
            (1, 1, 1),
        ];

        if CULL_FACES == false {
            return VisibleInfo {
                sides: axis_list,
                sizes: size_list,
            };
        }

        let mut vsides: Vec<Axis> = Vec::new();
        let mut vsizes: Vec<(u8, u8, u8)> = Vec::new();

        for i in axis_list.iter() {
            let (ox, oy, oz) = i.coord_offset_from(x.into(), y.into(), z.into());

            let block = self.get_block(ox as u8, oy as u8, oz as u8);

            let p = block.properties();

            if p.transparent == true && self.get_block(x, y, z).properties().transparent == false {
                vsides.push(*i);
            }
        }

        vsizes = size_list.clone(); // TMP

        if GREEDY_MESHING == false {
            vsizes = size_list;
        }

        return VisibleInfo {
            sides: vsides,
            sizes: vsizes,
        };
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

struct VisibleInfo {
    sides: Vec<Axis>,
    sizes: Vec<(u8, u8, u8)>,
}
