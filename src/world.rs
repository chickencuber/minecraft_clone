use crate::graphics::{*, draw::*};

pub struct World {
    chunks: Vec<Vec<[Chunk; 16]>>,
    pub blocks: Vec<BlockData>,
}

impl World {
    pub fn new() -> Self {
        let mut this = Self {
            chunks: vec![vec![[Chunk::new(); 16]]],
            blocks: Vec::new(),
        };
        this.reg_block(BlockData {
            model: ModelType::Block(BlockModelType {
                block_size: (1.0, 1.0, 1.0),
                texture: BlockTextureType::None,
            }),
            rotate: false,
            collision_size: (1.0, 1.0, 1.0),
            name: "air".to_string(),
            random_tick: None,
            tick: None,
            update: None,
            start: None,
            block_type: BlockType::None,
        });
        return this;
    }
    pub fn reg_block(&mut self, data: BlockData) {
        self.blocks.push(data)
    }
    pub fn place_block(&mut self, vec: Vec3, block: Block) {
        let (chunk, offset) = self.get_chunk(vec);
        if let Some(x) = self.chunks.get_mut(chunk.x as usize) {
            if let Some(z) = x.get_mut(chunk.z as usize) {
                if let Some(y) = z.get_mut(chunk.y as usize) {
                    y.blocks[offset.x as usize][offset.y as usize][offset.z as usize] = block;
                }
            }
        } 
    }
    fn get_chunk(&self, vec: Vec3) -> (Vec3, Vec3) {
        let chunk_size = Vec3::new(16.0, 16.0, 16.0);
        let chunk = vec.div(chunk_size).floor();
        let offset = vec.rem(chunk_size).round();
        return (chunk, offset);
    }
    pub fn get_block(&self, vec: Vec3) -> Block {
        let (chunk, offset) = self.get_chunk(vec);
        if let Some(x) = self.chunks.get(chunk.x as usize) {
            if let Some(z) = x.get(chunk.z as usize) {
                if let Some(y) = z.get(chunk.y as usize) {
                    return y.blocks[offset.x as usize][offset.y as usize][offset.z as usize];
                }
            }
        } 
        return Block::new(0, NbtBlock::new());
    }
    pub fn render(&self, vert: &mut Vec<f32>, player: Vec3) {
        let (chunk, _) = self.get_chunk(player); 
        if let Some(x) = self.chunks.get(chunk.x as usize) {
            if let Some(z) = x.get(chunk.z as usize) {
                if let Some(y) = z.get(chunk.y as usize) {
                }
            }
        } 
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

pub enum BlockTextureType{
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

pub enum BlockType {
    None,
    Solid,
    Transparent,
}

pub enum ModelType {
    Block(BlockModelType),
    Custom(Vec<Faces>)
}

pub struct Faces {
    pub points: (Vec3, Vec3, Vec3),
    pub texture: Box<dyn TextureName>,
}

pub struct BlockModelType {
    pub block_size: (f32, f32, f32),
    pub texture: BlockTextureType,
}

pub struct BlockData {
    pub model: ModelType,
    pub rotate: bool,
    pub collision_size: (f32, f32, f32),
    pub name: String,
    pub tick: Option<fn(Vec3, &mut Block, &mut World) -> ()>,
    pub update: Option<fn(Vec3, &mut Block, &mut World) -> ()>,
    pub start: Option<fn(Vec3, &mut Block, &mut World) -> ()>,
    pub random_tick: Option<fn(Vec3, &mut Block, &mut World) -> ()>,
    pub block_type: BlockType,
}

