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
#[derive(Debug)]
pub enum Block {
    Debug,
    Air,
}

impl Block {
    pub fn properties(block: Block) -> BlockProperties {
        return match block {
            Block::Debug => BlockProperties {
                ..Default::default()
            },

            Block::Air => BlockProperties {
                transparent: true,
                hardness: u8::MIN,
                drop: None,
                textures: [
                    None; 6
                ],
                ..Default::default()
            },
        };
    }
}

// Voxel for actually building a mesh.
pub struct Voxel {
    pub id: Block,
    pub position: Vec3,
    pub sides: Vec<Axis>,
}

// Return mesh for a voxel.
pub fn create_voxel_mesh(voxel: Voxel) -> Mesh {
    let (min_x, min_y, min_z) = (-1.0, -0.5, -0.5);
    let (max_x, max_y, max_z) = (1.0, 0.5, 0.5);

    let vertices = vec![
        // Front.
        ([min_x, max_y, min_z], [0., 0., -1.0], [1.0, 0.]),
        ([max_x, max_y, min_z], [0., 0., -1.0], [0., 0.]),
        ([max_x, min_y, min_z], [0., 0., -1.0], [0., 1.0]),
        ([min_x, min_y, min_z], [0., 0., -1.0], [1.0, 1.0]),
        // Back.
        ([min_x, min_y, max_z], [0., 0., 1.0], [0., 0.]),
        ([max_x, min_y, max_z], [0., 0., 1.0], [1.0, 0.]),
        ([max_x, max_y, max_z], [0., 0., 1.0], [1.0, 1.0]),
        ([min_x, max_y, max_z], [0., 0., 1.0], [0., 1.0]),
        // Right.
        ([max_x, min_y, min_z], [1.0, 0., 0.], [0., 0.]),
        ([max_x, max_y, min_z], [1.0, 0., 0.], [1.0, 0.]),
        ([max_x, max_y, max_z], [1.0, 0., 0.], [1.0, 1.0]),
        ([max_x, min_y, max_z], [1.0, 0., 0.], [0., 1.0]),
        // Left.
        ([min_x, min_y, max_z], [-1.0, 0., 0.], [1.0, 0.]),
        ([min_x, max_y, max_z], [-1.0, 0., 0.], [0., 0.]),
        ([min_x, max_y, min_z], [-1.0, 0., 0.], [0., 1.0]),
        ([min_x, min_y, min_z], [-1.0, 0., 0.], [1.0, 1.0]),
        // Top.
        ([max_x, max_y, min_z], [0., 1.0, 0.], [1.0, 0.]),
        ([min_x, max_y, min_z], [0., 1.0, 0.], [0., 0.]),
        ([min_x, max_y, max_z], [0., 1.0, 0.], [0., 1.0]),
        ([max_x, max_y, max_z], [0., 1.0, 0.], [1.0, 1.0]),
        // Bottom.
        ([max_x, min_y, max_z], [0., -1.0, 0.], [0., 0.]),
        ([min_x, min_y, max_z], [0., -1.0, 0.], [1.0, 0.]),
        ([min_x, min_y, min_z], [0., -1.0, 0.], [1.0, 1.0]),
        ([max_x, min_y, min_z], [0., -1.0, 0.], [0., 1.0]),
    ];

    let mut indices: Vec<u32> = Vec::new();

    for i in voxel.sides {
        if i == Axis::North {
            for j in [0, 1, 2, 2, 3, 0] {
                indices.push(j);
            }
        }

        else if i == Axis::South {
            for j in [4, 5, 6, 6, 7, 4] {
                indices.push(j);
            }
        }

        else if i == Axis::East {
            for j in [8, 9, 10, 10, 11, 8] {
                indices.push(j);
            }
        }

        else if i == Axis::West {
            for j in [12, 13, 14, 14, 15, 12] {
                indices.push(j);
            }
        }

        else if i == Axis::Up {
            for j in [16, 17, 18, 18, 19, 16] {
                indices.push(j);
            }
        }

        else if i == Axis::Down {
            for j in [20, 21, 22, 22, 23, 20] {
                indices.push(j);
            }
        }
    }

    return create_mesh(&vertices, &indices);
}
