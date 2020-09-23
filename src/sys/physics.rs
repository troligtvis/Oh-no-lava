use bevy::prelude::*;

use crate::comp::{self, physics};
use crate::res;
use bevy::sprite::collide_aabb::{collide};

pub struct GamePhysicsPlugin;

impl Plugin for GamePhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<res::CollisionEvent>()
            .add_system(process_velocity_system.system())
            // .add_system(drag_system.system())
            .add_system(gravity_system.system())
            .add_stage_after(stage::PRE_UPDATE, "stage::GroundCheck")
            .add_system_to_stage("stage::GroundCheck", player_collision_system.system());
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
    mut collision_events: ResMut<Events<res::CollisionEvent>>,
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
                collision_events.send(res::CollisionEvent {
                    hit_collision: collision,
                    hit_transform: c_transform.clone(),
                    hit_size: c_body.get_size(),
                });
            }
        }
    }
}