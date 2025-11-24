// panpan/src/graphics.rs - Cross-platform graphics (GLES2 compatible)
use crate::types::Color;
use glow::HasContext;
use std::sync::Mutex;

static RENDERER: Mutex<Option<Renderer>> = Mutex::new(None);

struct Renderer {
    gl: glow::Context,
    width: i32,
    height: i32,
    rect_program: glow::NativeProgram,
    rect_vao: glow::NativeVertexArray,
    rect_vbo: glow::NativeBuffer,
}

/// Initialize graphics backend (called by runner)
pub(crate) fn init(gl: glow::Context) {
    let renderer = Renderer::new(gl);
    *RENDERER.lock().unwrap() = Some(renderer);
}

/// Set viewport (called by runner on resize)
pub(crate) fn set_viewport(width: i32, height: i32) {
    if let Some(renderer) = RENDERER.lock().unwrap().as_mut() {
        unsafe {
            renderer.gl.viewport(0, 0, width, height);
        }
        renderer.width = width;
        renderer.height = height;
    }
}

/// Clear the screen with a color
pub fn clear_screen(color: Color) {
    if let Some(renderer) = RENDERER.lock().unwrap().as_ref() {
        unsafe {
            renderer.gl.clear_color(color.r, color.g, color.b, color.a);
            renderer
                .gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }
}

/// Draw a filled rectangle
pub fn draw_rect(x: f32, y: f32, width: f32, height: f32, color: Color) {
    if let Some(renderer) = RENDERER.lock().unwrap().as_ref() {
        renderer.draw_rect_impl(x, y, width, height, color);
    }
}

/// Draw a circle (approximated with triangles)
pub fn draw_circle(x: f32, y: f32, radius: f32, color: Color) {
    // Simple implementation using draw_rect for now
    draw_rect(x - radius, y - radius, radius * 2.0, radius * 2.0, color);
}

/// Draw text (simple bitmap font)
pub fn draw_text(text: &str, x: f32, y: f32, size: f32, color: Color) {
    let char_width = size * 0.6;
    for (i, _ch) in text.chars().enumerate() {
        let char_x = x + (i as f32) * char_width;
        draw_rect(char_x, y, char_width * 0.8, size, color);
    }
}

impl Renderer {
    fn new(gl: glow::Context) -> Self {
        unsafe {
            // Vertex shader - GLES 2.0 compatible (no version directive = max compatibility)
            let vs_src = r#"
attribute vec2 aPos;
uniform mat4 projection;
void main() {
    gl_Position = projection * vec4(aPos, 0.0, 1.0);
}
"#;

            // Fragment shader - GLES 2.0 compatible
            let fs_src = r#"
precision mediump float;
uniform vec4 color;
void main() {
    gl_FragColor = color;
}
"#;

            let vs = Self::compile_shader(&gl, vs_src, glow::VERTEX_SHADER);
            let fs = Self::compile_shader(&gl, fs_src, glow::FRAGMENT_SHADER);

            let program = gl.create_program().expect("Cannot create program");
            gl.attach_shader(program, vs);
            gl.attach_shader(program, fs);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                panic!("Program link error: {}", gl.get_program_info_log(program));
            }

            gl.delete_shader(vs);
            gl.delete_shader(fs);

            // Create VAO and VBO
            let vao = gl.create_vertex_array().expect("Cannot create VAO");
            let vbo = gl.create_buffer().expect("Cannot create VBO");

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            // Unit square vertices (will be transformed by model matrix)
            let vertices: [f32; 12] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0];

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                std::slice::from_raw_parts(
                    vertices.as_ptr() as *const u8,
                    vertices.len() * std::mem::size_of::<f32>(),
                ),
                glow::STATIC_DRAW,
            );

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

            gl.bind_vertex_array(None);

            Self {
                gl,
                width: 800,
                height: 600,
                rect_program: program,
                rect_vao: vao,
                rect_vbo: vbo,
            }
        }
    }

    unsafe fn compile_shader(gl: &glow::Context, src: &str, ty: u32) -> glow::NativeShader {
        let shader = gl.create_shader(ty).expect("Cannot create shader");
        gl.shader_source(shader, src);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            panic!("Shader compile error: {}", gl.get_shader_info_log(shader));
        }

        shader
    }

    fn draw_rect_impl(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        unsafe {
            self.gl.use_program(Some(self.rect_program));

            // Create orthographic projection matrix
            let proj = self.ortho_matrix();

            // Model matrix (scale and translate)
            #[rustfmt::skip]
            let model = [
                w, 0.0, 0.0, 0.0,
                0.0, h, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                x, y, 0.0, 1.0,
            ];

            // Multiply projection * model
            let mut mvp = [0.0f32; 16];
            for i in 0..4 {
                for j in 0..4 {
                    for k in 0..4 {
                        mvp[i * 4 + j] += proj[i * 4 + k] * model[k * 4 + j];
                    }
                }
            }

            // Set uniforms
            let proj_loc = self
                .gl
                .get_uniform_location(self.rect_program, "projection");
            self.gl
                .uniform_matrix_4_f32_slice(proj_loc.as_ref(), false, &mvp);

            let color_loc = self.gl.get_uniform_location(self.rect_program, "color");
            self.gl
                .uniform_4_f32(color_loc.as_ref(), color.r, color.g, color.b, color.a);

            // Enable blending
            self.gl.enable(glow::BLEND);
            self.gl
                .blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            // Draw
            self.gl.bind_vertex_array(Some(self.rect_vao));
            self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
            self.gl.bind_vertex_array(None);

            self.gl.disable(glow::BLEND);
        }
    }

    fn ortho_matrix(&self) -> [f32; 16] {
        let left = 0.0f32;
        let right = self.width as f32;
        let bottom = self.height as f32;
        let top = 0.0f32;
        let near = -1.0f32;
        let far = 1.0f32;

        #[rustfmt::skip]
        let matrix = [
            2.0 / (right - left), 0.0, 0.0, 0.0,
            0.0, 2.0 / (top - bottom), 0.0, 0.0,
            0.0, 0.0, -2.0 / (far - near), 0.0,
            -(right + left) / (right - left),
            -(top + bottom) / (top - bottom),
            -(far + near) / (far - near),
            1.0,
        ];

        matrix
    }
}
