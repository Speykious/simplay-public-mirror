#![allow(dead_code)]

use bracket_noise::prelude::*;

pub fn noise_3d(x: f32, y: f32, z: f32) -> f32 {
    let mut noise = FastNoise::seeded(2386972359623);

    let amp = 200.0;
    let freq = 0.1;
    let octaves = 8;

    noise.set_noise_type(NoiseType::Perlin);
    noise.set_frequency(freq);
    noise.set_fractal_octaves(octaves);

    let value = noise.get_noise3d(x, y, z) * amp;

    return value;
}
