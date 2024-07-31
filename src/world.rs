use crate::{TextureLocation, TextureName};
use crate::graphics::{Vec3, Ops};

const WORLDSIZE: usize = 255;

pub struct World {
    chunks: [[[Chunk; 16]; WORLDSIZE]; WORLDSIZE],
    blocks: Vec<BlockData>,
}

impl World {
    pub fn new() -> Self {
        let mut this = Self {
            chunks: [[[Chunk::new(); 16]; WORLDSIZE]; WORLDSIZE],
            blocks: Vec::new(),
        };
        this.reg_block(BlockData {
            texture: TextureType::None,
            rotate: false,
            size: (1.0, 1.0, 1.0),
            name: "air".to_string(),
            random_tick: None,
            tick: None,
            update: None,
            start: None,
        });
        return this;
    }
    pub fn reg_block(&mut self, data: BlockData) {
        self.blocks.push(data)
    }
    pub fn place_block(&mut self, vec: Vec3, block: Block) {
        let (mut chunk, offset) = self.get_chunk(vec);
        chunk.blocks[offset.x as usize][offset.y as usize][offset.z as usize] = block;
    }
    fn get_chunk(&self, vec: Vec3) -> (Chunk, Vec3) {
        let chunk_size = Vec3::new(16.0, 16.0, 16.0);

        let chunk = vec.div(chunk_size).floor();
        let offset = vec.rem(chunk_size).round();

        return (self.chunks[chunk.x as usize][chunk.z as usize][chunk.y as usize], offset);
    }
    pub fn get_block(&self, vec: Vec3) -> Block {
        let (chunk, offset) = self.get_chunk(vec);
        return chunk.blocks[offset.x as usize][offset.y as usize][offset.z as usize];
    }
}

#[derive(Clone, Copy)]
struct Chunk {
    x_s: [[u16; 16]; 16],
    y_s: [[u16; 16]; 16],
    z_s: [[u16; 16]; 16],

    x_t: [[u16; 16]; 16],
    y_t: [[u16; 16]; 16],
    z_t: [[u16; 16]; 16],

    blocks: [[[Block; 16]; 16]; 16],
}

impl Chunk {
    pub fn new() -> Self {
        return Self {
            x_s: [[0; 16]; 16],
            y_s: [[0; 16]; 16],
            z_s: [[0; 16]; 16],

            x_t: [[0; 16]; 16],
            y_t: [[0; 16]; 16],
            z_t: [[0; 16]; 16],

            blocks: [[[Block::new(0, NbtBlock::new()); 16]; 16]; 16],
        }
    }
}

#[derive(Clone, Copy)]
pub struct NbtBlock {
    
}

impl NbtBlock {
    pub fn new() -> Self {
        return Self {

        };
    }
}

pub struct NbtEntity {

}

impl NbtEntity {
    pub fn new() -> Self {
        return Self {

        };
    }
}

pub struct TextureMap {
    pub top: Box<dyn TextureName>,
    pub bottom: Box<dyn TextureName>,
    pub left: Box<dyn TextureName>,
    pub right: Box<dyn TextureName>,
    pub front: Box<dyn TextureName>,
    pub back: Box<dyn TextureName>,
}

pub struct LogTextureMap {
    pub top: Box<dyn TextureName>,
    pub side: Box<dyn TextureName>,
    pub bottom: Box<dyn TextureName>,
}

pub enum TextureType{
    None,
    All(Box<dyn TextureName>),
    Each(TextureMap),
    Log(LogTextureMap)
}

#[derive(Clone, Copy)]
pub struct Block {
    pub nbt: NbtBlock,
    id: u64,
}

impl Block {
    pub fn new(id: u64, nbt: NbtBlock) -> Self {
        return Self {
            nbt,
            id,
        }
    }
    pub fn get_id(&self) -> u64 {
        return self.id;
    }
}

pub struct BlockData {
    pub texture: TextureType,
    pub rotate: bool,
    pub size: (f32, f32, f32),
    pub name: String,
    pub tick: Option<fn(Vec3, &mut Block, &mut World) -> ()>,
    pub update: Option<fn(Vec3, &mut Block, &mut World) -> ()>,
    pub start: Option<fn(Vec3, &mut Block, &mut World) -> ()>,
    pub random_tick: Option<fn(Vec3, &mut Block, &mut World) -> ()>,
}

