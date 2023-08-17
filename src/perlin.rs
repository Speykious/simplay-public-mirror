#![allow(dead_code)]

use bracket_noise::prelude::*;

pub fn noise_3d(x: f32, y: f32, z: f32) -> f32 {
    let mut noise = FastNoise::seeded(2386972359623);

    println!("DEBUG: COORDS: {}, {}, {}", x, y, z);

    let amp = 50.0;
    let freq = 0.15;
    let octaves = 8;

    println!("DEBUG: AMP: {}, FREQ: {}, OCT: {}", amp, freq, octaves);

    noise.set_noise_type(NoiseType::Perlin);
    noise.set_frequency(freq);
    noise.set_fractal_octaves(octaves);

    let value = noise.get_noise3d(x, y, z) * amp;

    println!("DEBUG: VALUE: {}", value);

    return value;
}
