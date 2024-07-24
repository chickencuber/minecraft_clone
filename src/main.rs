mod graphics;
use graphics::{*, draw::*};

const DEV: bool = true;

struct GameData {
    
}

enum Textures {
    GrassBlockTop,
}

impl Textures {
    pub fn get(&self) -> &str {
        match self {
            Self::GrassBlockTop => "grass-block-top",
        }
    }
}

fn main() {
    let mut window = Window::create(640, 320, "minecraft_clone", WindowMode::Windowed, GameData {
       
    });
    window.set_min_size(640, 320);
    window.set_max_fps(60.0);
    window.set_update(update);
    window.set_startup(start); 
    window.set_render(render);
    window.set_on_event(on_event);

    window.shaders.set_vertex_shader(files::load_file("./shaders/vertex_shader.glsl", &DEV).unwrap().as_str());
    window.shaders.set_fragment_shader(files::load_file("./shaders/fragment_shader.glsl", &DEV).unwrap().as_str());
    window.shaders.compile_shaders();
   
    window.shaders.reg_texture(Textures::GrassBlockTop.get(), files::load_texture("./textures/grass_block_top.png", &DEV).unwrap());
    window.shaders.build_atlas();

    window.start();
}

fn on_event(window: &mut Window<GameData>, event: Event) {
    
}

fn start(window: &mut Window<GameData>) {
   
}

fn update(window: &mut Window<GameData>) {
    // Update logic
}

fn render(window: &mut Window<GameData>) {
    let triangles: Vec<Triangle> = create_square_triangles(*window.shaders.textures.get(Textures::GrassBlockTop.get()).unwrap());
    window.render_triangles(&triangles);
}

pub fn create_square_triangles(texture: TextureLocation) -> Vec<draw::Triangle> {
    let half_side = 0.1; // Half the length of the side

    let bl = Vec3::new(-half_side, -half_side, 1.0); // Bottom-left
    let br = Vec3::new(half_side, -half_side, 1.0);  // Bottom-right
    let tl = Vec3::new(-half_side, half_side, 1.0);  // Top-left
    let tr = Vec3::new(half_side, half_side, 1.0);   // Top-right

    let mut triangles = Vec::new();
    draw::Triangle::square(&mut triangles, bl, br, tr, tl, &texture);
    triangles
}

