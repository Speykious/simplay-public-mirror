#![allow(dead_code)]

use bracket_noise::prelude::*;

use crate::block::*;
use crate::chunk::*;
use crate::noise::*;
use crate::random;

const SEED: u64 = 6756747645;

pub fn regular(chunk: &mut Chunk) {
    let mut ns = NoiseSettings::new();

    ns.set_amp(200.0);
    ns.set_freq(0.1);
    ns.set_octaves(8);
    ns.set_noise_type(NoiseType::Perlin);

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            for z in 0..CHUNK_SIZE.z {
                let block_pos = BlockPos::new_unchecked(x, y, z);
                let (wx, wy, wz) = chunk.pos_local_to_global(block_pos);

                let a = noise_3d((wx as f32, wy as f32, wz as f32), SEED, &ns);

                if a > 25.0 {
                    let block = random::choice(&vec![
                        BlockType::Dirt,
                        BlockType::Stone,
                        BlockType::Diamond,
                    ]);

                    chunk.set_block(block_pos, block);
                }
            }
        }
    }
}
