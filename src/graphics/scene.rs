use std::collections::HashSet;

use beryllium::events::{SDL_Keycode, SDLK_a, SDLK_d, SDLK_LCTRL, SDLK_s, SDLK_SPACE, SDLK_w};
use rapier3d::control::KinematicCharacterController;
use rapier3d::prelude::*;
use ultraviolet::{Mat4, Vec3};
use ultraviolet::projection::perspective_gl;

use crate::{SCR_HEIGHT, SCR_WIDTH};
use crate::graphics::player_character::{MovementDirection, PlayerCharacter};
use crate::graphics::skybox::Skybox;
use crate::graphics::static_body_3d::StaticBody3D;
use crate::graphics::true_type_font::TrueTypeFont;
use crate::shader::Shader;

struct Physics {
    pub gravity: Vector<Real>,
    pub integration_parameters: IntegrationParameters,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub physics_hooks: (),
    pub event_handler: (),
    pub player_body_handle: RigidBodyHandle,
    pub character_controller: KinematicCharacterController,
}

pub struct Scene<'a> {
    static_bodies: Vec<StaticBody3D>,
    skybox: Option<Skybox>,
    player: PlayerCharacter,
    font: TrueTypeFont<'a>,
    physics: Physics,
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

        // Physics setup
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let ground_height = 0.1;
        let ground_size = 1000.0;

        let rigid_body_ground = RigidBodyBuilder::fixed().translation(vector![0.0, -ground_height, 0.0]);
        let ground_handle = rigid_body_set.insert(rigid_body_ground);

        let ground_collider = ColliderBuilder::cuboid(
            ground_size,
            ground_height,
            ground_size,
        );

        collider_set.insert_with_parent(ground_collider, ground_handle, &mut rigid_body_set);

        let rigid_body = RigidBodyBuilder::kinematic_position_based()
            .translation(vector![player.get_position().x, player.get_position().y, player.get_position().z])
            .build();
        let collider = ColliderBuilder::capsule_y(1.1, 0.5);
        let player_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, player_body_handle, &mut rigid_body_set);

        let gravity = vector![0.0, -9.81, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let event_handler = ();
        let character_controller = KinematicCharacterController::default();

        let physics = Physics {
            gravity,
            integration_parameters,
            rigid_body_set,
            collider_set,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            physics_hooks,
            event_handler,
            player_body_handle,
            character_controller,
        };

        Self {
            static_bodies,
            skybox,
            player,
            font,
            physics,
        }
    }
    pub fn update(&mut self, delta_time: f32, held_keys: &HashSet<SDL_Keycode>, mouse_delta: Option<(i32, i32)>) {
        if let Some(delta) = mouse_delta { self.player.process_mouse_movement(delta.0 as f32, delta.1 as f32, true) }

        let player_body = self.physics.rigid_body_set.get(self.physics.player_body_handle).unwrap();
        let player_collider = self.physics.collider_set.get(player_body.colliders()[0]).unwrap();

        let mut desired_movement = Vector::zeros();

        let look_direction = self.player.get_look_direction();
        let mut move_direction = Vector::zeros();
        move_direction.x = look_direction.x;
        move_direction.z = look_direction.z;
        if held_keys.contains(&SDLK_w) {
            desired_movement += move_direction;
        } else if held_keys.contains(&SDLK_s) {
            desired_movement -= move_direction;
        }

        desired_movement *= 0.05;
        desired_movement -= Vector::y() * 0.05;

        let mut collisions = vec![];
        let corrected_movement = self.physics.character_controller.move_shape(
            self.physics.integration_parameters.dt,              // The timestep length (can be set to SimulationSettings::dt).
            &self.physics.rigid_body_set,         // The RigidBodySet.
            &self.physics.collider_set,      // The ColliderSet.
            &self.physics.query_pipeline,        // The QueryPipeline.
            player_collider.shape(), // The character’s shape.
            player_body.position(),   // The character’s initial position.
            desired_movement.cast::<Real>(),
            QueryFilter::new()
                // Make sure the character we are trying to move isn’t considered an obstacle.
                .exclude_rigid_body(self.physics.player_body_handle),
            |collision| collisions.push(collision), // We don’t care about events in this example.
        );

        for collision in &collisions {
            self.physics.character_controller.solve_character_collision_impulses(
                self.physics.integration_parameters.dt,
                &mut self.physics.rigid_body_set,
                &self.physics.collider_set,
                &self.physics.query_pipeline,
                player_collider.shape(),
                player_collider.mass(),
                collision,
                QueryFilter::new().exclude_rigid_body(self.physics.player_body_handle),
            )
        }

        let player_body = &mut self.physics.rigid_body_set[self.physics.player_body_handle];
        let pos = player_body.position();

        player_body.set_next_kinematic_translation(pos.translation.vector + corrected_movement.translation);

        // NOTE: this call only moves dynamic bodies, kinetic bodies (like player) are unaffected
        self.physics.physics_pipeline.step(&self.physics.gravity, &self.physics.integration_parameters,
                                           &mut self.physics.island_manager,
                                           &mut self.physics.broad_phase,
                                           &mut self.physics.narrow_phase,
                                           &mut self.physics.rigid_body_set,
                                           &mut self.physics.collider_set,
                                           &mut self.physics.impulse_joint_set,
                                           &mut self.physics.multibody_joint_set,
                                           &mut self.physics.ccd_solver,
                                           Some(&mut self.physics.query_pipeline),
                                           &self.physics.physics_hooks,
                                           &self.physics.event_handler);


        let pos = self.physics.rigid_body_set[self.physics.player_body_handle].position();
        self.player.set_position(pos.translation.x, pos.translation.y, pos.translation.z);

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