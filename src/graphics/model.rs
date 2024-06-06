use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use tobj::Material;
use ultraviolet::{Vec2, Vec3};

use crate::graphics::mesh::Mesh;
use crate::graphics::vertex::Vertex;
use crate::opengl::texture::{MagFilterParam, MinFilterParam, Texture, TextureType, WrapCoordinate, WrapParam};
use crate::shader::Shader;

pub struct Model {
    meshes: Vec<Mesh>,
}

fn load_meshes_from_models(models: Vec<tobj::Model>, materials: Vec<Material>, path_root: &Path) -> Vec<Mesh> {
    let mut meshes = Vec::<Mesh>::new();

    for model in models {
        // Assuming that all models will be textured
        let material_id = match model.mesh.material_id {
            None => panic!("Model doesn't have material id"),
            Some(id) => id,
        };

        // TODO: Other textures
        let material = materials[material_id].clone();
        let mut material_path = path_root.to_path_buf();
        material_path.push(material.diffuse_texture.expect("Missing diffuse texture"));

        let texture = Texture::new(TextureType::Texture2d).expect("Failed to allocate texture");

        texture.set_wrap(WrapCoordinate::S, WrapParam::Repeat);
        texture.set_wrap(WrapCoordinate::T, WrapParam::Repeat);
        texture.set_min_filter(MinFilterParam::Linear);
        texture.set_mag_filter(MagFilterParam::Linear);

        texture.load_from_image_path(material_path.to_str().unwrap(), true);


        let mesh = &model.mesh;
        let num_vertices = mesh.positions.len() / 3;

        let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
        let indices: Vec<u32> = mesh.indices.clone();

        let (p, t) = (&mesh.positions, &mesh.texcoords);
        for i in 0..num_vertices {
            // NOTE: Flipping V coord for texture here, so it displays properly, if some problem with flipped textures occur in the future it may be the reason
            vertices.push(Vertex::new(Vec3::new(p[i * 3], p[i * 3 + 1], p[i * 3 + 2]), Vec2::new(t[i * 2], -t[i * 2 + 1])));
        }

        meshes.push(Mesh::new(vertices, indices, vec![texture]));
    }

    meshes
}

impl Model {
    pub fn load_from_file(path: &str) -> Self {
        let mut dir = env::current_dir().unwrap();
        dir.push(path);
        dir.pop();

        let mut reader = BufReader::new(File::open(path).expect("Failed to open obj file"));
        let (models, materials) = tobj::load_obj_buf(&mut reader, &tobj::LoadOptions { triangulate: true, single_index: true, ..Default::default() }, |p| {
            let mut mtl_path = dir.clone();
            mtl_path.push(p);

            let f = File::open(&mtl_path).expect(format!("Couldn't open MTL file, path {}", mtl_path.clone().to_str().unwrap()).as_str());
            tobj::load_mtl_buf(&mut BufReader::new(f))
        }).unwrap();

        if models.is_empty() {
            panic!("Obj file contains no models");
        }

        let materials = match materials {
            Err(e) => panic!("Materials loading error {e}"),
            Ok(m) => m,
        };

        let meshes = load_meshes_from_models(models, materials, &dir);

        Self {
            meshes,
        }
    }

    pub fn draw(&self, shader_program: &Shader) {
        for mesh in &self.meshes {
            mesh.draw(shader_program);
        }
    }
}