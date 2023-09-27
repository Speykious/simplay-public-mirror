#![allow(dead_code)]

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;

pub mod optimize {
    use crate::mesher::MeshData;

    pub fn share_vertices(mesh_data: &Vec<MeshData>, indices: &Vec<u32>) -> (Vec<MeshData>, Vec<u32>) {
        return (mesh_data.clone(), indices.clone()); // todo
    }
}

// This is just a cleaner way of representing vertices, normals, and uvs all in one object.
#[derive(Clone, Copy)]
pub struct MeshData {
    vertex: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}

impl MeshData {
    pub fn new(vertex: [f32; 3], normal: [f32; 3], uv: [f32; 2]) -> Self {
        return Self {
            vertex,
            normal,
            uv,
        };
    }

    pub fn from_general(general: ([f32; 3], [f32; 3], [f32; 2])) -> Self {
        let vertex: [f32; 3];
        let normal: [f32; 3];
        let uv: [f32; 2];

        vertex = general.0;
        normal = general.1;
        uv = general.2;

        return Self::new(vertex, normal, uv);
    }

    pub fn array_from_general_array(general: &Vec<([f32; 3], [f32; 3], [f32; 2])>) -> Vec<Self> {
        let mut self_vec: Vec<Self> = Vec::new();

        for i in general.iter() {
            self_vec.push(Self::new(i.0, i.1, i.2));
        }

        return self_vec;
    }
}

pub fn create_mesh(mesh_data: &Vec<MeshData>, indices: &Vec<u32>) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let positions: Vec<_> = mesh_data.iter().map(|x| x.vertex).collect();
    let normals: Vec<_> = mesh_data.iter().map(|x| x.normal).collect();
    let uvs: Vec<_> = mesh_data.iter().map(|x| x.uv).collect();

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
