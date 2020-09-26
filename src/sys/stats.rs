use crate::comp::actor;
use crate::comp::physics;
use crate::comp::stats;
use crate::res;

use bevy::prelude::*;   
use bevy::sprite::collide_aabb::Collision;

pub fn collider_contact_system(
    collision_events: Res<Events<res::GroundCollisionEvent>>,
    mut collision_event_reader: ResMut<res::GroundContactListenerState>,
    mut query: Query<(
        &actor::Player, 
        &mut stats::Grounded,
        &physics::ColliderBox, 
        &mut Transform, 
        &mut physics::GravitationalAttraction,
        &mut physics::CollisionData,
    )>,
) {
    for (
        _player, 
        mut grounded, 
        body, 
        mut transform, 
        mut attraction, 
        mut collision_data
    ) in &mut query.iter() {
        attraction.is_active = true;
        collision_data.reset();

        for event in collision_event_reader.event_reader.iter(&collision_events) {
            collision_data.reset();
            collision_data.below = set_grounded_if_needed(
                &event,
                &body.get_size(),
                &mut transform,
                &mut grounded,
                &mut attraction,
            );
        }
    }
}

fn set_grounded_if_needed(
    event: &res::GroundCollisionEvent,
    size: &Vec2,
    transform: &mut Transform, 
    grounded: &mut stats::Grounded, 
    attraction: &mut physics::GravitationalAttraction
) -> bool {
    match event.hit_collision {
        Collision::Left | Collision::Right => {
            if is_between(
                transform.translation().y() - size.y() / 2.,
                event.hit_transform.translation().y() + event.hit_size.y() / 2. - 2.,
                event.hit_transform.translation().y() + event.hit_size.y() / 2. - 10.
            ) { 
                return false
            }
        },
        Collision::Bottom => {
            if transform.translation().y() - size.y() / 2. <= event.hit_transform.translation().y() + event.hit_size.y() / 2. - 10. {
                return false
            }
        }
        _ => {}
    }

    let mut translation = transform.translation();
    *translation.y_mut() = event.hit_transform.translation().y() + event.hit_size.y() / 2. + size.y() / 2. + 0.1;
    transform.set_translation(translation);
                
    grounded.0 = true;
    attraction.is_active = false;

    true // Yes, we are grounded
}

fn is_between(value: f32, min: f32, max: f32) -> bool { 
    value < min && value < max
}