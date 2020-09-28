use crate::comp;
use crate::util::*;

use bevy::{
    prelude::*,
    input::{keyboard::KeyCode, Input},
    input::mouse::MouseButtonInput,
    window::CursorMoved,
};

const FALL_MULTIPLIER: f32 = 2.5;
const LOW_JUMP_MULTIPLIER: f32 = 2.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MouseState>()
            .add_system(handle_input_system.system())
            .add_system(adjust_jump_system.system());
    }
}

#[derive(Default)]
pub struct MouseState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

/// Converts real player input into Controller input
pub fn handle_input_system(
    windows: Res<Windows>,
    mut state: ResMut<MouseState>,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        With<comp::actor::Player, &mut comp::actor::Controller>, 
        &comp::physics::CollisionData,
        &comp::stats::Grounded,
    )>,
) {
    let window_size = get_window_size(windows);

    for (
        mut controller, 
        collision_data, 
        grounded
    ) in &mut query.iter() {
        if mouse_button_input.pressed(MouseButton::Left) { 
            controller.action
                .push_back(comp::actor::ControllerAction::Shoot);
        }

        for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
            let cursor_position = event.position - Vec2::new(window_size.width / 2., window_size.height / 2.);
            *controller.cursor_position.x_mut() = cursor_position.x();
            *controller.cursor_position.y_mut() = cursor_position.y();
        }

        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            *controller.movement.y_mut() += 1.0;
        }

        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            *controller.movement.y_mut() -= 1.0;
        }
        
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            *controller.movement.x_mut() -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            *controller.movement.x_mut() += 1.0;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            if collision_data.either_side() && !grounded.0 {
                controller.action
                    .push_back(comp::actor::ControllerAction::WallJump);
            } else {
                controller.action
                    .push_back(comp::actor::ControllerAction::Jump);
            }
        }
    }   
}

fn adjust_jump_system(
    time: Res<Time>,
    gravity: Res<comp::physics::Gravity>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &comp::actor::Player, 
        &mut comp::physics::Velocity, 
        &comp::physics::GravitationalAttraction
    )>,
) {
    let dt = time.delta_seconds;

    for (_player, mut velocity, affected) in &mut query.iter() {
        if !affected.is_active {
            break;
        }

        // Better jumping
        if velocity.0.y() < 0.0 {
            let vel = Vec2::unit_y() * -gravity.0 * (FALL_MULTIPLIER - 1.) * dt;
            velocity.0 += vel;
        } else if velocity.0.y() > 0.0 && !keyboard_input.pressed(KeyCode::Space) {
            let vel = Vec2::unit_y() * -gravity.0 * (LOW_JUMP_MULTIPLIER - 1.) * dt;
            velocity.0 += vel;
        }
    }
}