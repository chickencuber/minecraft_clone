mod graphics;
use graphics::*;

fn main() {
    let mut window = Window::create(640, 320, "minecraft_clone", WindowMode::Windowed);
    window.set_min_size(640, 320);
    window.set_update(update);
    window.set_startup(start);
    window.start();
}

fn update(window: &mut Window) {

}

fn start(window: &mut Window) {

}

