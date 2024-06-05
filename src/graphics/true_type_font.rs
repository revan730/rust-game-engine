use std::mem::size_of;

use rusttype::{Font, point, PositionedGlyph, Rect, Scale, vector};
use rusttype::gpu_cache::Cache;
use ultraviolet::Mat4;

use crate::opengl::draw_arrays;
use crate::opengl::Primitive::Triangles;
use crate::opengl::texture::{MagFilterParam, MinFilterParam, Texture, WrapCoordinate, WrapParam};
use crate::opengl::vertex_array_object::VertexArrayObject;
use crate::opengl::vertex_array_object::VertexAttribType::Float;
use crate::opengl::vertex_buffer_object::VertexBufferObject;
use crate::shader::Shader;

pub struct TrueTypeFont<'a> {
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

        let texture = Texture::new().expect("Failed to allocate texture for font");
        texture.bind();

        Texture::set_wrap(WrapCoordinate::S, WrapParam::ClampToEdge);
        Texture::set_wrap(WrapCoordinate::T, WrapParam::ClampToEdge);
        Texture::set_min_filter(MinFilterParam::Nearest);
        Texture::set_mag_filter(MagFilterParam::Nearest);

        Texture::load_empty(1280, 720); // TODO: Harcoded, replace
        Texture::unbind();

        Self {
            font,
            cache,
            texture,
        }
    }

    pub fn draw(&mut self, shader_program: &Shader, text: &str, font_size: f32, translation: Mat4) {
        let scale = Scale::uniform(font_size);
        let (screen_width, screen_height) = (1280.0, 720.0); // TODO: Hardcoded, fix

        let glyphs = layout_paragraph(&self.font, scale, screen_width as u32, text);

        for glyph in &glyphs {
            self.cache.queue_glyph(0, glyph.clone());
        }

        self.texture.bind();
        self.cache.cache_queued(|rect, data| {
            self.texture.upload_pixels(rect.min.x, rect.min.y, rect.width(), rect.height(), data.as_ptr());
        }).unwrap();
        Texture::unbind();

        #[derive(Copy, Clone)]
        #[repr(C)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
            colour: [f32; 4],
        }

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

        let vao = VertexArrayObject::new().expect("Failed to allocate vertex array object for font");
        vao.bind();

        let vertex_buffer = VertexBufferObject::new().expect("Failed to allocate vertex buffer for font");
        vertex_buffer.bind();

        VertexBufferObject::load_data(vertices.len() * size_of::<Vertex>(), vertices.as_ptr());

        VertexArrayObject::set_vertex_attribute(0, 4, Float, false, size_of::<Vertex>(), 0);
        VertexArrayObject::set_vertex_attribute(1, 4, Float, false, size_of::<Vertex>(), size_of::<[f32; 4]>());


        draw_arrays(Triangles, 0, vertices.len());
        VertexBufferObject::unbind();
    }
}