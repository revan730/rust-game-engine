use std::collections::HashSet;

use beryllium::events::{SDL_Keycode, SDLK_a, SDLK_d, SDLK_LCTRL, SDLK_s, SDLK_SPACE, SDLK_w};
use ultraviolet::{Mat4, Vec3};
use ultraviolet::projection::perspective_gl;

use crate::{SCR_HEIGHT, SCR_WIDTH};
use crate::graphics::player_character::{MovementDirection, PlayerCharacter};
use crate::graphics::skybox::Skybox;
use crate::graphics::static_body_3d::StaticBody3D;
use crate::graphics::true_type_font::TrueTypeFont;
use crate::shader::Shader;

pub struct Scene<'a> {
    static_bodies: Vec<StaticBody3D>,
    skybox: Option<Skybox>,
    player: PlayerCharacter,
    font: TrueTypeFont<'a>,
    // TODO: Gui?
    // TODO: lights
    // TODO: particles
}

fn process_input(pressed_keys: &HashSet<SDL_Keycode>, player: &mut PlayerCharacter, dt: f32) {
    if pressed_keys.contains(&SDLK_w) {
        player.process_keyboard(MovementDirection::Forward, dt);
    }

    if pressed_keys.contains(&SDLK_s) {
        player.process_keyboard(MovementDirection::Backward, dt);
    }

    if pressed_keys.contains(&SDLK_a) {
        player.process_keyboard(MovementDirection::Left, dt);
    }

    if pressed_keys.contains(&SDLK_d) {
        player.process_keyboard(MovementDirection::Right, dt);
    }

    if pressed_keys.contains(&SDLK_SPACE) {
        player.process_keyboard(MovementDirection::Up, dt);
    }

    if pressed_keys.contains(&SDLK_LCTRL) {
        player.process_keyboard(MovementDirection::Down, dt);
    }
}

impl Scene<'_> {
    pub fn new(static_bodies: Vec<StaticBody3D>, skybox: Option<Skybox>, player: PlayerCharacter) -> Self {
        let font = TrueTypeFont::load_from_file("res/fonts/futura.ttf");

        Self {
            static_bodies,
            skybox,
            player,
            font,
        }
    }

    pub fn update(&mut self, delta_time: f32, held_keys: &HashSet<SDL_Keycode>, mouse_delta: Option<(i32, i32)>) {
        if let Some(delta) = mouse_delta { self.player.process_mouse_movement(delta.0 as f32, delta.1 as f32, true) }


        let look_direction = self.player.get_look_direction();
        let right_direction = self.player.get_right_direction();
        let forward_direction = Vec3::new(look_direction.x, 0.0, look_direction.z);

        let mut desired_movement = Vec3::default();
        let speed = 10.0;
        let gravity = 0.981;

        if held_keys.contains(&SDLK_w) {
            desired_movement += forward_direction;
        } else if held_keys.contains(&SDLK_s) {
            desired_movement -= forward_direction;
        }

        if held_keys.contains(&SDLK_a) {
            desired_movement -= right_direction;
        } else if held_keys.contains(&SDLK_d) {
            desired_movement += right_direction;
        }

        desired_movement *= speed * delta_time;

        let collides_with_ground = self.player.check_collision(self.static_bodies[0].bounding_box);

        if collides_with_ground && held_keys.contains(&SDLK_SPACE) {
            self.player.add_vertical_velocity(1.0 * 50.0 * delta_time);
        }

        let mut pos = self.player.get_position();
        if collides_with_ground && !held_keys.contains(&SDLK_SPACE) {
            pos.y = self.static_bodies[0].bounding_box.y_max + self.player.get_half_height();
            self.player.reset_vertical_velocity();
        } else {
            self.player.add_vertical_velocity(-1.0 * gravity * delta_time);
            pos.y += self.player.get_vertical_velocity();
        }

        pos += desired_movement;
        // TODO: Detect direction of collision and push player out in this direction
        self.player.set_position(pos.x, pos.y, pos.z);

        // TODO: Update particles, lights, dynamic meshes (entities)
    }

    pub fn draw(&mut self, shader_program: &Shader, shader_program_font: &Shader) {
        shader_program.bind();

        let projection = perspective_gl(self.player.get_camera_zoom().to_radians(), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
        shader_program.set_mat4("projection", projection);

        let view = self.player.get_camera_view_matrix();
        shader_program.set_mat4("view", view);

        // TODO: Not sure if we need to pass shader from the outside or shaders will be loaded into scene
        for body in &self.static_bodies {
            body.draw(shader_program);
        }

        if self.skybox.as_ref().is_some() {
            self.skybox.as_ref().unwrap().draw(view, projection);
        }

        let text_translation = Mat4::from_translation(Vec3::new(0.1, -1.5, 0.0));
        // TODO: It needs orthogonal projection so that actual screen pixel positions can be used

        let player_pos = self.player.get_position();
        self.font.draw(shader_program_font, format!("X: {} Y: {} Z: {}", player_pos.x, player_pos.y, player_pos.z).as_str(), 32.0, text_translation);
    }
}