#![allow(dead_code)]

use bracket_noise::prelude::*;
use crate::noise::*;

const SEED: u64 = 6756747645;

pub fn regular(x: isize, y: isize, z: isize) -> f32 {
    let mut ns = NoiseSettings::new();

    ns.set_amp(200.0);
    ns.set_freq(0.1);
    ns.set_octaves(8);
    ns.set_noise_type(NoiseType::Perlin);

    return noise_3d((x as f32, y as f32, z as f32), SEED, ns);
}
