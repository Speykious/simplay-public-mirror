#![allow(dead_code)]

use bracket_noise::prelude::*;

use crate::noise::*;
use crate::block::*;
use crate::chunk::*;
use crate::random;

const SEED: u64 = 6756747645;

pub fn regular(chunk: &mut Chunk) {
    let mut ns = NoiseSettings::new();

    ns.set_amp(200.0);
    ns.set_freq(0.1);
    ns.set_octaves(8);
    ns.set_noise_type(NoiseType::Perlin);

    for x in 0..CHUNK_SIZE.0 {
        for y in 0..CHUNK_SIZE.1 {
            for z in 0..CHUNK_SIZE.2 {
                let (wx, wy, wz) = chunk.pos_local_to_global(x, y, z);

                let a = noise_3d((wx as f32, wy as f32, wz as f32), SEED, &ns);

                if a > 25.0 {
                    chunk.set_block_u8((x, y, z), random::choice(&vec![BlockType::Dirt, BlockType::Stone, BlockType::Diamond]));
                }
            }
        }
    }
}
