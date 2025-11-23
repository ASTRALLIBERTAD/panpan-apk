// panpan_sdf_text.rs
// Signed Distance Field (SDF) text renderer for PanPan engine
// - GLES2-compatible shaders
// - No VAO (works on GLES2)
// - Batch quads into a single VBO per frame
// - Loads a precomputed SDF atlas PNG + simple JSON metrics
//
// Place this file in your panpan library (e.g. src/text_sdf.rs) and
// call from example_crate. This assumes your engine exposes `gl: glow::Context`
// and a projection matrix upload helper. I inspected your uploaded files at
// `/mnt/data/engine.rs` and `/mnt/data/opengles.rs` to make this compatible.

use std::collections::HashMap;
use std::sync::Arc;

use glow::HasContext;

use serde::Deserialize;
use crate::util::Color;

#[derive(Deserialize)]
struct GlyphMeta {
    // x,y in pixels of atlas, width,height in pixels, xoffset,yoffset,xadvance in px
    ch: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    xoffset: i32,
    yoffset: i32,
    xadvance: i32,
}

#[derive(Deserialize)]
struct FontMeta {
    atlas_width: u32,
    atlas_height: u32,
    glyphs: Vec<GlyphMeta>,
}

pub struct Glyph {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
    pub width: f32,
    pub height: f32,
    pub xoffset: f32,
    pub yoffset: f32,
    pub xadvance: f32,
}

pub struct SdfFont {
    pub texture: glow::NativeTexture,
    pub glyphs: HashMap<u32, Glyph>,
    pub line_height: f32,
}

pub struct TextRenderer {
    gl: Arc<glow::Context>,
    program: glow::NativeProgram,
    attrib_pos: Option<glow::NativeVertexArray>, // we'll not use VAO on GLES2, keep names consistent
    vbo: glow::NativeBuffer,
    font: Option<SdfFont>,
    // uniform locations
uniform mat4 u_proj;
varying vec2 v_uv;

void main() {
    v_uv = a_uv;
    gl_Position = u_proj * vec4(a_pos.xy, 0.0, 1.0);
}
"#;

            // Fragment shader uses red channel as SDF distance value
            let frag = r#"
precision mediump float;

uniform sampler2D u_tex;
uniform vec4 u_color;
varying vec2 v_uv;

// smoothing width (tweak per atlas), smaller -> crisper
const float smoothing = 0.1;

void main() {
    float sd = texture2D(u_tex, v_uv).r; // SDF stored in red channel
    // convert SDF value to alpha using smoothstep around 0.5
    float alpha = smoothstep(0.5 - smoothing, 0.5 + smoothing, sd);
    gl_FragColor = vec4(u_color.rgb, u_color.a * alpha);
}
"#;

            let vs = gl.create_shader(glow::VERTEX_SHADER).map_err(|e| e.to_string())?;
            gl.shader_source(vs, vert);
            gl.compile_shader(vs);
            if !gl.get_shader_compile_status(vs) {
                return Err(gl.get_shader_info_log(vs));
            }

            let fs = gl.create_shader(glow::FRAGMENT_SHADER).map_err(|e| e.to_string())?;
            gl.shader_source(fs, frag);
            gl.compile_shader(fs);
            if !gl.get_shader_compile_status(fs) {
                return Err(gl.get_shader_info_log(fs));
            }

            let program = gl.create_program().map_err(|e| e.to_string())?;
            gl.attach_shader(program, vs);
            gl.attach_shader(program, fs);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                return Err(gl.get_program_info_log(program));
            }

            // clean up shaders
            gl.detach_shader(program, vs);
            gl.detach_shader(program, fs);
            gl.delete_shader(vs);
            gl.delete_shader(fs);

            // create VBO
            let vbo = gl.create_buffer().map_err(|e| e.to_string())?;

            // attributes locations (we rely on auto locations)
            let _a_pos_loc = 0; // we'll use attribute location 0
            let _a_uv_loc = 1;  // attribute location 1
            // bind attribute locations (some GL implementations allow binding before linking,
            // but to keep GLES2 compatibility we will use attribute indices with glVertexAttribPointer directly)

            // uniforms
            gl.use_program(Some(program));
            let u_proj = gl.get_uniform_location(program, "u_proj").ok_or("missing u_proj uniform")?;
            let u_color = gl.get_uniform_location(program, "u_color").ok_or("missing u_color uniform")?;
            let u_tex = gl.get_uniform_location(program, "u_tex").ok_or("missing u_tex uniform")?;
            gl.use_program(None);

            Ok(TextRenderer {
                gl,
                program,
                attrib_pos: None,
                vbo,
                font: None,
                u_proj,
                u_color,
                u_tex,
            })
        }
    }

    /// Load an SDF PNG atlas and a JSON metadata file describing glyph rectangles & metrics.
    /// `atlas_png_bytes` - bytes of PNG file
    /// `meta_json` - JSON string matching FontMeta
    pub fn load_font_from_bytes(&mut self, atlas_png_bytes: &[u8], meta_json: &str) -> Result<(), String> {
        unsafe {
            // decode PNG using the `image` crate (add dependency in Cargo.toml: image = "0.24")
            let img = image::load_from_memory(atlas_png_bytes).map_err(|e| e.to_string())?;
            let img = img.to_rgba8();
            let (w, h) = img.dimensions();

            let meta: FontMeta = serde_json::from_str(meta_json).map_err(|e| e.to_string())?;

            // upload texture
            let tex = self.gl.create_texture().map_err(|e| e.to_string())?;
            self.gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            // SDF usually stored in single channel, but we'll upload RGBA and read .r in shader
            self.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                w as i32,
                h as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(img.as_raw())),
            );

            self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

            // build glyph lookup
            let mut map = HashMap::new();
            for g in meta.glyphs.iter() {
                let u0 = (g.x as f32) / (meta.atlas_width as f32);
                let v0 = (g.y as f32) / (meta.atlas_height as f32);
                let u1 = ((g.x + g.width) as f32) / (meta.atlas_width as f32);
                let v1 = ((g.y + g.height) as f32) / (meta.atlas_height as f32);

                map.insert(g.ch, Glyph {
                    u0, v0, u1, v1,
                    width: g.width as f32,
                    height: g.height as f32,
                    xoffset: g.xoffset as f32,
                    yoffset: g.yoffset as f32,
                    xadvance: g.xadvance as f32,
                });
            }

            self.font = Some(SdfFont {
                texture: tex,
                glyphs: map,
                line_height:  (meta.glyphs.iter().map(|x| x.height).max().unwrap_or(16) as f32),
            });

            Ok(())
        }
    }

    /// Draw text at (x,y) in screen space (assumes projection already set to pixel coords)
    pub fn draw_text(&mut self, x: f32, y: f32, text: &str, color: Color, proj: [[f32;4];4]) {
        unsafe {
            let gl = &self.gl;
            let font = match &self.font { Some(f) => f, None => return };

            gl.use_program(Some(self.program));
            // upload projection
            gl.uniform_matrix_4_f32_slice(Some(&self.u_proj), false, &proj.concat());
            // upload color
            gl.uniform_4_f32(Some(&self.u_color), color.r, color.g, color.b, color.a);

            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(font.texture));
            gl.uniform_1_i32(Some(&self.u_tex), 0);

            // build vertex buffer: each glyph -> 6 vertices (2 triangles), vertex = (pos.x,pos.y, u,v)
            let mut verts: Vec<f32> = Vec::with_capacity(text.len() * 6 * 4);

            let mut pen_x = x;
            let pen_y = y;

            for c in text.chars() {
                let code = c as u32;
                if let Some(g) = font.glyphs.get(&code) {
                    let x0 = pen_x + g.xoffset;
                    let y0 = pen_y - g.yoffset; // depending on your coordinate convention
                    let x1 = x0 + g.width;
                    let y1 = y0 - g.height;

                    // UVs
                    let u0 = g.u0; let v0 = g.v0; let u1 = g.u1; let v1 = g.v1;

                    // Triangle 1
                    verts.extend_from_slice(&[x0, y0, u0, v0]);
                    verts.extend_from_slice(&[x1, y0, u1, v0]);
                    verts.extend_from_slice(&[x1, y1, u1, v1]);
                    // Triangle 2
                    verts.extend_from_slice(&[x0, y0, u0, v0]);
                    verts.extend_from_slice(&[x1, y1, u1, v1]);
                    verts.extend_from_slice(&[x0, y1, u0, v1]);

                    pen_x += g.xadvance;
                }
            }

            if verts.is_empty() {
                gl.use_program(None);
                return;
            }

            // upload VBO
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            // convert to bytes
            let byte_slice = std::slice::from_raw_parts(verts.as_ptr() as *const u8, verts.len() * std::mem::size_of::<f32>());
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, byte_slice, glow::STREAM_DRAW);

            // set up vertex attributes: pos (2 floats) uv (2 floats)
            let stride = 4 * std::mem::size_of::<f32>() as i32;
            // attribute location 0 => a_pos
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 0);
            // attribute location 1 => a_uv
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, stride, (2 * std::mem::size_of::<f32>()) as i32);

            // blending
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            let vert_count = (verts.len() / 4) as i32;
            gl.draw_arrays(glow::TRIANGLES, 0, vert_count);

            gl.disable_vertex_attrib_array(0);
            gl.disable_vertex_attrib_array(1);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_texture(glow::TEXTURE_2D, None);
            gl.use_program(None);
        }
    }
}

// --------------------------- Example usage (example_crate) ---------------------------
// In your example_crate's main.rs (or sample) you can do:
//
// let gl = /* obtain glow::Context from your runner (you said you already have desktop/android runners) */;
// let mut tr = TextRenderer::new(Rc::new(gl)).unwrap();
//
// // load atlas + metadata (you can bundle them into the example_crate assets)
// let atlas_bytes = include_bytes!("../assets/roboto_sdf.png");
// let meta = include_str!("../assets/roboto_sdf.json");
// tr.load_font_from_bytes(atlas_bytes, meta).unwrap();
//
// // in render loop, prepare an orthographic projection (pixel coords):
// let proj = ortho(0.0, window_w as f32, window_h as f32, 0.0, -1.0, 1.0);
// tr.draw_text(20.0, 40.0, "Hello SDF!", Color{r:1.0,g:1.0,b:1.0,a:1.0}, proj);

// Small helper to build column-major 4x4 matrix array for the uniform
trait ToF32Slice { fn concat(&self) -> [f32;16]; }
impl ToF32Slice for [[f32;4];4] {
    fn concat(&self) -> [f32;16] {
        [
            self[0][0], self[0][1], self[0][2], self[0][3],
            self[1][0], self[1][1], self[1][2], self[1][3],
            self[2][0], self[2][1], self[2][2], self[2][3],
            self[3][0], self[3][1], self[3][2], self[3][3],
        ]
    }
}

// ortho helper (left,right,bottom,top) returning column-major 4x4
pub fn ortho(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> [[f32;4];4] {
    let tx = -(r + l) / (r - l);
    let ty = -(t + b) / (t - b);
    let tz = -(f + n) / (f - n);
    [
        [2.0/(r-l), 0.0, 0.0, 0.0],
        [0.0, 2.0/(t-b), 0.0, 0.0],
        [0.0, 0.0, -2.0/(f-n), 0.0],
        [tx, ty, tz, 1.0],
    ]
}

// -------------------------------------------------------------------------------------

// Note: dependencies to add in panpan/Cargo.toml (optional features):
// [dependencies]
// glow = "0.11"
// image = "0.24"
// serde = { version = "1.0", features = ["derive"] }

// -------------------------------------------------------------------------------------
// Tips / Next steps:
// 1) Generate an SDF atlas using msdfgen / msdf-atlas-gen. Export a single-channel SDF (or keep RGBA) and a JSON with glyph boxes.
// 2) Tune `smoothing` constant in the fragment shader. It depends on the SDF generation distance range and atlas filtering.
// 3) If you want crisp subpixel rendering, generate higher resolution SDFs and use smaller smoothing.
// 4) Because PanPan is a library, keep the atlas & JSON inside example_crate's assets and `include_bytes!` them into the sample binary for convenience.

// If you want, I can also produce a small `roboto_sdf.png` + `roboto_sdf.json` example asset and the example_crate `main.rs` that uses your existing runners. Tell me if you want me to embed assets directly into the example crate (I can generate a tiny ASCII SDF atlas for demo).
