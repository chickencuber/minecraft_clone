
use glfw::{Action, Context, Glfw, Key, PWindow, WindowEvent, GlfwReceiver};

pub use glfw::WindowMode;

pub struct Window {
    pub window_handler: Box<PWindow>,
    pub glfw: Box<Glfw>,
    pub event_handler: Box<GlfwReceiver<(f64, WindowEvent)>>
}

impl Window {
    pub fn create(width: u32, height: u32, name: &str, mode: WindowMode) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let (mut window, events) = glfw.create_window(width, height, name, mode).expect("Failed to create GLFW window.");
        return Self {
            window_handler: Box::new(window),
            glfw: Box::new(glfw),
            event_handler: Box::new(events),
        }
    }
}

