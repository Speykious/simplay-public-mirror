#![allow(dead_code)]

use std::ops::Range;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use rand::Rng;

pub fn create_mesh(vertices: &Vec<([f32; 3], [f32; 3], [f32; 2])>, indices: &Vec<u32>) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices.clone())));

    return mesh;
}

pub fn combine_indices(indices_vec: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut new_vec: Vec<u32> = Vec::new();

    for v in indices_vec.iter() {
        let offset: u32 = match new_vec.iter().max() {
            Some(s) => *s + 1,
            None => 0,
        };

        for i in v.iter() {
            new_vec.push(i + offset);
        }
    }

    return new_vec;
}

pub fn randint(range: Range<i32>) -> i32 {
    return rand::thread_rng().gen_range(range);
}
