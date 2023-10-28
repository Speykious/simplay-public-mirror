#![allow(dead_code)]

use bevy::prelude::*;
use hashbrown::HashMap;
use std::ops::Not;
use std::ops::Range;
use std::rc::Rc;

use crate::block::BlockType;
use crate::mesher;
use crate::places;
use crate::voxel::mdi_from;
use crate::voxel::Voxel;
use crate::world;
use crate::world_generation;

pub struct ChunkManagerPlugin;

impl Plugin for ChunkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::new());
        app.add_systems(Startup, test_chunks);
    }
}

pub const CHUNK_SIZE: BlockPos = BlockPos::new_unchecked(16, 16, 16);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChunkPos {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl ChunkPos {
    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockPos {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl BlockPos {
    pub const fn new(x: u8, y: u8, z: u8) -> Option<Self> {
        let pos = Self { x, y, z };

        if pos.overflows_chunk() {
            return None;
        }

        Some(pos)
    }

    pub const fn new_unchecked(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }

    /// Is the position outside of a chunk?
    pub const fn overflows_chunk(&self) -> bool {
        self.x > CHUNK_SIZE.x || self.y > CHUNK_SIZE.y || self.z > CHUNK_SIZE.z
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BlockOffset {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl BlockOffset {
    pub const fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }

    pub fn full_pos(&self, chunk_pos: ChunkPos) -> (ChunkPos, BlockPos) {
        let overflow_directions = self.overflow_direction();

        let mut d_chunk_pos = chunk_pos;
        for overflow_direction in overflow_directions {
            match overflow_direction {
                world::Direction::North => d_chunk_pos.z -= 1,
                world::Direction::South => d_chunk_pos.z += 1,
                world::Direction::East => d_chunk_pos.x += 1,
                world::Direction::West => d_chunk_pos.x -= 1,
                world::Direction::Up => d_chunk_pos.y += 1,
                world::Direction::Down => d_chunk_pos.y -= 1,
            }
        }

        (d_chunk_pos, self.wrap())
    }

    // Wrap an overflowing position to a regular position.
    pub fn wrap(&self) -> BlockPos {
        let x: u8 = self.x.rem_euclid(CHUNK_SIZE.x as i8) as u8;
        let y: u8 = self.y.rem_euclid(CHUNK_SIZE.y as i8) as u8;
        let z: u8 = self.z.rem_euclid(CHUNK_SIZE.z as i8) as u8;

        BlockPos::new_unchecked(x, y, z)
    }

    // What direction does a position overflow in?
    pub fn overflow_direction(&self) -> Vec<world::Direction> {
        let mut directions: Vec<world::Direction> = Vec::new();

        if self.position_overflow().not() {
            return directions;
        }

        if self.x < 0 {
            directions.push(world::Direction::West);
        } else if self.x > (CHUNK_SIZE.x - 1) as i8 {
            directions.push(world::Direction::East);
        }

        if self.y < 0 {
            directions.push(world::Direction::Down);
        } else if self.y > (CHUNK_SIZE.y - 1) as i8 {
            directions.push(world::Direction::Up);
        }

        if self.z < 0 {
            directions.push(world::Direction::North);
        } else if self.z > (CHUNK_SIZE.z - 1) as i8 {
            directions.push(world::Direction::South);
        }

        directions
    }

    // Is a position outside of a chunk?
    pub fn position_overflow(&self) -> bool {
        (self.x < 0 || self.x >= CHUNK_SIZE.x as i8)
            || (self.y < 0 || self.y >= CHUNK_SIZE.y as i8)
            || (self.z < 0 || self.z >= CHUNK_SIZE.z as i8)
    }
}

#[derive(Debug)]
pub struct Chunk {
    /// Chunk position.
    pub pos: ChunkPos,
    /// The reason that I am using i8 instead of u8, is so I can read the blocks of neighboring chunks.
    blocks: HashMap<BlockPos, BlockType>,
}

impl Chunk {
    pub fn new(pos: ChunkPos) -> Self {
        Self {
            pos,
            blocks: HashMap::new(),
        }
    }

    pub fn mesh(&self) -> Mesh {
        // Voxels store data like what sides need to be drawn.
        let mut voxels: Vec<Voxel> = Vec::new();

        // Loop through every block in the chunk.
        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                for z in 0..CHUNK_SIZE.z {
                    let ibp = BlockPos::new_unchecked(x, y, z);

                    let block = self.get_block(ibp);

                    let mut voxel_data = Voxel::new((x, y, z), block);

                    // Loop through all the neighboring blocks, and check if a face should be drawn.
                    for d in world::Direction::all() {
                        let (dx, dy, dz) =
                            d.offset_with_position((x as isize, y as isize, z as isize)); // Returns isizes.
                        let dbp = BlockPos::new_unchecked(dx as u8, dy as u8, dz as u8); // Convert to (i8, i8, i8).

                        let d_block = self.get_block(dbp);

                        if Self::is_face(block, d_block) {
                            voxel_data.enable_side(d);
                        }
                    }

                    voxels.push(voxel_data);
                }
            }
        }

        let (mesh_data, indices) = mdi_from::voxel_array(&voxels);

        mesher::create_mesh(&mesh_data, &indices)
    }

    // Used in self.mesh() to check whether a block needs a face or not.
    fn is_face(block: BlockType, d_block: BlockType) -> bool {
        if block.properties().transparent && block == d_block {
            return false;
        }

        d_block.properties().transparent
    }

    pub fn get_block(&self, block_pos: BlockPos) -> BlockType {
        self.blocks
            .get(&block_pos)
            .copied()
            .unwrap_or(BlockType::Air)
    }

    pub fn set_block(&mut self, position: BlockPos, blocktype: BlockType) {
        match blocktype {
            BlockType::Air => self.blocks.remove(&position),
            _ => self.blocks.insert(position, blocktype),
        };
    }

    pub fn set_all_blocks_from_hashmap(&mut self, blocks: HashMap<BlockPos, BlockType>) {
        for (k, v) in blocks.iter() {
            self.set_block(*k, *v);
        }
    }

    pub fn pos_local_to_global(&self, block_pos: BlockPos) -> (isize, isize, isize) {
        (
            self.pos_local_to_global_single(block_pos.x, world::Axis::X),
            self.pos_local_to_global_single(block_pos.y, world::Axis::Y),
            self.pos_local_to_global_single(block_pos.z, world::Axis::Z),
        )
    }

    pub fn pos_local_to_global_single(&self, s: u8, coord_type: world::Axis) -> isize {
        let csize = match coord_type {
            world::Axis::X => CHUNK_SIZE.x,
            world::Axis::Y => CHUNK_SIZE.y,
            world::Axis::Z => CHUNK_SIZE.z,
        };

        let cpos = match coord_type {
            world::Axis::X => self.pos.x,
            world::Axis::Y => self.pos.y,
            world::Axis::Z => self.pos.z,
        };

        let global_s: isize = (cpos * csize as isize) + s as isize;

        global_s
    }
}

#[derive(Resource)]
struct ChunkManager {
    chunks: HashMap<ChunkPos, Chunk>,
}

impl ChunkManager {
    fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn mesh(&self, chunk_pos: ChunkPos) -> Mesh {
        let chunk = self.chunks.get(&chunk_pos).unwrap();

        // Voxels store data like what sides need to be drawn.
        let mut voxels: Vec<Voxel> = Vec::new();

        // Loop through every block in the chunk.
        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                for z in 0..CHUNK_SIZE.z {
                    let ibp = BlockPos::new_unchecked(x, y, z);

                    let block = chunk.get_block(ibp);

                    let mut voxel_data = Voxel::new((x, y, z), block);

                    // Loop through all the neighboring blocks, and check if a face should be drawn.
                    for d in world::Direction::all() {
                        let (dx, dy, dz) =
                            d.offset_with_position((x as isize, y as isize, z as isize)); // Returns isizes.

                        let d_offset = BlockOffset::new(dx as i8, dy as i8, dz as i8);
                        let (d_chunk_pos, dbp) = d_offset.full_pos(chunk_pos);

                        let d_chunk = self.chunks.get(&d_chunk_pos).unwrap();
                        let d_block = d_chunk.get_block(dbp);

                        if Chunk::is_face(block, d_block) {
                            voxel_data.enable_side(d);
                        }
                    }

                    voxels.push(voxel_data);
                }
            }
        }

        let (mesh_data, indices) = mdi_from::voxel_array(&voxels);

        mesher::create_mesh(&mesh_data, &indices)
    }
}

fn test_chunks(
    mut cmds: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_manager: ResMut<ChunkManager>,
    asset_server: ResMut<AssetServer>,
) {
    let xyz_ranges: Rc<(Range<isize>, Range<isize>, Range<isize>)> = Rc::new((-1..2, -1..2, -1..2));

    for cx in xyz_ranges.0.clone() {
        for cy in xyz_ranges.1.clone() {
            for cz in xyz_ranges.2.clone() {
                let chunk_pos = ChunkPos::new(cx, cy, cz);
                let mut chunk = Chunk::new(chunk_pos);

                world_generation::regular(&mut chunk);

                chunk_manager.chunks.insert(chunk_pos, chunk);
            }
        }
    }

    for cx in xyz_ranges.0.clone() {
        for cy in xyz_ranges.1.clone() {
            for cz in xyz_ranges.2.clone() {
                let chunk_pos = ChunkPos::new(cx, cy, cz);
                let chunk = chunk_manager.chunks.get(&chunk_pos).unwrap();

                cmds.spawn((
                    PbrBundle {
                        mesh: meshes.add(chunk.mesh()),
                        transform: Transform::from_xyz(
                            (cx * CHUNK_SIZE.x as isize) as f32,
                            (cy * CHUNK_SIZE.y as isize) as f32,
                            (cz * CHUNK_SIZE.z as isize) as f32,
                        ),
                        material: materials.add(StandardMaterial {
                            // base_color: Color::rgb(0.05, 0.5, 0.35), // The only reason this is still here, is because I think it is a cool color, and it is a secret comment!
                            base_color_texture: Some(asset_server.load(format!(
                                "{}/block_atlas.png",
                                places::custom_built_assets().to_string()
                            ))),
                            // double_sided: true, // debug
                            // cull_mode: None, // debug
                            reflectance: 0.15,
                            perceptual_roughness: 0.6,
                            ..default()
                        }),
                        ..default()
                    },
                    Name::new(format!("Chunk ({}, {}, {})", cx, cy, cz)),
                ));
            }
        }
    }
}
