use std::collections::HashSet;

use beryllium::*;
use beryllium::events::{SDLK_a, SDLK_d, SDLK_ESCAPE, SDLK_s, SDLK_w};
use beryllium::events::SDL_Keycode;
use beryllium::video::GlSwapInterval::Vsync;
use ogl33::glViewport;
use ultraviolet::{Mat4, Vec3, Vec4};
use ultraviolet::projection::perspective_gl;

use crate::camera::{Camera, PITCH, YAW};
use crate::camera::CameraMovement;
use crate::graphics::model::Model;
use crate::opengl::Capability;
use crate::opengl::ClearBitFlags::{ColorBuffer, DepthBuffer};
use crate::shader::Shader;

mod shader;
mod camera;
mod opengl;
mod graphics;


const SCR_WIDTH: i32 = 800;
const SCR_HEIGHT: i32 = 600;

const CUBE_POSITIONS: [Vec3; 10] = [
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(2.0, 5.0, -15.0),
    Vec3::new(-1.5, -2.2, -2.5),
    Vec3::new(-3.8, -2.0, -12.3),
    Vec3::new(2.4, -0.4, -3.5),
    Vec3::new(-1.7, 3.0, -7.5),
    Vec3::new(1.3, -2.0, -2.5),
    Vec3::new(1.5, 2.0, -2.5),
    Vec3::new(1.5, 0.2, -1.5),
    Vec3::new(-1.3, 1.0, -1.5),
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
    let mut camera: Camera = Camera::from_vec3(Vec3::new(0.0, 0.0, 3.0), Vec3::new(0.0, 1.0, 0.0), YAW, PITCH);

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

    let container_model = Model::load_from_file("res/models/cottage.obj");

    let shader_program = Shader::from_files("src/6.3.shader.vs", "src/6.3.shader.fs");

    shader_program.bind();
    shader_program.set_int("texture1", 0);
    shader_program.set_int("texture2", 1);


    let mut last_time = 0.0;
    let mut keys_held = HashSet::new();

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

        for (i, cube_pos) in CUBE_POSITIONS.iter().enumerate() {
            let mut model = Mat4::from_translation(*cube_pos);
            let angle = (20.0f32 * i as f32).to_radians();


            model = model * Mat4::from_scale(0.05) * Mat4::from_rotation_around(Vec4::new(1.0, 0.0, 0.0, 1.0), angle) * Mat4::from_rotation_around(Vec4::new(0.0, 1.0, 0.0, 1.0), angle) * Mat4::from_rotation_around(Vec4::new(0.0, 0.0, 1.0, 1.0), angle);

            shader_program.set_mat4("model", model);

            container_model.draw(&shader_program);
        }

        win.swap_window();
    }
}
