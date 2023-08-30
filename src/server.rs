#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Debug, PartialEq)]
pub enum SMError {
    UnexpectedServerValidation, // This is basically a fallback.
    IllegalMove,
    IllegalChunkLoad,
    IllegalBlockBreak,
    IllegalHitPoint,
    IllegalItemOperation,
    InvalidPlayer,
    BannedPlayer,
}

#[derive(Debug, PartialEq)]
pub enum SMValid {
    Yes,
    No(SMError),
}

#[derive(Resource)]
pub struct InternalServer {}

impl InternalServer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn ask() -> SMValid {
        todo!();
    }
}
