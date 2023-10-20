#![allow(dead_code)]

use crate::block::*;
use crate::world;
use crate::mesher::MeshData;
use crate::filesystem::*;
use crate::places;
use crate::asset_manager::{AtlasUVMapElement, BlockAtlasInfo};

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
            self.sides.push(direction.clone());
        }
    }

    pub fn disable_side(&mut self, direction: world::Direction) {
        self.sides = self.sides.clone().into_iter()
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

    pub fn get_sides(&self) -> Vec<world::Direction> {
        return self.sides.clone();
    }

    pub fn get_side_as_mdi(&self, direction: world::Direction, size: (u8, u8, u8)) -> Option<(Vec<MeshData>, Vec<u32>)> {
        if self.side_enabled(direction) == false {
            return None;
        }

        let atlas_toml: BlockAtlasInfo = match toml::from_str(&match file::read(&places::custom_built_assets().add_str("block_atlas.toml")) {
            Ok(o) => o,
            Err(_) => return None, // Kind of a weird way to handle this error, I know. (This error should never trigger.)
        }) {
            Ok(o) => o,
            Err(_) => return None,
        };

        let texture_name: String = match self.block.properties().textures.get(direction) {
            Some(s) => s,
            None => return None,
        };

        let uv_mod: AtlasUVMapElement = *atlas_toml.uv_map.get(&texture_name).unwrap_or(&AtlasUVMapElement { corner: (0, 0) , size: (0, 0) });
        let atlas_size: (u32, u32) = atlas_toml.size;

        drop(atlas_toml);
        drop(texture_name);

        let (min_x, min_y, min_z) = (-0.5 - (size.0 - 1) as f32, -0.5 - (size.1 - 1) as f32, -0.5 - (size.2 - 1) as f32);
        let (max_x, max_y, max_z) = (0.5, 0.5, 0.5);

        let indices = vec![0, 1, 2, 2, 3, 0];

        let uv_locate_info_1: (u32, u32) = uv_mod.corner; // Top left.
        let uv_locate_info_2: (u32, u32) = (uv_locate_info_1.0 + uv_mod.size.0, uv_locate_info_1.1 + uv_mod.size.1); // Bottom right.

        let uv_limits_x: (f32, f32) = (uv_locate_info_1.0 as f32 / atlas_size.0 as f32, uv_locate_info_2.0 as f32 / atlas_size.0 as f32);
        let uv_limits_y: (f32, f32) = (uv_locate_info_1.1 as f32 / atlas_size.1 as f32, uv_locate_info_2.1 as f32 / atlas_size.1 as f32);

        let mut general: Vec<([f32; 3], [f32; 3], [f32; 2])> = Vec::new();

        match direction {
            world::Direction::North => {
                for j in [
                    ([min_x + self.position.0 as f32, max_y + self.position.1 as f32, min_z + self.position.2 as f32], [0., 0., -1.0], [uv_limits_x.1, uv_limits_y.0]),
                    ([max_x + self.position.0 as f32, max_y + self.position.1 as f32, min_z + self.position.2 as f32], [0., 0., -1.0], [uv_limits_x.0, uv_limits_y.0]),
                    ([max_x + self.position.0 as f32, min_y + self.position.1 as f32, min_z + self.position.2 as f32], [0., 0., -1.0], [uv_limits_x.0, uv_limits_y.1]),
                    ([min_x + self.position.0 as f32, min_y + self.position.1 as f32, min_z + self.position.2 as f32], [0., 0., -1.0], [uv_limits_x.1, uv_limits_y.1]),
                ] {
                    general.push(j);
                }
            },
            world::Direction::South => {
                for j in [
                    ([min_x + self.position.0 as f32, min_y + self.position.1 as f32, max_z + self.position.2 as f32], [0., 0., 1.0], [uv_limits_x.1, uv_limits_y.1]),
                    ([max_x + self.position.0 as f32, min_y + self.position.1 as f32, max_z + self.position.2 as f32], [0., 0., 1.0], [uv_limits_x.0, uv_limits_y.1]),
                    ([max_x + self.position.0 as f32, max_y + self.position.1 as f32, max_z + self.position.2 as f32], [0., 0., 1.0], [uv_limits_x.0, uv_limits_y.0]),
                    ([min_x + self.position.0 as f32, max_y + self.position.1 as f32, max_z + self.position.2 as f32], [0., 0., 1.0], [uv_limits_x.1, uv_limits_y.0]),
                ] {
                    general.push(j);
                }
            },
            world::Direction::East => {
                for j in [
                    ([max_x + self.position.0 as f32, min_y + self.position.1 as f32, min_z + self.position.2 as f32], [1.0, 0., 0.], [uv_limits_x.0, uv_limits_y.1]),
                    ([max_x + self.position.0 as f32, max_y + self.position.1 as f32, min_z + self.position.2 as f32], [1.0, 0., 0.], [uv_limits_x.0, uv_limits_y.0]),
                    ([max_x + self.position.0 as f32, max_y + self.position.1 as f32, max_z + self.position.2 as f32], [1.0, 0., 0.], [uv_limits_x.1, uv_limits_y.0]),
                    ([max_x + self.position.0 as f32, min_y + self.position.1 as f32, max_z + self.position.2 as f32], [1.0, 0., 0.], [uv_limits_x.1, uv_limits_y.1]),
                ] {
                    general.push(j);
                }
            },
            world::Direction::West => {
                for j in [
                    ([min_x + self.position.0 as f32, min_y + self.position.1 as f32, max_z + self.position.2 as f32], [-1.0, 0., 0.], [uv_limits_x.1, uv_limits_y.1]),
                    ([min_x + self.position.0 as f32, max_y + self.position.1 as f32, max_z + self.position.2 as f32], [-1.0, 0., 0.], [uv_limits_x.1, uv_limits_y.0]),
                    ([min_x + self.position.0 as f32, max_y + self.position.1 as f32, min_z + self.position.2 as f32], [-1.0, 0., 0.], [uv_limits_x.0, uv_limits_y.0]),
                    ([min_x + self.position.0 as f32, min_y + self.position.1 as f32, min_z + self.position.2 as f32], [-1.0, 0., 0.], [uv_limits_x.0, uv_limits_y.1]),
                ] {
                    general.push(j);
                }
            },
            world::Direction::Up => {
                for j in [
                    ([max_x + self.position.0 as f32, max_y + self.position.1 as f32, min_z + self.position.2 as f32], [0., 1.0, 0.], [uv_limits_x.1, uv_limits_y.0]),
                    ([min_x + self.position.0 as f32, max_y + self.position.1 as f32, min_z + self.position.2 as f32], [0., 1.0, 0.], [uv_limits_x.0, uv_limits_y.0]),
                    ([min_x + self.position.0 as f32, max_y + self.position.1 as f32, max_z + self.position.2 as f32], [0., 1.0, 0.], [uv_limits_x.0, uv_limits_y.1]),
                    ([max_x + self.position.0 as f32, max_y + self.position.1 as f32, max_z + self.position.2 as f32], [0., 1.0, 0.], [uv_limits_x.1, uv_limits_y.1]),
                ] {
                    general.push(j);
                }
            },
            world::Direction::Down => {
                for j in [
                    ([max_x + self.position.0 as f32, min_y + self.position.1 as f32, max_z + self.position.2 as f32], [0., -1.0, 0.], [uv_limits_x.0, uv_limits_y.0]),
                    ([min_x + self.position.0 as f32, min_y + self.position.1 as f32, max_z + self.position.2 as f32], [0., -1.0, 0.], [uv_limits_x.1, uv_limits_y.0]),
                    ([min_x + self.position.0 as f32, min_y + self.position.1 as f32, min_z + self.position.2 as f32], [0., -1.0, 0.], [uv_limits_x.1, uv_limits_y.1]),
                    ([max_x + self.position.0 as f32, min_y + self.position.1 as f32, min_z + self.position.2 as f32], [0., -1.0, 0.], [uv_limits_x.0, uv_limits_y.1]),
                ] {
                    general.push(j);
                }
            },
        };

        let mesh_data: Vec<MeshData> = MeshData::array_from_general_array(&general);

        return Some((mesh_data, indices));
    }
}

// MDI = MeshData and Indices
pub mod mdi_from {
    use crate::mesher::{self, MeshData};
    use crate::voxel::*;

    pub fn voxel_array(voxels: &Vec<Voxel>) -> (Vec<MeshData>, Vec<u32>) {
        let mut mesh_data: Vec<MeshData> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
    
        for v in voxels.iter() {
            let mdi = voxel(v);

            mesh_data.extend(mdi.0);

            indices = mesher::combine_indices(&vec![indices, mdi.1]);
        }
    
        return (mesh_data, indices);
    }

    pub fn voxel(v: &Voxel) -> (Vec<MeshData>, Vec<u32>) {
        let mut mesh_data: Vec<MeshData> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for s in v.get_sides() {
            let mdi = match v.get_side_as_mdi(s, (1, 1, 1)) {
                Some(s) => s,
                None => panic!("Oh shoot! I couldn't get a face for: {:?} (File this as a bug report!)", s), // If this is ever triggered, there is a bug.
            };

            mesh_data.extend(mdi.0);

            indices = mesher::combine_indices(&vec![indices, mdi.1]);
        }

        return (mesh_data, indices);
    }
}
