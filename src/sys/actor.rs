use bevy::prelude::*;
use crate::comp::{actor, physics, stats};
use crate::res;
use crate::animation::{Animation, AnimCommonState, AnimStateDescriptor};

pub struct GameActorPlugin;

impl Plugin for GameActorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<res::JumpEvent>()
            .init_resource::<res::JumpListenerState>()
            .add_system(process_commands_system.system())
            .add_stage_before(stage::PRE_UPDATE, "stage::Jumping")
            .add_system_to_stage("stage::Jumping", jump_system.system());
        //     .add_system(weapon_shoot_system.system())
        //     .add_system(weapon_reload_system.system());
    }
}

/// Flip the transform depending on facing direction
fn flip_sprite(
    transform: &mut Transform,
    direction: f32,
) {
    if direction == 0. {
        return
    }

    let pi = std::f32::consts::PI;
    let rotation = if direction > 
    0. {
        Quat::identity()
    } else {
        Quat::from_rotation_y(pi)
    };

    transform.set_rotation(rotation);
}

pub fn process_commands_system(
    mut jump_command_event: ResMut<Events<res::JumpEvent>>,
    mut animation: ResMut<Animation>,
    mut query: Query<(
        &mut actor::Controller,
        &mut physics::Velocity,
        &mut Transform,
        &stats::MovementSpeed,
    )>,
) {
    for (mut controller, mut velocity, mut transform, speed) in &mut query.iter() {
        let movement = if controller.movement.x() + controller.movement.y() != 0.0 {
            controller.movement.normalize()
        } else {
            controller.movement
        };

        flip_sprite(&mut transform, movement.x());

        for command in controller.action.drain(..) {
            match command {
                actor::ControllerAction::Shoot => {
                    // Add event
                },
                actor::ControllerAction::Jump => {
                    jump_command_event.send(res::JumpEvent);
                },
            }
        }

        // Now apply movement vector to input.
        // Cap horizontal speed.
        if (velocity.x() + movement.x() * speed.accel).abs() < speed.max {
            *velocity.x_mut() += movement.x() * speed.accel;
        } else {
            if velocity.x() < 0.0 {
                *velocity.x_mut() = -speed.max;
            } else {
                *velocity.x_mut() = speed.max;
            }
        }

        // TODO: Stop if no movement is happening
        if movement.x().abs() > 0. {
            animation.set_anim(AnimCommonState::Run.name());
        } else {
            animation.set_anim(AnimCommonState::Idle.name());
            *velocity.x_mut() = 0.;
        }

        controller.reset_movement();
    }
}

pub fn jump_system(
    jump_event: Res<Events<res::JumpEvent>>,
    mut jump_event_reader: ResMut<res::JumpListenerState>,
    mut query: Query<(
        &actor::Player, 
        &mut physics::Velocity,
        &stats::JumpForce,
        &mut Transform,
        &mut physics::GravitationalAttraction,
    )>,
) {
    for _event in jump_event_reader.event_reader.iter(&jump_event) {
        for (_, mut velocity, jump_force, mut transform, mut attraction) in &mut query.iter() {
            // Move the position of the player a bit up to 
            // avoid colliding with object before jumping
            let mut translation = transform.translation();
            *translation.y_mut() += 2.;
            transform.set_translation(translation);

            attraction.is_active = true;
            velocity.0.set_y(jump_force.0);
        }
    }
}

// pub fn process_crosshair_system(
//     mut query_1: Query<(&actor::Player, &Transform)>,
//     mut query_2: Query<(&actor::Crosshair, &mut Transform,)>,
// ) {
//     for (_, player_transform) in &mut query_1.iter() {
//         for (crosshair, crosshair_transform) in &mut query_2.iter() {            
//             set_aim(
//                 &player_transform.translation().truncate(),
//                 &crosshair.aim,
//                 crosshair.distance,
//                 &mut crosshair_transform,
//             );
//         }
//     }
// }

// fn set_aim(a: &Vec2, b: &Vec2, distance: f32, transform: &mut Transform) {
//     let direction = get_direction(a, b);
//     let norm = direction.normalize() * distance;
//     let aim = Vec2::new(a.x() + norm.x(), a.y() + norm.y()).extend(0.);

//     transform.set_translation(aim);
// }