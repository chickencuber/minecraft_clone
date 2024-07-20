
use glfw::{Context, Glfw, PWindow, GlfwReceiver};

pub use glfw::{WindowMode, WindowEvent as Event, Key, Action};

trait Unwrap<T, B> {
 fn unwrap_to_option(&self) -> (Option<T>, Option<B>);
}

impl Unwrap<u32, u32> for Option<(u32, u32)> {
    fn unwrap_to_option(&self) -> (Option<u32>, Option<u32>) {
        if let Some(t) = self {
           return (Some(t.0), Some(t.1)); 
        } else {
            return (None, None);
        }
    }
}

pub struct Window {
    pub window_handler: Box<PWindow>,
    pub glfw: Box<Glfw>,
    pub event_handler: Box<GlfwReceiver<(f64, Event)>>,
    startup: fn (&mut Self) -> (),
    update: fn (&mut Self) -> (),
    pub min_size: Option<(u32, u32)>,
    pub max_size: Option<(u32, u32)>,
    on_event: fn(&mut Self, Event) -> (),
}

impl Window {
    pub fn create(width: u16, height: u16, name: &str, mode: WindowMode) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        glfw.window_hint(glfw::WindowHint::DepthBits(Some(24)));

        glfw.window_hint(glfw::WindowHint::Samples(Some(4))); 

        let (mut window, events) = glfw.create_window(width.into(), height.into(), name, mode).expect("Failed to create GLFW window.");
        let (screen_width, screen_height) = window.get_framebuffer_size();

        window.make_current();
        window.set_key_polling(true);
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        unsafe {
            gl::Viewport(0, 0, screen_width, screen_height);
            gl::ClearColor(0.2, 0.3, 0.4, 1.0);
        }

        return Self {
            window_handler: Box::new(window),
            glfw: Box::new(glfw),
            event_handler: Box::new(events),
            startup: |_window: &mut Self| {},
            update: |_window: &mut Self| {},
            on_event: |_window: &mut Self, _event: Event| {},
            min_size: None,
            max_size: None,
        }
    }
    pub fn set_min_size(&mut self, width: u32, height: u32) {
       self.min_size = Some((width, height));
       let (w, h) = self.max_size.unwrap_to_option();
       self.window_handler.set_size_limits(Some(width), Some(height), w, h);
    }
    pub fn set_max_size(&mut self, width: u32, height: u32) {
        self.max_size = Some((width, height));
        let (w, h) = self.min_size.unwrap_to_option();
        self.window_handler.set_size_limits(w, h, Some(width), Some(height));
    }
    pub fn set_size(&mut self, width: u16, height: u16) {
        self.window_handler.set_size(width.into(), height.into());
    }
    pub fn set_name(&mut self, name: &str) {
        self.window_handler.set_title(name);
    }
    pub fn get_size(&self) -> (u16, u16) {
        let (w, h) = self.window_handler.get_size();
        return (w.try_into().unwrap_or(0), h.try_into().unwrap_or(0));
    }
    pub fn get_resolution(&self) -> (u32, u32) {
        let (w, h) =  self.window_handler.get_framebuffer_size();
        return (w.try_into().unwrap_or(0), h.try_into().unwrap_or(0));
    }
    pub fn set_on_event(&mut self, func: fn(&mut Self, event: Event) -> ()) {
        self.on_event = func;
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
            // Poll events
            self.glfw.poll_events();

            // Collect events to a temporary vector
            let events: Vec<(f64, Event)> = glfw::flush_messages(&*self.event_handler).collect();

            // Handle events with immutable access to the event handler
            for (_, event) in events {
                (self.on_event)(self, event);
            }

            // Render loop
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            (self.update)(self);

            self.window_handler.swap_buffers();
        }
    }
}

