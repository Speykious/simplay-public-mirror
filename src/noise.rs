#![allow(dead_code)]

use bracket_noise::prelude::*;

pub struct NoiseSettings {
    amp: f32,
    freq: f32,
    octaves: i32,
    ntype: NoiseType,
}

impl NoiseSettings {
    pub fn from(amp: f32, freq: f32, octaves: i32, ntype: NoiseType) -> Self {
        return Self {
            amp,
            freq,
            octaves,
            ntype,
        };
    }

    pub fn new() -> Self {
        return Self::from(0.0, 0.0, 0, NoiseType::Perlin);
    }

    pub fn set_amp(&mut self, amp: f32) {
        self.amp = amp;
    }

    pub fn set_freq(&mut self, freq: f32) {
        self.freq = freq;
    }

    pub fn set_octaves(&mut self, octaves: i32) {
        self.octaves = octaves;
    }

    pub fn set_noise_type(&mut self, ntype: NoiseType) {
        self.ntype = ntype;
    }
}

pub fn noise_3d(position: (f32, f32, f32), seed: u64, ns: &NoiseSettings) -> f32 {
    let mut noise = FastNoise::seeded(seed);

    noise.set_noise_type(ns.ntype);
    noise.set_frequency(ns.freq);
    noise.set_fractal_octaves(ns.octaves);

    let value = noise.get_noise3d(position.0, position.1, position.2) * ns.amp;

    return value;
}
