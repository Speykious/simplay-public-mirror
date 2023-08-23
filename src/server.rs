#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Resource)]
pub struct InternalServer {}

impl InternalServer {
    pub fn new() -> Self {
        return Self {};
    }
}
