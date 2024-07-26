use glfw::{Context, Glfw, GlfwReceiver, PWindow};
pub use glfw::{WindowMode, WindowEvent as Event, Key, Action, CursorMode, MouseButton};
use image::{DynamicImage, GenericImageView};
use std::{collections::HashMap, thread::sleep, time::{Duration, Instant}};

use nalgebra_glm::{self as glm, Mat4};
pub use glm::{Vec3, Vec2};

pub mod files {
    use std::{fs, io::prelude::Read, env::current_exe};

    use image::DynamicImage;

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

    pub fn load_texture(filename: &str, dev: &bool) -> Result<DynamicImage, std::io::Error> {
        let path = file_name(filename, dev)?;
        let img = image::open(path).expect("Failed to load texture");

        return Ok(img);
    }
}



pub mod draw {
    use crate::{TextureLocation, TextureMapping};
    
    use super::Vec3;

    pub struct Triangle {
        pub p1: Vec3,
        pub p2: Vec3,
        pub p3: Vec3,

        pub t1: (f32, f32),
        pub t2: (f32, f32),
        pub t3: (f32, f32),
    }

    impl Triangle {
        pub fn new(p1: Vec3, p2: Vec3, p3: Vec3, texture_id: &TextureLocation, t1: TextureMapping, t2: TextureMapping, t3: TextureMapping) -> Self {
            Self {
                p1, p2, p3, t1: t1.get(*texture_id), t2: t2.get(*texture_id), t3: t3.get(*texture_id),
            }
        } 
        pub fn square(vec: &mut Vec<Triangle>, tl: Vec3, tr: Vec3, br: Vec3, bl: Vec3, texture_id: &TextureLocation) {
            vec.push(Triangle::new(tl, tr, br, texture_id, TextureMapping::TopLeft, TextureMapping::TopRight, TextureMapping::BottomRight));
            vec.push(Triangle::new(tl, bl, br, texture_id, TextureMapping::TopLeft, TextureMapping::BottomLeft, TextureMapping::BottomRight));
        }
        pub fn to_points(&self, vec: &mut Vec<f32>) {
            vec.push(self.p1.x);
            vec.push(self.p1.y);
            vec.push(-self.p1.z);
            vec.push(self.t1.0);
            vec.push(self.t1.1);
            vec.push(self.p2.x);
            vec.push(self.p2.y);
            vec.push(-self.p2.z);
            vec.push(self.t2.0);
            vec.push(self.t2.1);
            vec.push(self.p3.x);
            vec.push(self.p3.y);
            vec.push(-self.p3.z);
            vec.push(self.t3.0);
            vec.push(self.t3.1);
        }
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

#[derive(Clone, Copy)]
pub struct TextureLocation {
    tl: (f32, f32),
    tr: (f32, f32),
    bl: (f32, f32),
    br: (f32, f32),
}

pub enum TextureMapping {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl TextureMapping {
    pub fn get(&self, location: TextureLocation) -> (f32, f32) {
        match self {
            Self::BottomLeft => location.bl,
            Self::BottomRight => location.br,
            Self::TopLeft => location.tl,
            Self::TopRight => location.tr,
        }
    }
}

pub trait TextureName {
    fn get_texture_name(&self) -> String;
}

impl TextureName for &str {
    fn get_texture_name(&self) -> String {
        return self.to_string();
    }
}

impl TextureName for String {
    fn get_texture_name(&self) -> String {
        return self.clone();
    }
}

impl TextureName for i32 {
    fn get_texture_name(&self) -> String {
        return self.to_string();
    }
}

impl TextureName for f32 {
    fn get_texture_name(&self) -> String {
        return self.to_string();
    }
}

pub struct Shaders {
    vertex_shader_code: Option<String>,
    fragment_shader_code: Option<String>,
    geometry_shader_code: Option<String>,
    tess_control_shader_code: Option<String>,
    tess_eval_shader_code: Option<String>,
    compute_shader_code: Option<String>,
    pub program: u32,
    pub textures: HashMap<String, TextureLocation>,
    uncompiled_textures: HashMap<String, DynamicImage>,
    pub texture_atlas: u32,
    atlas_built: bool,
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
            textures: HashMap::new(),
            uncompiled_textures: HashMap::new(),
            texture_atlas: 0,
            atlas_built: false,
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
    pub fn set_uniform_matrix(&self, name: &str, matrix: &glm::Mat4) {
        unsafe {
            let location = gl::GetUniformLocation(self.program, std::ffi::CString::new(name).unwrap().as_ptr() as *const i8);
            if location != -1 {
                gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
            } else {
                eprintln!("Uniform {} not found in shader program", name);
            }
        }
    }
    pub fn reg_texture<T: TextureName>(&mut self, name: T, img: DynamicImage) { 
        self.uncompiled_textures.insert(name.get_texture_name(), img);
    }
    pub fn build_atlas(&mut self) {
        let mut img: image::RgbaImage; 
        {
            let mut width = 0;
            let mut height = 0;
            for (_, v) in self.uncompiled_textures.iter() {
                let (w, h) = v.dimensions();
                width += w + 2;
                if h > height {
                    height = h;
                }
            }
            width -= 2;
            height += 1;
            img = image::RgbaImage::new(width, height);

            let mut x_offset = 0;

            // Add each texture to the atlas
            for (key, texture) in self.uncompiled_textures.iter() {
                let (w, h) = texture.dimensions();

                // Copy the texture data into the atlas image
                let texture_data = texture.to_rgba8();
                for y in 0..h {
                    for x in 0..w {
                        img.put_pixel(x + x_offset, y, *texture_data.get_pixel(x, y));
                    }
                }

                // Calculate the texture coordinates in the atlas
                let (atlas_width, atlas_height) = img.dimensions();
                let texture_location = TextureLocation {
                    bl: (x_offset as f32 / atlas_width as f32, 0.0),
                    br: ((x_offset + w) as f32 / atlas_width as f32, 0.0),
                    tl: (x_offset as f32 / atlas_width as f32, h as f32 / atlas_height as f32),
                    tr: ((x_offset + w) as f32 / atlas_width as f32, h as f32 / atlas_height as f32),
                };

                // Store the texture coordinates
                self.textures.insert(key.clone(), texture_location);

                // Update x_offset for the next texture
                x_offset += w + 2;
            }
        }
        let (width, height) = img.dimensions();
        let data = img;

        let mut texture: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);


            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // Upload texture data
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const std::ffi::c_void,
                );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        self.texture_atlas = texture;
        self.atlas_built = true;
    }
    pub fn get_texture<T: TextureName>(&self, name: T) -> TextureLocation {
        let s = name.get_texture_name();
        match self.textures.get(&s) {
            Some(t) => *t,
            None => {
                if !self.atlas_built {
                    panic!("the atlas has not been built");
                }
                panic!("{} is not a texture", s)
            }
        }
    }
}

pub struct Camera {
    pub pos: Vec3,
    pub rotation: Vec2,
}

impl Camera {
    pub fn new() -> Self{
       return Self {
           pos: Vec3::new(0.0, 0.0, 0.0),
           rotation: Vec2::new(0.0, 0.0),
       } 
    }
    pub fn to_matrix(&self) -> Mat4 {
        let mut view = glm::identity();
        let mut pos = self.pos.clone();
        pos.z *= -1.0;
        
        view = glm::rotate_x(&view, self.rotation.y);

        view = glm::rotate_y(&view, self.rotation.x);
        
        view = glm::translate(&view, &-pos);

        return view;
    }
}

pub struct Window<Data> {
    pub window_handler: Box<PWindow>,
    pub glfw: Box<Glfw>,
    pub event_handler: Box<GlfwReceiver<(f64, Event)>>,
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
    pub camera: Camera,
}

impl<Data> Window<Data> {
    pub fn close(&mut self) {
        self.window_handler.set_should_close(true);
    }
    pub fn set_cursor_mode(&mut self, mode: CursorMode) {
        self.window_handler.set_cursor_mode(mode);
    }
    pub fn get_cursor_mode(&self) -> CursorMode {
        return self.window_handler.get_cursor_mode();
    }
    pub fn set_raw_mouse_motion(&mut self, b: bool) {
        self.window_handler.set_raw_mouse_motion(b);
    }
    pub fn get_raw_mouse_motion(&self) -> bool {
        return self.window_handler.uses_raw_mouse_motion();
    }
    pub fn get_cursor_pos(&self) -> Vec2 {
        let (x, y) = self.window_handler.get_cursor_pos();
        return Vec2::new(x as f32, y as f32);
    }
    pub fn set_cursor_pos(&mut self, pos: Vec2) {
        self.window_handler.set_cursor_pos(pos.x as f64, pos.y as f64);
    }
    pub fn create (width: u16, height: u16, name: &str, mode: WindowMode, data: Data) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        glfw.window_hint(glfw::WindowHint::DepthBits(Some(24)));

        glfw.window_hint(glfw::WindowHint::Samples(Some(4))); 

        let (mut window, events) = glfw.create_window(width.into(), height.into(), name, mode).expect("Failed to create GLFW window.");

        window.make_current();
        window.set_all_polling(true);
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        let (screen_width, screen_height) = window.get_framebuffer_size();

        unsafe {
            gl::Viewport(0, 0, screen_width, screen_height);
            gl::ClearColor(0.2, 0.3, 0.4, 1.0);
        }

        return Self {
            window_handler: Box::new(window),
            glfw: Box::new(glfw),
            event_handler: Box::new(events),
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
            camera: Camera::new(),
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
    pub fn set_update(&mut self, func: fn(&mut Self) -> ()) {
        self.update = func;
    }
    pub fn start(&mut self) {
        let mut last_time = Instant::now();

        while !self.window_handler.should_close() {
            // Poll events
            self.glfw.poll_events();

            // Collect events to a temporary vector
            let events: Vec<(f64, Event)> = glfw::flush_messages(&*self.event_handler).collect();

            // Handle events with immutable access to the event handler
            for (_, event) in events {
                if let Event::FramebufferSize(w, h) = event {
                    unsafe { gl::Viewport(0, 0, w, h) }
                }
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
    pub fn render_triangles(&self, vec: &Vec<draw::Triangle>) {
        let mut new_vec: Vec<f32> = Vec::new();
        for tri in vec {
            tri.to_points(&mut new_vec); 
        }
        unsafe {
            let (width, height) = self.get_resolution();
            let aspect_ratio = width as f32 / height as f32;

            // Create perspective matrix
            let fov_y = 45.0_f32.to_radians();
            let near = 0.1;
            let far = 100.0;
            let projection_matrix = glm::perspective(aspect_ratio, fov_y, near, far);

            gl::ActiveTexture(gl::TEXTURE0); // Activate texture unit 0
            gl::BindTexture(gl::TEXTURE_2D, self.shaders.texture_atlas); // Bind the texture

            self.shaders.use_program();
            self.shaders.set_uniform_matrix("u_ProjectionMatrix", &projection_matrix);
            self.shaders.set_uniform_matrix("u_CameraMatrix", &self.camera.to_matrix());

            // Set the texture uniform
            let texture_uniform_location = gl::GetUniformLocation(self.shaders.program, "textureSampler\0".as_ptr() as *const i8);
            gl::Uniform1i(texture_uniform_location, 0);

            // Vertex Array Object (VAO) and Vertex Buffer Object (VBO) setup
            let mut vao: u32 = 0;
            let mut vbo: u32 = 0; 

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (new_vec.len() * std::mem::size_of::<f32>()) as isize,
                new_vec.as_ptr() as *const _,
                gl::STATIC_DRAW,
                );

            // Set up position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // Set up texture coordinate attribute
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as i32,
                (3 * std::mem::size_of::<f32>()) as *const _,
                );
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            // Draw the square using two triangles
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // Clear buffers
            self.shaders.use_program(); // Ensure shader program is used
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, (vec.len() * 3) as i32);

            // Clean up
            gl::BindVertexArray(0);
            gl::DeleteVertexArrays(1, &mut vao);
            gl::DeleteBuffers(1, &mut vbo);
        }    
    }
}

