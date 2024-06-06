use std::mem::size_of;

use rusttype::{Font, point, PositionedGlyph, Rect, Scale, vector};
use rusttype::gpu_cache::Cache;
use ultraviolet::Mat4;

use crate::opengl::draw_arrays;
use crate::opengl::Primitive::Triangles;
use crate::opengl::texture::{MagFilterParam, MinFilterParam, Texture, TextureType, WrapCoordinate, WrapParam};
use crate::opengl::vertex_array_object::VertexArrayObject;
use crate::opengl::vertex_array_object::VertexAttribType::Float;
use crate::opengl::vertex_buffer_object::{BufferUsage, VertexBufferObject};
use crate::shader::Shader;

#[derive(Copy, Clone)]
#[repr(C)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    colour: [f32; 4],
}

pub struct TrueTypeFont<'a> {
    vao: VertexArrayObject,
    vbo: VertexBufferObject,
    font: Font<'a>,
    cache: Cache<'a>,
    texture: Texture,
}

fn layout_paragraph<'a>(
    font: &Font<'a>,
    scale: Scale,
    width: u32,
    text: &str,
) -> Vec<PositionedGlyph<'a>> {
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let mut caret = point(0.0, v_metrics.ascent);
    let mut last_glyph_id = None;
    for c in text.chars() {
        if c.is_control() {
            match c {
                '\r' => {
                    caret = point(0.0, caret.y + advance_height);
                }
                '\n' => {}
                _ => {}
            }
            continue;
        }
        let base_glyph = font.glyph(c);
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());
        let mut glyph = base_glyph.scaled(scale).positioned(caret);
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph.set_position(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }
    result
}


impl TrueTypeFont<'_> {
    pub fn load_from_file(path: &str) -> Self {
        let mut f = std::fs::File::open(path).expect("Failed to open font file");
        let mut bytes = vec![];
        std::io::Read::read_to_end(&mut f, &mut bytes).expect("Failed to read font file");

        let font = Font::try_from_vec(bytes).expect("Error constructing Font");

        let (cache_width, cache_height) = (1280, 720);
        let mut cache: Cache<'_> = Cache::builder()
            .dimensions(cache_width, cache_height)
            .build();

        let texture = Texture::new(TextureType::Texture2d).expect("Failed to allocate texture for font");

        texture.set_wrap(WrapCoordinate::S, WrapParam::ClampToEdge);
        texture.set_wrap(WrapCoordinate::T, WrapParam::ClampToEdge);
        texture.set_min_filter(MinFilterParam::Nearest);
        texture.set_mag_filter(MagFilterParam::Nearest);

        texture.load_empty(1280, 720); // TODO: Harcoded, replace

        let vao = VertexArrayObject::new().expect("Failed to allocate vertex array object for font");
        vao.bind();

        let vbo = VertexBufferObject::new().expect("Failed to allocate vertex buffer for font");
        vbo.bind();

        VertexArrayObject::set_vertex_attribute(0, 4, Float, false, size_of::<Vertex>(), 0);
        VertexArrayObject::set_vertex_attribute(1, 4, Float, false, size_of::<Vertex>(), size_of::<[f32; 4]>());

        Self {
            font,
            cache,
            texture,
            vao,
            vbo,
        }
    }

    pub fn draw(&mut self, shader_program: &Shader, text: &str, font_size: f32, translation: Mat4) {
        // TODO: It's not optimal to repeat this whole process when the text is the same as it was in the last call (should not be done at font level but rather on GUI widget level - same font can be used to draw lots of different strings)
        // TODO: Also, it's not optimal to reallocate VAO/VBO and set VAO attributes again as they will basically never change and new vertex data can be reloaded into existing buffer

        let scale = Scale::uniform(font_size);
        let (screen_width, screen_height) = (1280.0, 720.0); // TODO: Hardcoded, fix

        let glyphs = layout_paragraph(&self.font, scale, screen_width as u32, text);

        for glyph in &glyphs {
            self.cache.queue_glyph(0, glyph.clone());
        }

        self.cache.cache_queued(|rect, data| {
            self.texture.upload_pixels(rect.min.x, rect.min.y, rect.width(), rect.height(), data.as_ptr());
        }).unwrap();

        let colour = [1.0, 1.0, 1.0, 1.0];
        let origin = point(0.0, 0.0);

        let vertices: Vec<Vertex> = glyphs.iter()
            .filter_map(|g| self.cache.rect_for(0, g).ok().flatten())
            .flat_map(|(uv_rect, screen_rect)| {
                let gl_rect = Rect {
                    min: origin
                        + (vector(
                        screen_rect.min.x as f32 / screen_width - 0.5,
                        1.0 - screen_rect.min.y as f32 / screen_height - 0.5,
                    )) * 2.0,
                    max: origin
                        + (vector(
                        screen_rect.max.x as f32 / screen_width - 0.5,
                        1.0 - screen_rect.max.y as f32 / screen_height - 0.5,
                    )) * 2.0,
                };
                vec![
                    Vertex {
                        position: [gl_rect.min.x, gl_rect.max.y],
                        tex_coords: [uv_rect.min.x, uv_rect.max.y],
                        colour,
                    },
                    Vertex {
                        position: [gl_rect.min.x, gl_rect.min.y],
                        tex_coords: [uv_rect.min.x, uv_rect.min.y],
                        colour,
                    },
                    Vertex {
                        position: [gl_rect.max.x, gl_rect.min.y],
                        tex_coords: [uv_rect.max.x, uv_rect.min.y],
                        colour,
                    },
                    Vertex {
                        position: [gl_rect.max.x, gl_rect.min.y],
                        tex_coords: [uv_rect.max.x, uv_rect.min.y],
                        colour,
                    },
                    Vertex {
                        position: [gl_rect.max.x, gl_rect.max.y],
                        tex_coords: [uv_rect.max.x, uv_rect.max.y],
                        colour,
                    },
                    Vertex {
                        position: [gl_rect.min.x, gl_rect.max.y],
                        tex_coords: [uv_rect.min.x, uv_rect.max.y],
                        colour,
                    },
                ]
            }).collect();

        shader_program.bind();

        Texture::set_active_texture(0);
        self.texture.bind();
        shader_program.set_int("tex", 0);
        shader_program.set_mat4("translation", translation);

        self.vao.bind();

        VertexBufferObject::load_data(vertices.len() * size_of::<Vertex>(), vertices.as_ptr(), BufferUsage::DynamicDraw);


        draw_arrays(Triangles, 0, vertices.len());
        VertexArrayObject::unbind();
    }
}