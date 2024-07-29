use crate::{TextureLocation, TextureName};
use crate::graphics::Vec3;

pub struct World {
    chunks: Vec<Vec<Chunk>>,
    blocks: Vec<BlockData>,
}

impl World {
    pub fn new() -> Self {
        let mut this = Self {
            chunks: Vec::new(),
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
        this.chunks.push(vec![Chunk::new()]);
        return this;
    }
    pub fn reg_block(&mut self, data: BlockData) {
        self.blocks.push(data)
    }
}

pub struct Chunk {
    y_chunks: [YChunks; 16],
}

impl Chunk {
    pub fn new() -> Self {
        let y: [YChunks; 16] = [YChunks::new(); 16];
        return Self {
            y_chunks: y, 
        } 
    }
}

#[derive(Clone, Copy)]
struct YChunks {
    x_s: [[u16; 16]; 16],
    y_s: [[u16; 16]; 16],
    z_s: [[u16; 16]; 16],

    x_t: [[u16; 16]; 16],
    y_t: [[u16; 16]; 16],
    z_t: [[u16; 16]; 16],

    blocks: [[[(u64, NbtBlock); 16]; 16]; 16],
}

impl YChunks {
    pub fn new() -> Self {
        return Self {
            x_s: [[0; 16]; 16],
            y_s: [[0; 16]; 16],
            z_s: [[0; 16]; 16],

            x_t: [[0; 16]; 16],
            y_t: [[0; 16]; 16],
            z_t: [[0; 16]; 16],

            blocks: [[[(0, NbtBlock::new()); 16]; 16]; 16],
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

pub struct BlockData {
    pub texture: TextureType,
    pub rotate: bool,
    pub size: (f32, f32, f32),
    pub name: String,
    pub tick: Option<fn(Vec3, &mut World) -> ()>,
    pub update: Option<fn(Vec3, &mut World) -> ()>,
    pub start: Option<fn(Vec3, &mut World) -> ()>,
    pub random_tick: Option<fn(Vec3, &mut World) -> ()>,
}

