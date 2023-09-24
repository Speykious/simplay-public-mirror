#![allow(dead_code)]

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;

#[derive(Clone, Copy)]
pub struct MeshInfo {
    pub general: ([f32; 3], [f32; 3], [f32; 2]),
    pub index: u32,
}

impl MeshInfo {
    // Returns trash values.
    pub fn trash() -> Self {
        return Self {
            general: ([0.0; 3], [0.0; 3], [0.0; 2]),
            index: 0,
        };
    }
}

pub fn create_mesh(mesh_info_vec: &Vec<MeshInfo>) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut general: Vec<([f32; 3], [f32; 3], [f32; 2])> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for i in mesh_info_vec.iter() {
        general.push(i.general);
        indices.push(i.index);
    }

    let positions: Vec<_> = general.iter().map(|(p, _, _)| *p).collect();
    let normals: Vec<_> = general.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = general.iter().map(|(_, _, uv)| *uv).collect();

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh.set_indices(Some(Indices::U32(indices.clone())));

    return mesh;
}
