use bevy::prelude::*;

use crate::{comp::{
    actor::{Player, Controller, Action, Jump}, 
    physics::{GravitationalAttraction, Velocity}, stats::{Facing, MovementSpeed}
}, animation::{Animation, AnimationState}};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<Jump>()
        .add_system(handle_input)
        .add_system(process_input)
        .add_system_to_stage(CoreStage::PreUpdate, jump_listener);
    }
}

fn handle_input(
    //time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    // mut jump_event: EventWriter<Jump>,
    mut query: Query<(With<Player>, &mut Controller)>
) {
    for (_, mut controller) in &mut query {
        // if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        //     transform.translation.y += 1.0; // * time.delta_seconds();
        // }

        // if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        //     transform.translation.y -= 1.0;
        // }
        
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            // transform.translation.x -= 1.0;
            controller.movement.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            // transform.translation.x += 1.0;
            controller.movement.x += 1.0;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            // jump_event.send(Jump::default());
            controller.movement_action.push_back(Action::Jump);
            //controller.movement_action.push_back(MovementAction::Jump);
        }
    } 
}

fn process_input(
    mut jump_event: EventWriter<Jump>,
    mut query: Query<(
        With<Player>, 
        &mut Controller, 
        &mut Velocity,
        &MovementSpeed,
        &mut Transform, 
        &mut Facing,
        &mut Animation,
    )>
) {

    for (_, mut controller, mut velocity, speed, mut transform, mut facing, mut animation) in query.iter_mut() {
        let movement = if controller.movement.x + controller.movement.y != 0.0 {
            controller.movement.normalize()
        } else {
            controller.movement
        };

        let facing_direction = clamp(movement.x, -1., 1.);
        if facing_direction != 0. {
            facing.0 = facing_direction;

            flip_sprite(&mut transform, facing_direction);
        };

        for action in controller.movement_action.drain(..) {
            match action {
                Action::Jump => jump_event.send(Jump::default()),
            }
        }

        if (velocity.0.x + movement.x * speed.accel).abs() < speed.max {
            velocity.0.x += movement.x * speed.accel;
        } else {
            if velocity.0.x < 0.0 {
                velocity.0.x = -speed.max;
            } else {
                velocity.0.x = speed.max;
            }
        }

        if movement.x.abs() > 0. {
            animation.set_animation(AnimationState::Run);
        } else {
            animation.set_animation(AnimationState::Idle);
        }
        
        controller.reset_movement();
    }
}

fn jump_listener(
    mut jump_event: EventReader<Jump>,
    mut query: Query<(With<Player>, &mut Transform, &mut GravitationalAttraction, &mut Velocity)>
) {
    for _ in jump_event.iter() {
        for (_, mut transform, mut gravitational_attraction, mut velocity) in query.iter_mut() {
            let mut translation = transform.translation;
            translation.y += 2.;
            transform.translation = translation;
            gravitational_attraction.is_active = false;
            
            velocity.0.y = 200.;
        }
    }
}

// TODO: convert to macro?
fn clamp(value: f32, min: f32, max: f32) -> f32 {
    match value {
        x if x > 0. => max,
        x if x < 0. => min,
        _ => 0.
    }
}

/// Flip the transform depending on facing direction
fn flip_sprite(
    transform: &mut Transform,
    direction: f32,
) {
    let pi = std::f32::consts::PI;
    let rotation = if direction > 
    0. {
        Quat::IDENTITY
    } else {
        Quat::from_rotation_y(pi)
    };

    transform.rotation = rotation;
}

// use crate::comp;
// use crate::util::*;

// use bevy::{
//     prelude::*,
//     input::{keyboard::KeyCode, Input},
//     input::mouse::MouseButtonInput,
//     window::CursorMoved,
// };

// pub struct PlayerPlugin;

// impl Plugin for PlayerPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.init_resource::<MouseState>()
//             .add_system(handle_input_system.system());
//     }
// }

// #[derive(Default)]
// pub struct MouseState {
//     mouse_button_event_reader: EventReader<MouseButtonInput>,
//     cursor_moved_event_reader: EventReader<CursorMoved>,
// }

// /// Converts real player input into Controller input
// pub fn handle_input_system(
//     windows: Res<Windows>,
//     mut state: ResMut<MouseState>,
//     mouse_button_input: Res<Input<MouseButton>>,
//     cursor_moved_events: Res<Events<CursorMoved>>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(
//         With<comp::actor::Player, &mut comp::actor::Controller>, 
//         &comp::physics::CollisionData,
//         &comp::stats::Grounded,
//     )>,
// ) {
//     let window_size = get_window_size(windows);

//     for (
//         mut controller, 
//         collision_data, 
//         grounded
//     ) in query.iter_mut() {
//         if mouse_button_input.pressed(MouseButton::Left) { 
//             controller.action
//                 .push_back(comp::actor::ControllerAction::Shoot);
//         }

//         for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
//             let cursor_position = event.position - Vec2::new(window_size.width / 2., window_size.height / 2.);
//             *controller.cursor_position.x_mut() = cursor_position.x();
//             *controller.cursor_position.y_mut() = cursor_position.y();
//         }

//         if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
//             *controller.movement.y_mut() += 1.0;
//         }

//         if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
//             *controller.movement.y_mut() -= 1.0;
//         }
        
//         if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
//             *controller.movement.x_mut() -= 1.0;
//         }

//         if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
//             *controller.movement.x_mut() += 1.0;
//         }

//         if keyboard_input.just_pressed(KeyCode::Space) {
//             if collision_data.either_side() && !grounded.0 {
//                 controller.action
//                     .push_back(comp::actor::ControllerAction::WallJump);
//             } else {
//                 controller.action
//                     .push_back(comp::actor::ControllerAction::Jump);
//             }
//         }
//     }   
// }