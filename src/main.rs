mod graphics;
use graphics::*;

const DEV: bool = true;

struct GameData {

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

    window.start();
}

fn on_event(window: &mut Window<GameData>, event: Event) {
    
}

fn start(window: &mut Window<GameData>) {
    
}

fn update(window: &mut Window<GameData>) {
    println!("{}", window.fps)
}

fn render(window: &mut Window<GameData>) {

}

