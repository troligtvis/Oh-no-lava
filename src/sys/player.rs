use bevy::prelude::*;

use crate::comp::{
    actor::{Player, Controller, MovementAction, Jump}, 
    physics::{GravitationalAttraction, Velocity}
};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<Jump>()
        .add_system(handle_input)
        .add_system(jump_listener);
    }
}

fn handle_input(
    //time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut jump_event: EventWriter<Jump>,
    mut query: Query<(With<Player>, &mut Controller, &mut Transform)>
) {
    for (_, mut controller, mut transform) in &mut query {
        // if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        //     transform.translation.y += 1.0; // * time.delta_seconds();
        // }

        // if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        //     transform.translation.y -= 1.0;
        // }
        
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 1.0;
            println!("Go left");
        }

        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 1.0;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            jump_event.send(Jump::default());
            //controller.movement_action.push_back(MovementAction::Jump);
        }
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
            println!("Jump!");
            gravitational_attraction.is_active = false;
            
            velocity.0.y = 200.;
        }
    }
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