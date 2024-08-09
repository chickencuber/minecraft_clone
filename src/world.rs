use draw::Triangle;

use crate::graphics::{*, draw::*};

macro_rules! block_match {
    ($var:ident, $($name:ident => $body:block),* $(,)?) => {
        $(
            if $var.$name $body
         )*
    };
}

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
        this.chunks.push(vec![core::array::from_fn(|_| Chunk::new(&this))]);
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
                        y.add_block(offset, &b.block_type); 
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
    pub fn render<T>(&self, vert: &mut Vec<f32>, player: Vec3, window: &Window<T>) {
        let (chunk, _) = self.get_chunk(player); 
        if let Some(x) = self.chunks.get(chunk.x as usize) {
            if let Some(z) = x.get(chunk.z as usize) {
                if let Some(y) = z.get(chunk.y as usize) {
                    let blocks = y.get_mesh_data(true);
                    for block in blocks.iter() {
                       let pos = chunk.mul(Vec3::new(16.0, 16.0, 16.0)).add(block.pos); 
                       block_match! {
                           block,
                           top => {
                               self.render_side(vert, Side::Top, pos, block.model_data.clone(), window);
                           },
                           bottom => {
                               self.render_side(vert, Side::Bottom, pos, block.model_data.clone(), window);
                           },
                           left => {
                               self.render_side(vert, Side::Left, pos, block.model_data.clone(), window);
                           },
                           right => {
                               self.render_side(vert, Side::Right, pos, block.model_data.clone(), window);
                           },
                           front => {
                               self.render_side(vert, Side::Front, pos, block.model_data.clone(), window);
                           },
                           back => {
                               self.render_side(vert, Side::Back, pos, block.model_data.clone(), window);
                           },
                       }
                    }
                }
            }
        } 
    }
    fn render_side<T>(&self, vert: &mut Vec<f32>, face: Side, pos: Vec3, model_data: ModelType, window: &Window<T>) {
        if let ModelType::Block(block) = model_data {
            let data = generate_face_vertices(pos, &face); 
            Triangle::create_square(vert, data[0], data[1], data[2], data[3], &window.shaders.get_texture(
                    block.get_texture(&face)
            ));
        }
    }
}

macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
       Vec3::new($x, $y, $z) 
    };
}

fn generate_face_vertices(position: Vec3, face: &Side) -> [Vec3; 4] {
    match face {
        Side::Top => [
            // Top face
            vec3![position.x - 0.5, position.y + 0.5, position.z - 0.5], // Top-left
            vec3![position.x + 0.5, position.y + 0.5, position.z - 0.5], // Top-right
            vec3![position.x + 0.5, position.y + 0.5, position.z + 0.5], // Bottom-right
            vec3![position.x - 0.5, position.y + 0.5, position.z + 0.5], // Bottom-left
        ],
        Side::Bottom => [
            // Bottom face
            vec3![position.x - 0.5, position.y - 0.5, position.z - 0.5], // Bottom-left
            vec3![position.x + 0.5, position.y - 0.5, position.z - 0.5], // Bottom-right
            vec3![position.x + 0.5, position.y - 0.5, position.z + 0.5], // Top-right
            vec3![position.x - 0.5, position.y - 0.5, position.z + 0.5], // Top-left
        ],
        Side::Front => [
            // Front face
            vec3![position.x - 0.5, position.y - 0.5, position.z - 0.5], // Bottom-left
            vec3![position.x + 0.5, position.y - 0.5, position.z - 0.5], // Bottom-right
            vec3![position.x + 0.5, position.y + 0.5, position.z - 0.5], // Top-right
            vec3![position.x - 0.5, position.y + 0.5, position.z - 0.5], // Top-left
        ],
        Side::Back => [
            // Back face
            vec3![position.x - 0.5, position.y - 0.5, position.z + 0.5], // Bottom-right
            vec3![position.x + 0.5, position.y - 0.5, position.z + 0.5], // Bottom-left
            vec3![position.x + 0.5, position.y + 0.5, position.z + 0.5], // Top-left
            vec3![position.x - 0.5, position.y + 0.5, position.z + 0.5], // Top-right
        ],
        Side::Left => [
            // Left face
            vec3![position.x - 0.5, position.y - 0.5, position.z - 0.5], // Bottom-left
            vec3![position.x - 0.5, position.y - 0.5, position.z + 0.5], // Top-left
            vec3![position.x - 0.5, position.y + 0.5, position.z + 0.5], // Top-right
            vec3![position.x - 0.5, position.y + 0.5, position.z - 0.5], // Bottom-right
        ],
        Side::Right => [
            // Right face
            vec3![position.x + 0.5, position.y - 0.5, position.z - 0.5], // Bottom-right
            vec3![position.x + 0.5, position.y - 0.5, position.z + 0.5], // Top-right
            vec3![position.x + 0.5, position.y + 0.5, position.z + 0.5], // Top-left
            vec3![position.x + 0.5, position.y + 0.5, position.z - 0.5], // Bottom-left
        ],
    }
}

#[derive(Clone)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

#[derive(Clone)]
struct Chunk {
    solid: [[[bool; 16]; 16]; 16],

    transparent: [[[bool; 16]; 16]; 16],

    blocks: Vec<Vec<Vec<Block>>>,
}

fn default(world: &World) -> Vec<Vec<Vec<Block>>> {
    let mut vec = Vec::new();
    for x in 0..16 {
        vec.push(Vec::new());
        for y in 0..16 {
            vec[x].push(Vec::new());
            for _ in 0..16 {
                vec[x][y].push(Block::new(0, NbtBlock::new(), world));
            }
        }
    }
    return vec;
}

impl Chunk {
    pub fn new(world: &World) -> Self {
        return Self {
            solid: [[[false; 16]; 16]; 16], // xyz

            transparent: [[[false; 16]; 16]; 16], // xyz

            blocks: default(world),
        }
    }
    pub fn add_block(&mut self, pos: Vec3, block: &BlockType) {
        match block {
            BlockType::None => {
                self.solid[pos.x as usize][pos.y as usize][pos.z as usize] = false;

                self.transparent[pos.x as usize][pos.y as usize][pos.z as usize] = false;
            }
            BlockType::Solid => {
                self.solid[pos.x as usize][pos.y as usize][pos.z as usize] = true;

                self.transparent[pos.x as usize][pos.y as usize][pos.z as usize] = true;
            }
            BlockType::Transparent => {
                self.transparent[pos.x as usize][pos.y as usize][pos.z as usize] = true;
            }
        }
    }
    pub fn get_mesh_data(&self, solid: bool) -> Vec<BlockFaces> {
        let mut vec = Vec::new(); 
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if solid {
                        if self.blocks[x][y][z].solid {
                            vec.push(BlockFaces {
                                pos: Vec3::new(x as f32, y as f32, z as f32),
                                model_data: self.blocks[x][y][z].model_data.clone(),
                                top: true,
                                bottom: true,
                                front: true,
                                back: true,
                                left: true,
                                right: true,
                            });          
                        }
                    } else {
                        if self.blocks[x][y][z].transparent {
                            vec.push(BlockFaces {
                                pos:Vec3::new(x as f32, y as f32, z as f32),
                                model_data: self.blocks[x][y][z].model_data.clone(),
                                top: true,
                                bottom: true,
                                front: true,
                                back: true,
                                left: true,
                                right: true,
                            })
                        }
                    }
                }
            }
        }
        return vec;
    }
}

struct BlockFaces {
    pos: Vec3,
    model_data: ModelType,
    top: bool,
    bottom: bool,
    front: bool, // +x
    back: bool, // -x
    left: bool, // -z
    right: bool, // +z
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
    solid: bool,
    transparent: bool,
}

impl Block {
    pub fn new(id: u64, nbt: NbtBlock, world: &World) -> Self {
        let block_type = &world.blocks.get(id as usize).unwrap().block_type;
        return Self {
            nbt,
            id,
            model_data: world.blocks.get(id as usize).unwrap().model.clone(),
            collision_data: world.blocks.get(id as usize).unwrap().collision_data.clone(),
            solid: *block_type == BlockType::Solid,
            transparent: *block_type != BlockType::None,
        }
    }
    pub fn get_id(&self) -> u64 {
        return self.id;
    }
}

#[derive(Clone, PartialEq)]
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
    Plant(Box<dyn TextureName>),
}

#[derive(Clone)]
pub struct BlockModelType {
    pub block_size: (f32, f32, f32),
    pub texture: BlockTextureType,
}

impl BlockModelType {
    pub fn get_texture(&self, side: &Side) -> String {
        match &self.texture {
            BlockTextureType::None => {panic!("cant render None")},
            BlockTextureType::All(t) => t.get_texture_name(),
            BlockTextureType::Log(t) => {
                match side {
                    Side::Top => t.top.get_texture_name(),
                    Side::Bottom => t.bottom.get_texture_name(),
                    _ => t.side.get_texture_name(),
                }
            },
            BlockTextureType::Each(t) => {
                match side {
                    Side::Top => t.top.get_texture_name(),
                    Side::Bottom => t.bottom.get_texture_name(),
                    Side::Left => t.left.get_texture_name(),
                    Side::Right => t.right.get_texture_name(),
                    Side::Front => t.front.get_texture_name(),
                    Side::Back => t.back.get_texture_name(),
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct BlockCollision {
    pub offset: Vec3,
    pub size: (f32, f32, f32),
}

#[derive(Clone)]
pub enum CollisionData {
    None,
    Interact(f32, f32, f32),
    Normal(f32, f32, f32),
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

