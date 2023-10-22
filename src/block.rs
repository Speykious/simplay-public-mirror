#![allow(dead_code)]

use hashbrown::HashMap;
use crate::world;

// Blocks for the game.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlockType {
    Air, // Fun fact: the air block never exists in the chunk block array. It is basically an empty block.
    Debug,
    Grass,
    Dirt,
    Stone,
}

impl BlockType {
    pub fn properties(&self) -> BlockProperties {
        return match self {
            BlockType::Debug => BlockProperties { ..BlockProperties::default() },

            BlockType::Air => BlockProperties {
                name: "Air".into(),
                collision: BlockCollisionType::Gas,
                textures: BlockTextures::blank(),
                transparent: true,
                ..BlockProperties::default()
            },

            BlockType::Grass => BlockProperties {
                name: "Grass Block".into(),
                collision: BlockCollisionType::Solid,
                textures: BlockTextures::new([
                    Some("grass_side"),
                    Some("grass_side"),
                    Some("grass_side"),
                    Some("grass_side"),
                    Some("grass_top"),
                    Some("dirt"),
                ]),
                transparent: false,
                ..BlockProperties::default()
            },

            BlockType::Dirt => BlockProperties {
                name: "Dirt Block".into(),
                collision: BlockCollisionType::Solid,
                textures: BlockTextures::new([
                    Some("dirt"); 6
                ]),
                transparent: false,
                ..BlockProperties::default()
            },

            BlockType::Stone => BlockProperties {
                name: "Stone Block".into(),
                collision: BlockCollisionType::Solid,
                textures: BlockTextures::new([
                    Some("stone"); 6
                ]),
                transparent: false,
                ..BlockProperties::default()
            },
        };
    }
}

// Block textures struct.
pub struct BlockTextures {
    textures: HashMap<world::Direction, Option<String>>,
}

impl BlockTextures {
    /// Texture order: [North, South, East, West, Up, Down]
    pub fn new(textures: [Option<&str>; 6]) -> Self {
        let mut texture_map: HashMap<world::Direction, Option<String>> = HashMap::new();

        for (i, d) in world::Direction::all().iter().enumerate() {
            texture_map.insert(*d, match textures[i] { Some(s) => Some(s.to_string()), None => None });
        }

        return Self {
            textures: texture_map,
        };
    }

    pub fn blank() -> Self {
        return Self::new([None; 6]);
    }

    pub fn get(&self, direction: world::Direction) -> Option<String> {
        return self.textures.get(&direction).unwrap().clone();
    }
}

// Properties for a block. This is returned in the BlockType.properties() function.
pub struct BlockProperties {
    pub name: String, // This is the name of the block.
    pub collision: BlockCollisionType, // Solid? Liquid? Gas?
    pub transparent: bool, // Are any of the textures transparent? This is used in mesh generation.
    pub textures: BlockTextures, // A list of textures for the block. (Index of block texture array).
}

impl Default for BlockProperties {
    fn default() -> Self {
        // Note that the default values are what is used for the Debug Block.
        return Self {
            name: "Debug Block".into(),
            collision: BlockCollisionType::Solid,
            transparent: false,
            textures: BlockTextures::new([Some("debug"); 6]),
        };
    }
}

// Collision type for a block.
pub enum BlockCollisionType {
    Solid,
    Liquid,
    Gas,
}
