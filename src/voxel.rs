#![allow(dead_code)]

use bevy::prelude::*;
use crate::world::Axis;
use crate::library::*;

// Properties of a block.
#[derive(Default, Debug)]
pub struct BlockProperties {
    pub transparent: bool,
    pub hardness: u8, // This basically calculates how hard it is to move through the block. (u8::MIN = nothing, u8::MAX completely solid)
    pub drop: Option<Block>,
    pub textures: [Option<usize>; 6],
}

impl BlockProperties {
    pub fn default() -> BlockProperties {
        return BlockProperties {
            transparent: false,
            hardness: u8::MAX,
            drop: Some(Block::Debug),
            textures: [
                Some(0); 6
            ],
        };
    }
}

// Block types.
#[derive(Debug, Clone, Copy)]
pub enum Block {
    Void, // Basically null.
    Air,
    Debug,
}

impl Block {
    pub fn properties(&self) -> BlockProperties {
        return match self {
            Self::Debug => BlockProperties {
                ..BlockProperties::default()
            },

            Self::Air => BlockProperties {
                transparent: true,
                hardness: u8::MIN,
                drop: None,
                textures: [
                    None; 6
                ],
                ..BlockProperties::default()
            },

            Self::Void => BlockProperties {
                transparent: true,
                hardness: u8::MIN,
                drop: None,
                textures: [
                    None; 6
                ],
                ..BlockProperties::default()
            },
        };
    }
}

// Voxel for actually building a mesh.
#[derive(Debug)]
pub struct Voxel {
    pub id: Block,
    pub sides: Vec<VoxelSide>,
}

impl Voxel {
    // Really only use this for testing.
    pub fn into_mesh(&self) -> Mesh {
        let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for i in self.sides.iter() {
            vertices.extend(i.vertices.clone());

            indices = combine_indices(&vec![indices, i.indices.clone()]);
        }

        return create_mesh(&vertices, &indices);
    }
}

// Voxel side meshes, basically.
#[derive(Debug, PartialEq)]
pub struct VoxelSide {
    pub side: Axis,
    pub vertices: Vec<([f32; 3], [f32; 3], [f32; 2])>,
    pub indices: Vec<u32>,
    pub pos: (u8, u8, u8),
}

impl VoxelSide {
    pub fn new(side: Axis, pos: (u8, u8, u8)) -> Self {
        let (min_x, min_y, min_z) = (-0.5, -0.5, -0.5);
        let (max_x, max_y, max_z) = (0.5, 0.5, 0.5);

        let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = Vec::new();

        let indices = vec![0, 1, 2, 2, 3, 0];

        match side {
            Axis::North => {
                for j in [
                    ([min_x + pos.0 as f32, max_y + pos.1 as f32, min_z + pos.2 as f32], [0., 0., -1.0], [1.0, 0.]),
                    ([max_x + pos.0 as f32, max_y + pos.1 as f32, min_z + pos.2 as f32], [0., 0., -1.0], [0., 0.]),
                    ([max_x + pos.0 as f32, min_y + pos.1 as f32, min_z + pos.2 as f32], [0., 0., -1.0], [0., 1.0]),
                    ([min_x + pos.0 as f32, min_y + pos.1 as f32, min_z + pos.2 as f32], [0., 0., -1.0], [1.0, 1.0]),
                ] {
                    vertices.push(j);
                }
            },
            Axis::South => {
                for j in [
                    ([min_x + pos.0 as f32, min_y + pos.1 as f32, max_z + pos.2 as f32], [0., 0., 1.0], [0., 0.]),
                    ([max_x + pos.0 as f32, min_y + pos.1 as f32, max_z + pos.2 as f32], [0., 0., 1.0], [1.0, 0.]),
                    ([max_x + pos.0 as f32, max_y + pos.1 as f32, max_z + pos.2 as f32], [0., 0., 1.0], [1.0, 1.0]),
                    ([min_x + pos.0 as f32, max_y + pos.1 as f32, max_z + pos.2 as f32], [0., 0., 1.0], [0., 1.0]),
                ] {
                    vertices.push(j);
                }
            },
            Axis::East => {
                for j in [
                    ([max_x + pos.0 as f32, min_y + pos.1 as f32, min_z + pos.2 as f32], [1.0, 0., 0.], [0., 0.]),
                    ([max_x + pos.0 as f32, max_y + pos.1 as f32, min_z + pos.2 as f32], [1.0, 0., 0.], [1.0, 0.]),
                    ([max_x + pos.0 as f32, max_y + pos.1 as f32, max_z + pos.2 as f32], [1.0, 0., 0.], [1.0, 1.0]),
                    ([max_x + pos.0 as f32, min_y + pos.1 as f32, max_z + pos.2 as f32], [1.0, 0., 0.], [0., 1.0]),
                ] {
                    vertices.push(j);
                }
            },
            Axis::West => {
                for j in [
                    ([min_x + pos.0 as f32, min_y + pos.1 as f32, max_z + pos.2 as f32], [-1.0, 0., 0.], [1.0, 0.]),
                    ([min_x + pos.0 as f32, max_y + pos.1 as f32, max_z + pos.2 as f32], [-1.0, 0., 0.], [0., 0.]),
                    ([min_x + pos.0 as f32, max_y + pos.1 as f32, min_z + pos.2 as f32], [-1.0, 0., 0.], [0., 1.0]),
                    ([min_x + pos.0 as f32, min_y + pos.1 as f32, min_z + pos.2 as f32], [-1.0, 0., 0.], [1.0, 1.0]),
                ] {
                    vertices.push(j);
                }
            },
            Axis::Up => {
                for j in [
                    ([max_x + pos.0 as f32, max_y + pos.1 as f32, min_z + pos.2 as f32], [0., 1.0, 0.], [1.0, 0.]),
                    ([min_x + pos.0 as f32, max_y + pos.1 as f32, min_z + pos.2 as f32], [0., 1.0, 0.], [0., 0.]),
                    ([min_x + pos.0 as f32, max_y + pos.1 as f32, max_z + pos.2 as f32], [0., 1.0, 0.], [0., 1.0]),
                    ([max_x + pos.0 as f32, max_y + pos.1 as f32, max_z + pos.2 as f32], [0., 1.0, 0.], [1.0, 1.0]),
                ] {
                    vertices.push(j);
                }
            },
            Axis::Down => {
                for j in [
                    ([max_x + pos.0 as f32, min_y + pos.1 as f32, max_z + pos.2 as f32], [0., -1.0, 0.], [0., 0.]),
                    ([min_x + pos.0 as f32, min_y + pos.1 as f32, max_z + pos.2 as f32], [0., -1.0, 0.], [1.0, 0.]),
                    ([min_x + pos.0 as f32, min_y + pos.1 as f32, min_z + pos.2 as f32], [0., -1.0, 0.], [1.0, 1.0]),
                    ([max_x + pos.0 as f32, min_y + pos.1 as f32, min_z + pos.2 as f32], [0., -1.0, 0.], [0., 1.0]),
                ] {
                    vertices.push(j);
                }
            },
        };

        return Self {
            side,
            pos,
            vertices,
            indices,
        };
    }

    pub fn vec_from_axis_vec(axis_vec: &Vec<Axis>, pos: (u8, u8, u8)) -> Vec<Self> {
        let mut face_vec: Vec<Self> = Vec::new();

        for i in axis_vec.iter() {
            face_vec.push(Self::new(*i, pos));
        }

        return face_vec;
    }

    pub fn coord_offset(&self) -> (i8, i8, i8) {
        let sp = self.pos;

        return self.side.coord_offset_from(sp.0 as i16, sp.1 as i16, sp.2 as i16);
    }
}
