#![allow(dead_code)]

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;

pub mod optimize {
    use hashbrown::*;
    use crate::mesher::{MeshData, MeshDataHashable};

    // This seems to just be a useless function. (Shit! That is around 8 hours wasted!)
    pub fn share_vertices(mesh_data: &Vec<MeshData>, indices: &Vec<u32>) -> (Vec<MeshData>, Vec<u32>) {
        let mesh_data_hashable: Vec<MeshDataHashable> = mesh_data.iter().map(|x| x.to_hashable()).collect();

        let mut new_mesh_data: Vec<MeshData> = Vec::new();
        let mut new_indices: Vec<u32> = Vec::new();
        
        let mut mapping: HashMap<MeshDataHashable, u32> = HashMap::new();

        for i in indices.iter() {
            let mdh = mesh_data_hashable[*i as usize];

            if mapping.contains_key(&mdh) == false {
                new_mesh_data.push(mdh.to_mesh_data());
                new_indices.push((new_mesh_data.len() - 1) as u32);
                mapping.insert(mdh, (new_mesh_data.len() - 1) as u32);
            }

            else {
                new_indices.push(*mapping.get(&mdh).unwrap());
            }
        }

        println!("MAPPING LEN: {}", mapping.len());

        return (new_mesh_data, new_indices);
    }
}

// This is just a cleaner way of representing vertices, normals, and uvs all in one object.
#[derive(Debug, Clone, Copy)]
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

    pub fn to_hashable(&self) -> MeshDataHashable {
        let sv = self.vertex;
        let sn = self.normal;
        let su = self.uv;
        
        return MeshDataHashable {
            vertex: (MeshDataF32Hashable::new(sv[0]), MeshDataF32Hashable::new(sv[1]), MeshDataF32Hashable::new(sv[2])),
            normal: (MeshDataF32Hashable::new(sn[0]), MeshDataF32Hashable::new(sn[1]), MeshDataF32Hashable::new(sn[2])),
            uv: (MeshDataF32Hashable::new(su[0]), MeshDataF32Hashable::new(su[1]))
        };
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct MeshDataHashable {
    vertex: (MeshDataF32Hashable, MeshDataF32Hashable, MeshDataF32Hashable),
    normal: (MeshDataF32Hashable, MeshDataF32Hashable, MeshDataF32Hashable),
    uv: (MeshDataF32Hashable, MeshDataF32Hashable),
}

impl MeshDataHashable {
    pub fn to_mesh_data(&self) -> MeshData {
        let sv = self.vertex;
        let sn = self.normal;
        let su = self.uv;
        
        return MeshData::new([sv.0.to_f32(), sv.1.to_f32(), sv.2.to_f32()], [sn.0.to_f32(), sn.1.to_f32(), sn.2.to_f32()], [su.0.to_f32(), su.1.to_f32()]);
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct MeshDataF32Hashable {
    value: (i32, i32),
    is_negative: bool, // For whatever reason, .parse() does not support parsing negative numbers! ( assert!("-2".parse().unwrap() == 2); )
}

impl MeshDataF32Hashable {
    pub fn new(v: f32) -> Self {
        let mut tnum: Vec<i32> = Vec::new();

        // This is for fixing an issue where the "-" sign is lost on numbers starting with -0.x.
        let is_negative_0_starting = v.to_string().starts_with("-0");

        for i in v.to_string().trim().split(".") {
            let int: i32 = i.parse().unwrap();
            
            tnum.push(int);
        }

        if tnum.len() == 1 {
            tnum.push(0);
        }

        return Self {
            value: (tnum[0], tnum[1]),
            is_negative: is_negative_0_starting,
        };
    }

    pub fn to_f32(&self) -> f32 {
        let mut return_value: f32 = format!("{}.{}", self.value.0, self.value.1).trim().parse().unwrap();

        if self.is_negative {
            return_value = -return_value;
        }

        return return_value;
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
