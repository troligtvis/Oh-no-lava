use bevy::prelude::*;

use crate::comp::{self, physics, stats};
use crate::res;
use bevy::sprite::collide_aabb::{collide, Collision};

const FALL_MULTIPLIER: f32 = 2.5;
const LOW_JUMP_MULTIPLIER: f32 = 2.;

pub struct GamePhysicsPlugin;

impl Plugin for GamePhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<res::GroundCollisionEvent>()
            .add_system(process_velocity_system.system())
            .add_system(drag_system.system())
            .add_system(gravity_system.system())
            .add_stage_after(stage::PRE_UPDATE, "stage::GroundCheck")
            .add_system_to_stage("stage::GroundCheck", player_collision_system.system())
            .add_stage_after(stage::PRE_UPDATE, "stage::Raycast")
            .add_system_to_stage("stage::Raycast", update_raycast.system())
            .add_system_to_stage("stage::Raycast", shoot_raycast.system())
            .add_system(adjust_jump_system.system());
    }
}

/// Move entities with Velocity components
pub fn process_velocity_system(
    time: Res<Time>,
    mut query: Query<(&physics::Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in &mut query.iter() {
        transform.translate(velocity.0.extend(0.) * time.delta_seconds);
    }
}

pub fn drag_system(
    time: Res<Time>, 
    mut query: Query<(&mut physics::Velocity, &physics::Drag)>) {
    for (mut velocity, drag) in &mut query.iter() {
        *velocity = physics::Velocity(velocity.lerp(Vec2::zero(), time.delta_seconds * drag.0));
    }
}

pub fn gravity_system(
    gravity: Res<physics::Gravity>,
    time: Res<Time>,
    attraction: &physics::GravitationalAttraction,
    mut velocity: Mut<physics::Velocity>,
) {
    if attraction.is_active {
        *velocity.0.y_mut() -= gravity.0 * time.delta_seconds;
    } else {        
        *velocity.0.y_mut() = 0.;
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

fn player_collision_system(
    mut collision_events: ResMut<Events<res::GroundCollisionEvent>>,
    mut query_1: Query<(
        &comp::actor::Player, 
        &comp::physics::ColliderBox,
        &mut Transform
    )>,
    mut query_2: Query<(
        &comp::physics::ColliderBox,
        Without<comp::actor::Player, &Transform>,
        &comp::physics::Velocity,
    )>,
) {
    for (_, body, transform) in &mut query_1.iter() {
        for (other_body, other_transform, other_velocity) in &mut query_2.iter() {
            let mut translation = transform.translation().clone();
            *translation.y_mut() -= 1.;

            let collision = collide(
                other_transform.translation(), 
                other_body.get_size(), 
                translation,
                body.get_size(),
            );

            if let Some(collision) = collision {
                collision_events.send(res::GroundCollisionEvent {
                    hit_collision: collision,
                    hit_transform: other_transform.clone(),
                    hit_size: other_body.get_size(),
                    hit_velocity: other_velocity.clone(),
                });
            }
        }
    }
}

pub fn update_raycast(
    mut query: Query<(With<comp::actor::Player, &Transform>, &mut comp::physics::Raycast)>
) {
    for (transform, mut raycast) in &mut query.iter() {
        raycast.origin = transform.translation().truncate();
    }   
}

pub fn shoot_raycast(
    mut query1: Query<(
        With<comp::actor::Player, &mut Transform>, 
        &comp::physics::Raycast, 
        &mut physics::GravitationalAttraction, 
        &comp::physics::ColliderBox,
        &comp::stats::Facing,
        &mut comp::physics::CollisionData,
        &mut comp::physics::Velocity,
    )>,
    mut query2: Query<(With<stats::Wall, &Transform>, &physics::ColliderBox, &physics::Velocity)>,
) {
    for (
        mut transform, 
        raycast, 
        mut attraction, 
        p_box, 
        facing, 
        mut collision_data,
        mut velocity,
    ) in &mut query1.iter() {
        let mut position = raycast.origin.extend(0.);
        *position.x_mut() = position.x() + (p_box.get_size().x() / 2. * facing.0);

        let size = Vec2::new(12., 1.);
        
        for (other_transform, other_box, other_velocity) in &mut query2.iter() {
            if let Some(collision) = collide(position, size, other_transform.translation(), other_box.get_size()) {
                
                collision_data.right = false;
                collision_data.left = false;

                attraction.is_active = true;
                match collision {
                    Collision::Left => {
                        attraction.is_active = false;

                        let mut translation = transform.translation();

                        if translation.x() + p_box.get_size().x() / 2. > other_transform.translation().x() - other_box.get_size().x() / 2. {
                            *translation.x_mut() = other_transform.translation().x() - other_box.get_size().x() / 2. - p_box.get_size().x() / 2.;
                            transform.set_translation(translation);
                        }
                       
                        collision_data.right = true;

                        *velocity.0.x_mut() = other_velocity.0.x();
                    },
                    Collision::Right => {
                        attraction.is_active = false;

                        let mut translation = transform.translation();

                        if translation.x() - p_box.get_size().x() / 2. < other_transform.translation().x() + other_box.get_size().x() / 2. {
                            *translation.x_mut() = other_transform.translation().x() + other_box.get_size().x() / 2. + p_box.get_size().x() / 2.;
                            transform.set_translation(translation);
                        }

                        collision_data.left = true;

                        *velocity.0.x_mut() = other_velocity.0.x();
                    },
                    _ => {},
                };
            }
        }
    }
}