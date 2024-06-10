use std::collections::HashSet;
use std::rc::Rc;

use beryllium::*;
use beryllium::events::SDLK_ESCAPE;
use beryllium::video::GlSwapInterval::Vsync;
use ogl33::glViewport;
use ultraviolet::Vec3;

use crate::camera::Camera;
use crate::graphics::model::Model;
use crate::graphics::node_3d::Node3D;
use crate::graphics::player_character::PlayerCharacter;
use crate::graphics::scene::Scene;
use crate::graphics::skybox::Skybox;
use crate::graphics::static_body_3d::StaticBody3D;
use crate::math::rotation::Rotation;
use crate::opengl::{BlendFactor, Capability, UnpackAlignment};
use crate::opengl::ClearBitFlags::{ColorBuffer, DepthBuffer};
use crate::shader::Shader;

mod shader;
mod camera;
mod opengl;
mod graphics;
mod math;


const SCR_WIDTH: i32 = 1280;
const SCR_HEIGHT: i32 = 720;

const CUBE_POSITIONS: [Vec3; 4] = [
    Vec3::new(-10.3, 1.25, 5.0),
    Vec3::new(-13.3, 1.36, 5.0),
    Vec3::new(-13.4, 1.4, 7.0),
    Vec3::new(-13.4, 1.4, 10.0),
    /*Vec3::new(2.4, -0.4, -3.5),
    Vec3::new(-1.7, 3.0, -7.5),
    Vec3::new(1.3, -2.0, -2.5),
    Vec3::new(1.5, 2.0, -2.5),
    Vec3::new(1.5, 0.2, -1.5),
    Vec3::new(-1.3, 1.0, -1.5),*/
];

fn framebuffer_size_callback(width: i32, height: i32) {
    unsafe {
        glViewport(0, 0, width, height);
    }
}


fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    #[cfg(target_os = "macos")]
    {
        sdl
            .set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
            .unwrap();
    }

    let win_args = video::CreateWinArgs {
        title: "OpenGL",
        width: SCR_WIDTH,
        height: SCR_HEIGHT,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let win = sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context");
    win.set_swap_interval(Vsync).unwrap();
    sdl.set_relative_mouse_mode(true).unwrap();

    opengl::load_gl(&win);
    opengl::enable(Capability::DepthTest);
    opengl::enable(Capability::Blending);
    opengl::blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
    opengl::pixel_store_unpack_alignment(UnpackAlignment::One);

    let shader_program_font = Shader::from_files("res/shaders/font.vs", "res/shaders/font.fs");

    shader_program_font.bind();
    shader_program_font.set_int("tex", 0);

    let shader_program = Shader::from_files("res/shaders/default.vs", "res/shaders/default.fs");

    shader_program.bind();
    shader_program.set_int("texture1", 0);
    shader_program.set_int("texture2", 1);


    let mut last_time = 0.0;
    let mut keys_held = HashSet::new();
    let mut mouse_delta = (0, 0);


    //let mut scene = create_default_scene();
    let mut scene = create_physics_test_scene();

    'main_loop: loop {
        let mut mouse_moved = false;
        while let Some((event, _)) = sdl.poll_events() {
            match event {
                events::Event::Quit => break 'main_loop,
                events::Event::Key { pressed, keycode, .. } => {
                    if keycode == SDLK_ESCAPE {
                        break 'main_loop;
                    }

                    if pressed {
                        keys_held.insert(keycode);
                    } else {
                        keys_held.remove(&keycode);
                    }
                }
                events::Event::MouseMotion { x_delta, y_delta, .. } => {
                    mouse_delta.0 = x_delta;
                    mouse_delta.1 = -y_delta;
                    mouse_moved = true;
                }
                events::Event::WindowResized { width, height, .. } => framebuffer_size_callback(width, height),
                _ => (),
            }
        }

        let time = sdl.get_ticks() as f32 / 10_000.0_f32;
        let delta_time = time - last_time;
        last_time = time;

        let mouse_delta = if mouse_moved {
            Some(mouse_delta)
        } else {
            None
        };

        scene.update(delta_time, &keys_held, mouse_delta);

        opengl::clear_color(0.2, 0.3, 0.3, 1.0);
        opengl::clear(ColorBuffer | DepthBuffer);


        scene.draw(&shader_program, &shader_program_font);

        win.swap_window();
    }
}

fn create_default_scene() -> Scene<'static> {
    let camera: Camera = Camera::from_vec3(Vec3::default(), Vec3::new(0.0, 1.0, 0.0), -62.0, -16.29);
    let player: PlayerCharacter = PlayerCharacter::new(Node3D { world_position: Vec3::new(-13.65, 5.6, 13.36), rotation: Rotation::default(), scale: Vec3::new(1.0, 1.0, 1.0) }, camera);

    let container_model = Model::load_from_file("res/models/cottage.obj");
    let container_model = Rc::new(container_model);

    let landscape_model = Model::load_from_file("res/models/landscape.obj");
    let landscape_model = Rc::new(landscape_model);

    let mut static_bodies = Vec::<StaticBody3D>::with_capacity(CUBE_POSITIONS.len());


    for (i, cube_pos) in CUBE_POSITIONS.iter().enumerate() {
        let angle = (20.0f32 * i as f32).to_radians();
        let rotation = Rotation { angle_x: 0.0, angle_y: angle, angle_z: 0.0 };
        let body = StaticBody3D { node3d: Node3D { world_position: *cube_pos, scale: Vec3::new(0.05, 0.05, 0.05), rotation }, model: container_model.clone() };
        static_bodies.push(body);
    }

    let shader_program_skybox = Shader::from_files("res/shaders/skybox.vs", "res/shaders/skybox.fs");

    let landscape_rotation = Rotation { angle_x: 0.0, angle_y: 0.0, angle_z: 0.0 };
    static_bodies.push(StaticBody3D { node3d: Node3D { world_position: Vec3::default(), scale: Vec3::new(5.0, 5.0, 5.0), rotation: landscape_rotation }, model: landscape_model.clone() });

    let skybox = Skybox::new_from_image_paths(shader_program_skybox, ["res/models/textures/skybox/right.jpg", "res/models/textures/skybox/left.jpg", "res/models/textures/skybox/top.jpg", "res/models/textures/skybox/bottom.jpg", "res/models/textures/skybox/front.jpg", "res/models/textures/skybox/back.jpg"]);

    Scene::new(static_bodies, Some(skybox), player)
}

fn create_physics_test_scene() -> Scene<'static> {
    let camera: Camera = Camera::from_vec3(Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 1.0, 0.0), -62.0, -16.29);
    let player: PlayerCharacter = PlayerCharacter::new(Node3D { world_position: Vec3::new(0.0, 5.0, 0.0), rotation: Rotation::default(), scale: Vec3::new(1.0, 1.0, 1.0) }, camera);

    let container_model = Model::load_from_file("res/models/cottage.obj");
    let container_model = Rc::new(container_model);

    let mut static_bodies = Vec::<StaticBody3D>::with_capacity(1);
    static_bodies.push(StaticBody3D { node3d: Node3D { world_position: Vec3::new(0.0, -0.1, 0.0), scale: Vec3::new(1000.00, 0.1, 1000.0), rotation: Rotation::default() }, model: container_model.clone() });

    Scene::new(static_bodies, None, player)
}