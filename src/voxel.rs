#![allow(dead_code)]

use bevy::prelude::*;
use hashbrown::HashMap;

use crate::block::*;
use crate::world;
use crate::mesher;
use crate::mesher::MeshInfo;
use crate::chunk;

pub struct Voxel {
    pub block: BlockType,
    pub position: (u8, u8, u8),
    sides: Vec<world::Direction>,
}

impl Voxel {
    pub fn new(position: (u8, u8, u8), block: BlockType) -> Self {
        return Self {
            block,
            position,
            sides: Vec::new(),
        };
    }

    pub fn enable_side(&mut self, direction: world::Direction) {
        if self.side_enabled(direction) == false {
            self.sides.push(direction);
        }
    }

    pub fn disable_side(&mut self, direction: world::Direction) {
        self.sides = self.sides.into_iter()
            .filter(|x| *x != direction)
            .collect();
    }

    pub fn enable_sides(&mut self, directions: &Vec<world::Direction>) {
        for i in directions.iter() {
            self.enable_side(*i);
        }
    }

    pub fn disable_sides(&mut self, directions: &Vec<world::Direction>) {
        for i in directions.iter() {
            self.disable_side(*i);
        }
    }

    pub fn side_enabled(&self, direction: world::Direction) -> bool {
        if self.sides.contains(&direction) {
            return true;
        } else {
            return false;
        }
    }
}

fn mesh_info_from_voxels(voxels: &Vec<Voxel>) -> Vec<mesher::MeshInfo> {
    let mut mesh_info_map: HashMap<(u8, u8, u8), MeshInfo> = HashMap::new();

    for voxel in voxels.iter() {
        todo!();
    }

    let mut mesh_info_vec: Vec<MeshInfo> = Vec::new();

    for x in 0..chunk::CHUNK_SIZE.0 {
        for y in 0..chunk::CHUNK_SIZE.1 {
            for z in 0..chunk::CHUNK_SIZE.2 {
                mesh_info_vec.push(match mesh_info_map.get(&(x, y, z)) {
                    Some(s) => *s,
                    None => {
                        println!("Something went wrong inside of mesh_info_from_voxels()!");

                        MeshInfo::trash() // Just returns trash values.
                    },
                });
            }
        }
    }

    return mesh_info_vec;
}

pub fn mesh_from_voxels(voxels: &Vec<Voxel>) -> Mesh {
    return mesher::create_mesh(&mesh_info_from_voxels(&voxels));
}
