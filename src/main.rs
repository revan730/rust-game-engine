use std::collections::HashSet;
use std::rc::Rc;

use beryllium::*;
use beryllium::events::{SDLK_a, SDLK_d, SDLK_ESCAPE, SDLK_s, SDLK_w};
use beryllium::events::SDL_Keycode;
use beryllium::video::GlSwapInterval::Vsync;
use ogl33::glViewport;
use ultraviolet::{Mat4, Vec3};
use ultraviolet::projection::perspective_gl;

use crate::camera::Camera;
use crate::camera::CameraMovement;
use crate::graphics::model::Model;
use crate::graphics::node_3d::Node3D;
use crate::graphics::scene::Scene;
use crate::graphics::skybox::Skybox;
use crate::graphics::static_body_3d::StaticBody3D;
use crate::graphics::true_type_font::TrueTypeFont;
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

fn process_input(pressed_keys: &HashSet<SDL_Keycode>, camera: &mut Camera, dt: f32) {
    if pressed_keys.contains(&SDLK_w) {
        camera.process_keyboard(CameraMovement::Forward, dt);
    }

    if pressed_keys.contains(&SDLK_s) {
        camera.process_keyboard(CameraMovement::Backward, dt);
    }

    if pressed_keys.contains(&SDLK_a) {
        camera.process_keyboard(CameraMovement::Left, dt);
    }

    if pressed_keys.contains(&SDLK_d) {
        camera.process_keyboard(CameraMovement::Right, dt);
    }
}

fn main() {
    let mut camera: Camera = Camera::from_vec3(Vec3::new(-13.65, 1.6, 13.36), Vec3::new(0.0, 1.0, 0.0), -62.0, -16.29); // If world surface will have y coord 0 our camera will lay on the floor

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

    let container_model = Model::load_from_file("res/models/cottage.obj");
    let container_model = Rc::new(container_model);

    let landscape_model = Model::load_from_file("res/models/landscape.obj");
    let landscape_model = Rc::new(landscape_model);

    let shader_program_font = Shader::from_files("res/shaders/font.vs", "res/shaders/font.fs");

    shader_program_font.bind();
    shader_program_font.set_int("tex", 0);

    let shader_program = Shader::from_files("res/shaders/default.vs", "res/shaders/default.fs");

    shader_program.bind();
    shader_program.set_int("texture1", 0);
    shader_program.set_int("texture2", 1);

    let shader_program_skybox = Shader::from_files("res/shaders/skybox.vs", "res/shaders/skybox.fs");


    let mut last_time = 0.0;
    let mut keys_held = HashSet::new();

    let mut static_bodies = Vec::<StaticBody3D>::with_capacity(CUBE_POSITIONS.len());


    for (i, cube_pos) in CUBE_POSITIONS.iter().enumerate() {
        let angle = (20.0f32 * i as f32).to_radians();
        let rotation = Rotation { angle_x: 0.0, angle_y: angle, angle_z: 0.0 };
        let body = StaticBody3D { node3d: Node3D { world_position: *cube_pos, scale: Vec3::new(0.05, 0.05, 0.05), rotation }, model: container_model.clone() };
        static_bodies.push(body);
    }

    let landscape_rotation = Rotation { angle_x: 0.0, angle_y: 0.0, angle_z: 0.0 };
    static_bodies.push(StaticBody3D { node3d: Node3D { world_position: Vec3::new(0.0, 0.0, 0.0), scale: Vec3::new(5.0, 5.0, 5.0), rotation: landscape_rotation }, model: landscape_model.clone() });

    let mut skybox = Skybox::new_from_image_paths(shader_program_skybox, ["res/models/textures/skybox/right.jpg", "res/models/textures/skybox/left.jpg", "res/models/textures/skybox/top.jpg", "res/models/textures/skybox/bottom.jpg", "res/models/textures/skybox/front.jpg", "res/models/textures/skybox/back.jpg"]); // TODO: Should be a part of the scene

    let scene = Scene::new(static_bodies, Some(skybox));
    let mut font = TrueTypeFont::load_from_file("res/fonts/futura.ttf");

    'main_loop: loop {
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
                events::Event::MouseMotion { x_delta, y_delta, .. } => camera.process_mouse_movement(x_delta as f32, -y_delta as f32, true),
                events::Event::WindowResized { width, height, .. } => framebuffer_size_callback(width, height),
                _ => (),
            }
        }

        let time = sdl.get_ticks() as f32 / 10_000.0_f32;
        let delta_time = time - last_time;
        last_time = time;

        process_input(&keys_held, &mut camera, delta_time);

        opengl::clear_color(0.2, 0.3, 0.3, 1.0);
        opengl::clear(ColorBuffer | DepthBuffer);

        shader_program.bind();

        let projection = perspective_gl(camera.zoom.to_radians(), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
        shader_program.set_mat4("projection", projection);

        let view = camera.get_view_matrix();
        shader_program.set_mat4("view", view);

        let text_translation = Mat4::from_translation(Vec3::new(0.1, -1.5, 0.0));
        // TODO: It needs orthogonal projection so that actual screen pixel positions can be used

        scene.draw(&shader_program, view, projection);
        font.draw(&shader_program_font, format!("X: {} Y: {} Z: {}", camera.position.x, camera.position.y, camera.position.z).as_str(), 32.0, text_translation);

        win.swap_window();
    }
}
