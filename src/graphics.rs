use glfw::{Context, Glfw, PWindow, GlfwReceiver};
pub use glfw::{WindowMode, WindowEvent as Event, Key, Action};
use std::{time::{Instant, Duration}, thread::sleep};

pub mod files {
    use std::{fs, io::prelude::Read, env::current_exe};

    fn file_name(name: &str, dev: &bool) -> Result<String, std::io::Error> {
        let mut exe = current_exe()?; 
        exe.pop();
        if *dev {
            exe.pop();
            exe.pop();
            exe.push("src");
        }
        exe.push(name);
        return Ok(exe.to_str().unwrap().to_string());
    }

    pub fn load_file(filename: &str, dev: &bool) -> Result<String, std::io::Error> {
        let mut file = fs::File::open(file_name(filename, dev)?)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}


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

pub struct Shaders {
    vertex_shader_code: Option<String>,
    fragment_shader_code: Option<String>,
    geometry_shader_code: Option<String>,
    tess_control_shader_code: Option<String>,
    tess_eval_shader_code: Option<String>,
    compute_shader_code: Option<String>,
    program: u32,
}

impl Shaders {
    pub fn new() -> Self {
        Self {
            vertex_shader_code: None,
            fragment_shader_code: None,
            geometry_shader_code: None,
            tess_control_shader_code: None,
            tess_eval_shader_code: None,
            compute_shader_code: None,
            program: 0,
        }
    }

    pub fn set_vertex_shader(&mut self, code: &str) {
        self.vertex_shader_code = Some(code.to_string());
    }

    pub fn set_fragment_shader(&mut self, code: &str) {
        self.fragment_shader_code = Some(code.to_string());
    }

    pub fn set_geometry_shader(&mut self, code: &str) {
        self.geometry_shader_code = Some(code.to_string());
    }

    pub fn set_tess_control_shader(&mut self, code: &str) {
        self.tess_control_shader_code = Some(code.to_string());
    }

    pub fn set_tess_eval_shader(&mut self, code: &str) {
        self.tess_eval_shader_code = Some(code.to_string());
    }

    pub fn set_compute_shader(&mut self, code: &str) {
        self.compute_shader_code = Some(code.to_string());
    }

    pub fn compile_shaders(&mut self) {
        unsafe {
            let mut shaders = vec![];

            if let Some(ref code) = self.vertex_shader_code {
                shaders.push(self.compile_shader(code, gl::VERTEX_SHADER));
            }
            if let Some(ref code) = self.fragment_shader_code {
                shaders.push(self.compile_shader(code, gl::FRAGMENT_SHADER));
            }
            if let Some(ref code) = self.geometry_shader_code {
                shaders.push(self.compile_shader(code, gl::GEOMETRY_SHADER));
            }
            if let Some(ref code) = self.tess_control_shader_code {
                shaders.push(self.compile_shader(code, gl::TESS_CONTROL_SHADER));
            }
            if let Some(ref code) = self.tess_eval_shader_code {
                shaders.push(self.compile_shader(code, gl::TESS_EVALUATION_SHADER));
            }
            if let Some(ref code) = self.compute_shader_code {
                shaders.push(self.compile_shader(code, gl::COMPUTE_SHADER));
            }

            self.program = gl::CreateProgram();

            for shader in &shaders {
                gl::AttachShader(self.program, *shader);
            }

            gl::LinkProgram(self.program);

            let mut success = gl::FALSE as i32;
            gl::GetProgramiv(self.program, gl::LINK_STATUS, &mut success);
            if success == gl::FALSE as i32 {
                let mut len = 0;
                gl::GetProgramiv(self.program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len - 1) as usize);
                gl::GetProgramInfoLog(
                    self.program,
                    len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut gl::types::GLchar,
                );
                eprintln!("Program link error: {:?}", String::from_utf8(buf).unwrap());
            }

            for shader in shaders {
                gl::DeleteShader(shader);
            }
        }
    }

    fn compile_shader(&self, source: &str, shader_type: u32) -> u32 {
        unsafe {
            let shader = gl::CreateShader(shader_type);
            let c_source = std::ffi::CString::new(source).unwrap();
            gl::ShaderSource(shader, 1, &c_source.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            let mut success = gl::FALSE as i32;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == gl::FALSE as i32 {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len - 1) as usize);
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut gl::types::GLchar,
                );
                eprintln!("Shader compile error: {:?}", String::from_utf8(buf).unwrap());
            }

            shader
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}

pub struct Window<Data> {
    pub window_handler: Box<PWindow>,
    pub glfw: Box<Glfw>,
    pub event_handler: Box<GlfwReceiver<(f64, Event)>>,
    startup: fn (&mut Self) -> (),
    update: fn (&mut Self) -> (),
    pub min_size: Option<(u32, u32)>,
    pub max_size: Option<(u32, u32)>,
    on_event: fn(&mut Self, Event) -> (),
    render: fn(&mut Self) -> (),
    pub shaders: Shaders,
    pub data: Data,
    pub fps: f64,
    pub deltatime: f64,
    pub frame_count: u64,
    pub max_fps: Option<f64>,
}

impl<Data> Window<Data> {
    pub fn create (width: u16, height: u16, name: &str, mode: WindowMode, data: Data) -> Self {
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
            startup: |_window| {},
            update: |_window| {},
            on_event: |_window, _event| {},
            render: |_window| {},
            min_size: None,
            max_size: None,
            shaders: Shaders::new(),
            data,
            fps: 0.0,
            deltatime: 0.0,
            frame_count: 0,
            max_fps: None,
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
    pub fn set_max_fps(&mut self, fps: f64) {
        self.max_fps = Some(fps);
    }
    pub fn set_on_event(&mut self, func: fn(&mut Self, Event) -> ()) {
        self.on_event = func;
    }
    pub fn set_render(&mut self, func: fn(&mut Self) -> ()) {
        self.render = func;
    }
    pub fn set_startup(&mut self, func: fn(&mut Self) -> ()) {
        self.startup = func;
    }
    pub fn set_update(&mut self, func: fn(&mut Self) -> ()) {
        self.update = func;
    }
    pub fn start(&mut self) {
        (self.startup)(self);
        
        let mut last_time = Instant::now();

        while !self.window_handler.should_close() {
            // Poll events
            self.glfw.poll_events();

            // Collect events to a temporary vector
            let events: Vec<(f64, Event)> = glfw::flush_messages(&*self.event_handler).collect();

            // Handle events with immutable access to the event handler
            for (_, event) in events {
                (self.on_event)(self, event);
            }

            (self.update)(self);

            // Render loop
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            (self.render)(self);

            self.window_handler.swap_buffers();

            if let Some(target_fps) = self.max_fps {
                let target_frame_duration = Duration::from_secs_f64(1.0 / target_fps);
                let frame_time = Instant::now().duration_since(last_time);

                if frame_time < target_frame_duration {
                    sleep(target_frame_duration - frame_time);
                }
            }

            self.frame_count = self.frame_count.wrapping_add(1);
            let current = Instant::now(); 
            self.deltatime = current.duration_since(last_time).as_secs_f64();
            last_time = current;
            self.fps = 1.0 / self.deltatime;
        }
    }
}

