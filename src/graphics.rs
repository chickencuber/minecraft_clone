
use glfw::{Action, Context, Glfw, Key, PWindow, WindowEvent, GlfwReceiver};

pub use glfw::WindowMode;

pub struct Window {
    pub window_handler: Box<PWindow>,
    pub glfw: Box<Glfw>,
    pub event_handler: Box<GlfwReceiver<(f64, WindowEvent)>>,
    startup: fn (&mut Self) -> (),
    update: fn (&mut Self) -> (),
}

impl Window {
    pub fn create(width: u32, height: u32, name: &str, mode: WindowMode) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let (mut window, events) = glfw.create_window(width, height, name, mode).expect("Failed to create GLFW window.");
        window.make_current();
        window.set_key_polling(true);
        gl::load_with(|s| window.get_proc_address(s) as *const _);
        return Self {
            window_handler: Box::new(window),
            glfw: Box::new(glfw),
            event_handler: Box::new(events),
            startup: |_window: &mut Self| {},
            update: |_window: &mut Self| {},
        }
    }
    pub fn set_startup(&mut self, func: fn(&mut Self) -> ()) {
        self.startup = func;
    }
    pub fn set_update(&mut self, func: fn(&mut Self) -> ()) {
        self.update = func;
    }
    pub fn start(&mut self) {
        (self.startup)(self);
        while !self.window_handler.should_close() {
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&*self.event_handler) {

            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.window_handler.swap_buffers();
        }
    }
}

