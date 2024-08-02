mod graphics;
use graphics::{*, draw::*};
mod world;
use world::*;

use nalgebra_glm as glm;

const DEV: bool = true;

struct GameData {
   player: Player,
   keys: Keys,
   sensitivity: f32,
   world: World,
}

struct Player {
    speed: f32,
}

impl Player {
    pub fn new() -> Self {
        return Self {
            speed: 0.02, 
        }
    }
}

struct Keys {
    w: bool,
    s: bool,
    a: bool,
    d: bool,
    space: bool,
    shift: bool,
}

impl Keys {
    pub fn new() -> Self {
        return Self {
            w: false,
            s: false,
            a: false,
            d: false,
            space: false,
            shift: false,
        }
    }
}

enum Textures {
    GrassBlockTop,
    GrassBlockSide,
    DirtBlock,
}

impl TextureName for Textures {
    fn get_texture_name(&self) -> String {
        match self {
            Self::GrassBlockTop => "grass-block-top",
            Self::GrassBlockSide => "grass-block-side",
            Self::DirtBlock => "dirt-block",
        }.to_string()
    }
}

fn main() {
    let mut window = Window::create(640, 320, "minecraft_clone", WindowMode::Windowed, GameData {
        player: Player::new(),
        keys: Keys::new(),
        sensitivity: 0.003,
        world: World::new(),
    });

    // sets up window

    window.set_min_size(640, 320);
    window.set_max_fps(60.0);
    window.set_update(update);
    window.set_render(render);
    window.set_on_event(on_event);

    // creates shaders

    window.shaders.set_vertex_shader(files::load_file("./shaders/vertex_shader.glsl", &DEV).unwrap().as_str());
    window.shaders.set_fragment_shader(files::load_file("./shaders/fragment_shader.glsl", &DEV).unwrap().as_str());
    window.shaders.compile_shaders();
   
    // creates textures

    window.shaders.reg_texture(Textures::GrassBlockTop, files::load_texture("./textures/grass_block_top.png", &DEV).unwrap());
    window.shaders.reg_texture(Textures::GrassBlockSide, files::load_texture("./textures/grass_block_side.png",&DEV).unwrap());
    window.shaders.reg_texture(Textures::DirtBlock, files::load_texture("./textures/dirt_block.png", &DEV).unwrap());
    window.shaders.build_atlas();

    // creates blocks

    window.data.world.reg_block(BlockData {
        name: "grass block".to_string(),
        collision_size: (1.0, 1.0, 1.0),
        model: ModelType::Block(BlockModelType {
            block_size: (1.0, 1.0, 1.0), 
            texture: BlockTextureType::Log(LogTextureMap {
                top: Box::new(Textures::GrassBlockTop),
                side: Box::new(Textures::GrassBlockSide),
                bottom: Box::new(Textures::DirtBlock),
            }),
        }),
        rotate: false,
        start: None,
        tick: None,
        update: None,
        random_tick: None,
        block_type: BlockType::Solid,
    });

    window.data.world.reg_block(BlockData {
        name: "dirt block".to_string(),
        collision_size: (1.0, 1.0, 1.0),
        model: ModelType::Block(BlockModelType {
            block_size:(1.0, 1.0, 1.0),
            texture: BlockTextureType::All(Box::new(Textures::DirtBlock)),
        }),
        rotate: false,
        tick: None,
        start: None,
        update: None,
        random_tick: None,
        block_type: BlockType::Solid,
    });
    //starts window
   
    window.data.world.place_block(Vec3::new(0.0, 0.0, 0.0), Block::new(1, NbtBlock::new()));

    window.start();
}

fn on_event(window: &mut Window<GameData>, event: Event) {
    match event {
        Event::Key(Key::W, _, action, _) => {
            if action == Action::Repeat {return;}
            window.data.keys.w = action == Action::Press;
        }
        Event::Key(Key::S, _, action, _) => {
            if action == Action::Repeat {return;}
            window.data.keys.s = action == Action::Press;
        }
        Event::Key(Key::A, _, action, _) => {
            if action == Action::Repeat {return;}
            window.data.keys.a = action == Action::Press;
        }
        Event::Key(Key::D, _, action, _) => {
            if action == Action::Repeat {return;}
            window.data.keys.d = action == Action::Press;
        }
        Event::Key(Key::Space, _, action, _) => {
            if action == Action::Repeat {return;}
            window.data.keys.space = action == Action::Press;
        }
        Event::Key(Key::LeftShift, _, action, _) => {
            if action == Action::Repeat {return;}
            window.data.keys.shift = action == Action::Press;
        }
        Event::Key(Key::Escape, _, Action::Press, _) => {
            if window.get_cursor_mode() == CursorMode::Disabled {
                window.set_cursor_mode(CursorMode::Normal);
            } else {
                window.set_cursor_mode(CursorMode::Disabled);
                window.set_cursor_pos(Vec2::new(0.0, 0.0));
            }
        }
        Event::CursorPos(_, _) => {
            if window.get_cursor_mode() == CursorMode::Normal {return;}
            let mut pos = window.get_cursor_pos();
            pos.x *= window.data.sensitivity;
            pos.y *= window.data.sensitivity;
            window.camera.rotation += pos;
            window.set_cursor_pos(Vec2::new(0.0, 0.0));
        }
        _ => {}
    }    
}

fn update(window: &mut Window<GameData>) {
    let mut move_vec = Vec3::new(0.0, 0.0, 0.0);
    if window.data.keys.w {
       move_vec.z += 1.0;
    }
    if window.data.keys.s {
        move_vec.z -= 1.0;
    }
    if window.data.keys.a {
        move_vec.x -= 1.0;
    }
    if window.data.keys.d {
        move_vec.x += 1.0;
    }
    if move_vec != Vec3::new(0.0, 0.0, 0.0) {
        move_vec = move_vec.normalize() * window.data.player.speed;
        rotate(&mut move_vec, &window.camera.rotation); 
        window.camera.pos = window.camera.pos + move_vec;
    }
    if window.data.keys.space {
        window.camera.pos.y += window.data.player.speed;
    }
    if window.data.keys.shift {
        window.camera.pos.y -= window.data.player.speed;
    }
}

fn render(window: &mut Window<GameData>) {
    let mut verts: Vec<f32> = Vec::new();
    window.render_triangles(&verts);
}

fn rotate(vec: &mut Vec3, rotation: &Vec2) {
    let mut view = glm::identity();

    view = glm::rotate_y(&view, rotation.x);
    
    let new_vec = view * glm::vec3_to_vec4(vec);
    *vec = new_vec.xyz();
}

