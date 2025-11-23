use super::Renderer2D;
use crate::util::Color;
use glow::HasContext;
use crate::engine::get_screen_size;

pub struct GLESRenderer {
    pub gl: glow::Context,
    rect_program: Option<glow::NativeProgram>,
    rect_vao: Option<glow::NativeVertexArray>,
    rect_vbo: Option<glow::NativeBuffer>,
}

impl GLESRenderer {
    pub fn new(gl: glow::Context) -> Self {
        let mut renderer = Self { 
            gl,
            rect_program: None,
            rect_vao: None,
            rect_vbo: None,
        };
        renderer.init_rect_shader();
        renderer
    }

    fn init_rect_shader(&mut self) {
        unsafe {
            let vs_src = r#"#version 330 core
                layout (location = 0) in vec2 aPos;
                uniform mat4 projection;
                void main() {
                    gl_Position = projection * vec4(aPos, 0.0, 1.0);
                }
            "#;

            let fs_src = r#"#version 330 core
                out vec4 FragColor;
                uniform vec4 color;
                void main() {
                    FragColor = color;
                }
            "#;

            let vs = self.compile_shader(vs_src, glow::VERTEX_SHADER);
            let fs = self.compile_shader(fs_src, glow::FRAGMENT_SHADER);
            
            let program = self.gl.create_program().unwrap();
            self.gl.attach_shader(program, vs);
            self.gl.attach_shader(program, fs);
            self.gl.link_program(program);
            
            // Check linking status
            if !self.gl.get_program_link_status(program) {
                let log = self.gl.get_program_info_log(program);
                println!("ERROR: Shader linking failed: {}", log);
            } else {
                println!("Shader program linked successfully!");
            }
            
            self.gl.delete_shader(vs);
            self.gl.delete_shader(fs);

            let vao = self.gl.create_vertex_array().unwrap();
            let vbo = self.gl.create_buffer().unwrap();

            self.gl.bind_vertex_array(Some(vao));
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            // Rectangle vertices (2 triangles forming a unit square)
            let vertices: [f32; 12] = [
                0.0, 0.0,
                1.0, 0.0,
                1.0, 1.0,
                0.0, 0.0,
                1.0, 1.0,
                0.0, 1.0,
            ];

            let bytes = std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * std::mem::size_of::<f32>(),
            );
            self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytes, glow::STATIC_DRAW);

            self.gl.enable_vertex_attrib_array(0);
            self.gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
            self.gl.bind_vertex_array(None);

            self.rect_program = Some(program);
            self.rect_vao = Some(vao);
            self.rect_vbo = Some(vbo);
            
            println!("Rectangle shader initialized!");
        }
    }

    unsafe fn compile_shader(&self, src: &str, shader_type: u32) -> glow::NativeShader {
        unsafe {
            let shader = self.gl.create_shader(shader_type).unwrap();
            self.gl.shader_source(shader, src);
            self.gl.compile_shader(shader);
            
            if !self.gl.get_shader_compile_status(shader) {
                let log = self.gl.get_shader_info_log(shader);
                println!("ERROR: Shader compilation failed: {}", log);
            }
            
            shader
        }
    }
}

impl Renderer2D for GLESRenderer {
    fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            self.gl.clear_color(r, g, b, a);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    fn draw_text(&self, text: &str, x: f32, y: f32, scale: f32, color: Color) {
        // Simple text rendering - draw small rectangles for each character
        let char_width = 8.0 * scale;
        let char_height = 12.0 * scale;
        
        for (i, _ch) in text.chars().enumerate() {
            let char_x = x + (i as f32 * char_width);
            self.draw_rect(char_x, y, char_width * 0.7, char_height, color);
        }
    }

    fn draw_rect(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let (screen_w, screen_h) = get_screen_size();
        if screen_w == 0 || screen_h == 0 {
            println!("WARNING: Screen size not set! Cannot draw.");
            return;
        }

        unsafe {
            if let Some(program) = self.rect_program {
                self.gl.use_program(Some(program));

                // Orthographic projection matrix
                let left = 0.0f32;
                let right = screen_w as f32;
                let bottom = screen_h as f32;
                let top = 0.0f32;
                let near = -1.0f32;
                let far = 1.0f32;

                let projection = [
                    2.0 / (right - left), 0.0, 0.0, 0.0,
                    0.0, 2.0 / (top - bottom), 0.0, 0.0,
                    0.0, 0.0, -2.0 / (far - near), 0.0,
                    -(right + left) / (right - left),
                    -(top + bottom) / (top - bottom),
                    -(far + near) / (far - near),
                    1.0,
                ];

                // Model matrix for position and scale
                let model = [
                    w, 0.0, 0.0, 0.0,
                    0.0, h, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    x, y, 0.0, 1.0,
                ];

                // Combine matrices
                let mut mvp = [0.0f32; 16];
                for i in 0..4 {
                    for j in 0..4 {
                        for k in 0..4 {
                            mvp[i * 4 + j] += projection[i * 4 + k] * model[k * 4 + j];
                        }
                    }
                }

                let proj_loc = self.gl.get_uniform_location(program, "projection");
                if let Some(loc) = proj_loc {
                    self.gl.uniform_matrix_4_f32_slice(Some(&loc), false, &mvp);
                }

                let color_loc = self.gl.get_uniform_location(program, "color");
                if let Some(loc) = color_loc {
                    self.gl.uniform_4_f32(Some(&loc), color.r, color.g, color.b, color.a);
                }

                self.gl.enable(glow::BLEND);
                self.gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

                if let Some(vao) = self.rect_vao {
                    self.gl.bind_vertex_array(Some(vao));
                    self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
                    self.gl.bind_vertex_array(None);
                }

                self.gl.disable(glow::BLEND);
                self.gl.use_program(None);
            }
        }
    }
}