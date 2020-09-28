use bevy::prelude::*;

use crate::comp::{self, physics, stats};
use crate::res;
use bevy::sprite::collide_aabb::{collide, Collision};

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
            .add_system_to_stage("stage::Raycast", shoot_raycast.system());
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
        *velocity = physics::Velocity(velocity.lerp(Vec2::new(0.0, 0.0), time.delta_seconds * drag.0));
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

fn player_collision_system(
    mut collision_events: ResMut<Events<res::GroundCollisionEvent>>,
    mut query_1: Query<(
        &comp::actor::Player, 
        &comp::physics::ColliderBox,
        &mut Transform
    )>,
    mut query_2: Query<(
        &comp::physics::ColliderBox,
        Without<comp::actor::Player, &Transform>
    )>,
) {
    for (_, p_body, p_transform) in &mut query_1.iter() {
        for (c_body, c_transform) in &mut query_2.iter() {
            let mut translation = p_transform.translation().clone();
            *translation.y_mut() -= 1.;

            let collision = collide(
                c_transform.translation(), 
                c_body.get_size(), 
                translation,
                p_body.get_size(),
            );

            if let Some(collision) = collision {
                collision_events.send(res::GroundCollisionEvent {
                    hit_collision: collision,
                    hit_transform: c_transform.clone(),
                    hit_size: c_body.get_size(),
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
    )>,
    mut query2: Query<(With<stats::Wall, &Transform>, &physics::ColliderBox)>,
) {
    for (
        mut transform, 
        raycast, 
        mut attraction, 
        p_box, 
        facing, 
        mut collision_data
    ) in &mut query1.iter() {
        let mut position = raycast.origin.extend(0.);
        *position.x_mut() = position.x() + (p_box.get_size().x() / 2. * facing.0);

        let size = Vec2::new(12., 1.);
        
        for (other_transform, other_box) in &mut query2.iter() {
            if let Some(collision) = collide(position, size, other_transform.translation(), other_box.get_size()) {
                
                attraction.is_active = true;
                match collision {
                    Collision::Left => {
                        attraction.is_active = false;

                        //println!("Raycast Left, {}", time.delta_seconds); 
                        let mut translation = transform.translation();

                        if translation.x() + p_box.get_size().x() / 2. > other_transform.translation().x() - other_box.get_size().x() / 2. {
                            *translation.x_mut() = other_transform.translation().x() - other_box.get_size().x() / 2. - p_box.get_size().x() / 2.;
                            transform.set_translation(translation);
                        }
                       
                        collision_data.right = true;
                    },
                    Collision::Right => {
                        attraction.is_active = false;

                        //println!("Raycast Right, {}", time.delta_seconds);

                        let mut translation = transform.translation();

                        if translation.x() - p_box.get_size().x() / 2. < other_transform.translation().x() + other_box.get_size().x() / 2. {
                            *translation.x_mut() = other_transform.translation().x() + other_box.get_size().x() / 2. + p_box.get_size().x() / 2.;
                            transform.set_translation(translation);
                        }

                        collision_data.left = true;
                    },
                    _ => {},
                };
            }
        }
    }
}