#![allow(dead_code)]

// Blocks for the game.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlockType {
    Air, // Fun fact: the air block never exists in the chunk block array. It is basically an empty block.
    Debug,
}

impl BlockType {
    pub fn properties(&self) -> BlockProperties {
        return match self {
            BlockType::Debug => BlockProperties { ..BlockProperties::default() },

            BlockType::Air => BlockProperties {
                name: "Air".into(),
                collision: BlockCollisionType::Gas,
                textures: [None; 6],
                transparent: true,
                ..BlockProperties::default()
            },
        };
    }
}

// Properties for a block. This is returned in the BlockType.properties() function.
pub struct BlockProperties {
    pub name: String, // This is the name of the block.
    pub collision: BlockCollisionType, // Solid? Liquid? Gas?
    pub transparent: bool, // Are any of the textures transparent? This is used in mesh generation.
    pub textures: [Option<usize>; 6], // A list of textures for the block. (Index of block texture array).
}

impl Default for BlockProperties {
    fn default() -> Self {
        // Note that the default values are what is used for the Debug Block.
        return Self {
            name: "Debug Block".into(),
            collision: BlockCollisionType::Solid,
            transparent: false,
            textures: [Some(0); 6],
        };
    }
}

// Collision type for a block.
pub enum BlockCollisionType {
    Solid,
    Liquid,
    Gas,
}
