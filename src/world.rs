use crate::graphics::{*, draw::*};

pub struct World {
    chunks: Vec<Vec<[Chunk; 16]>>,
    pub blocks: Vec<BlockData>,
}

impl World {
    pub fn new() -> Self {
        let mut this = Self {
            chunks: Vec::new(),
            blocks: Vec::new(),
        };
        this.chunks.push(vec![core::array::from_fn(|_| Chunk::new(&this))]);
        this.reg_block(BlockData {
            model: ModelType::Block(BlockModelType {
                block_size: (1.0, 1.0, 1.0),
                texture: BlockTextureType::None,
            }),
            rotate: false,
            collision_data: CollisionData::None,
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
                    if let Some(b) = self.blocks.get(block.id as usize) {
                        match b.block_type {
                            BlockType::None => {},
                            BlockType::Solid => {
                                
                            },
                            BlockType::Transparent => {

                            }
                        }
                    }
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
    pub fn get_block(&mut self, vec: Vec3) -> Option<&mut Block> {
        let (chunk, offset) = self.get_chunk(vec);
        if let Some(x) = self.chunks.get_mut(chunk.x as usize) {
            if let Some(z) = x.get_mut(chunk.z as usize) {
                if let Some(y) = z.get_mut(chunk.y as usize) {
                    if let Some(a) = y.blocks.get_mut(offset.x as usize) {
                        if let Some(b) = a.get_mut(offset.y as usize) {
                            if let Some(c) = b.get_mut(offset.z as usize) {
                                return Some(c);
                            }
                        }
                    }
                }
            }
        } 
        return None;
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

#[derive(Clone)]
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
    pub fn new(world: &World) -> Self {
        let blocks: [[[Block; 16]; 16]; 16] = core::array::from_fn(|_| {
            return core::array::from_fn(|_| {
                return core::array::from_fn(|_| {
                    return Block::new(0, NbtBlock::new(), world);
                });
            });
        });
        return Self {
            x_s: [[0; 16]; 16], // yzx
            y_s: [[0; 16]; 16], // xzy
            z_s: [[0; 16]; 16], // xyz

            x_t: [[0; 16]; 16], // yzx
            y_t: [[0; 16]; 16], // xzy
            z_t: [[0; 16]; 16], // xyz

            blocks,
        }
    }
    pub fn add_block(&mut self, solid: bool, pos: Vec3) {
        if solid {
            
        } else {

        }
    }
    pub fn get_mesh_data(&self) {
        
    }
}

#[derive(Clone)]
pub struct NbtBlock {
    
}

impl NbtBlock {
    pub fn new() -> Self {
        return Self {

        };
    }
}

#[derive(Clone)]
pub struct NbtEntity {

}

impl NbtEntity {
    pub fn new() -> Self {
        return Self {

        };
    }
}

#[derive(Clone)]
pub struct TextureMap {
    pub top: Box<dyn TextureName>,
    pub bottom: Box<dyn TextureName>,
    pub left: Box<dyn TextureName>,
    pub right: Box<dyn TextureName>,
    pub front: Box<dyn TextureName>,
    pub back: Box<dyn TextureName>,
}

#[derive(Clone)]
pub struct LogTextureMap {
    pub top: Box<dyn TextureName>,
    pub side: Box<dyn TextureName>,
    pub bottom: Box<dyn TextureName>,
}

#[derive(Clone)]
pub enum BlockTextureType{
    None,
    All(Box<dyn TextureName>),
    Each(TextureMap),
    Log(LogTextureMap)
}

#[derive(Clone)]
pub struct Block {
    pub nbt: NbtBlock,
    id: u64,
    pub model_data: ModelType,
    pub collision_data: CollisionData,
}

impl Block {
    pub fn new(id: u64, nbt: NbtBlock, world: &World) -> Self {
        return Self {
            nbt,
            id,
            model_data: world.blocks.get(id as usize).unwrap().model.clone(),
            collision_data: world.blocks.get(id as usize).unwrap().collision_data.clone(),
        }
    }
    pub fn get_id(&self) -> u64 {
        return self.id;
    }
}

#[derive(Clone)]
pub enum BlockType {
    None,
    Solid,
    Transparent,
}

impl Clone for Box<dyn TextureName> {
    fn clone(&self) -> Self {
        return Box::new(self.get_texture_name());
    }
}

#[derive(Clone)]
pub enum ModelType {
    Block(BlockModelType),
    Custom(Vec<Faces>),
    Plant(Box<dyn TextureName>),
}

#[derive(Clone)]
pub struct Faces {
    pub points: (Vec3, Vec3, Vec3),
    pub texture: Box<dyn TextureName>,
}

#[derive(Clone)]
pub struct BlockModelType {
    pub block_size: (f32, f32, f32),
    pub texture: BlockTextureType,
}

#[derive(Clone)]
pub enum CollisionType {
    Block(f32, f32, f32),
    Custom(Vec<BlockCollision>),
}

#[derive(Clone)]
pub struct BlockCollision {
    pub offset: Vec3,
    pub size: (f32, f32, f32),
}

#[derive(Clone)]
pub enum CollisionData {
    None,
    Interact(CollisionType),
    Normal(CollisionType),
}

#[derive(Clone)]
pub struct BlockData {
    pub model: ModelType,
    pub rotate: bool,
    pub collision_data: CollisionData,
    pub name: String,
    pub tick: Option<fn(Vec3, &mut Block, &mut World) -> ()>,
    pub update: Option<fn(Vec3, &mut Block, &mut World) -> ()>,
    pub start: Option<fn(Vec3, &mut Block, &mut World) -> ()>,
    pub random_tick: Option<fn(Vec3, &mut Block, &mut World) -> ()>,
    pub block_type: BlockType,
}

