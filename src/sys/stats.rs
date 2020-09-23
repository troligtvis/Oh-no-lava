use crate::comp::actor;
use crate::comp::physics;
use crate::comp::stats;
use crate::res;

use bevy::prelude::*;   
use bevy::sprite::collide_aabb::Collision;

pub fn collider_contact_system(
    collision_events: Res<Events<res::CollisionEvent>>,
    mut collision_event_reader: ResMut<res::ContactListenerState>,
    mut query: Query<(
        &actor::Player, 
        &mut stats::Grounded,
        &physics::ColliderBox, 
        &mut Transform, 
        &mut physics::GravitationalAttraction,
        &mut physics::CollisionData,
    )>,
) {
    for (_player, mut grounded, body, mut transform, mut attraction, mut collision_data) in &mut query.iter() {
        attraction.is_active = true;
        collision_data.reset();

        for event in collision_event_reader.event_reader.iter(&collision_events) {
            let hit_translation = event.hit_transform.translation();
            let mut translation = transform.translation();
            let size = body.get_size();
            let hit_size = event.hit_size;

            collision_data.reset();

            collision_data.below = set_grounded_if_needed(
                &event,
                &size,
                &mut transform,
                &mut grounded,
                &mut attraction,
            );

            match event.hit_collision {
                Collision::Left => {
                    if translation.y() - size.y() < hit_translation.y() + hit_size.y() / 2. {
                        if translation.x() - size.x() / 2. < hit_translation.x() + hit_size.x() / 2. - 0.1 {
                            *translation.x_mut() = hit_translation.x() + hit_size.x() / 2. + size.x() / 2.;
                            transform.set_translation(translation);
                        }
                    }

                    collision_data.left = true
                },
                Collision::Right => {
                    if translation.y() - size.y() < hit_translation.y() + hit_size.y() / 2. {
                        if translation.x() + size.x() / 2. > hit_translation.x() - hit_size.x() / 2. + 1. {
                            *translation.x_mut() = hit_translation.x() - hit_size.x() / 2. - size.x() / 2.;
                            transform.set_translation(translation);
                        }                        
                    }
                    collision_data.right = true
                },
                Collision::Bottom => {
                    // if transform.translation().y() - size.y() / 2. <= hit_translation.y() + hit_size.y() / 2. - 10. {
                    //     if translation.y() - size.y() < hit_translation.y() + hit_size.y() / 2. {
                    //         if translation.x() - size.x() / 2. < hit_translation.x() + hit_size.x() / 2. - 0.1 {
                    //             *translation.x_mut() = hit_translation.x() + hit_size.x() / 2. + size.x() / 2.;
                    //             transform.set_translation(translation);
                                
                    //             println!("Right");
                    //         }
                    //     }
                        
                    //     if translation.y() - size.y() < hit_translation.y() + hit_size.y() / 2. {
                    //         if translation.x() + size.x() / 2. > hit_translation.x() - hit_size.x() / 2. + 1. {
                    //             *translation.x_mut() = hit_translation.x() - hit_size.x() / 2. - size.x() / 2.;
                    //             transform.set_translation(translation);
                                
                    //             println!("Left");
                    //         }
                    //     }                        
                    // }

                    if transform.translation().y() - size.y() / 2. <= hit_translation.y() + hit_size.y() / 2. - 10. {
                        if is_in_range(
                            translation.x() + size.x() / 2.,
                            hit_translation.x() - hit_size.x() / 2. - 3.,
                            hit_translation.x() - hit_size.x() / 2. + 3.
                        ) {
                            println!("Right");
                            *translation.x_mut() = hit_translation.x() - hit_size.x() / 2. - size.x() / 2.;
                            transform.set_translation(translation);
                            collision_data.right = true
                        } else 

                        if is_between(
                            translation.x() - size.x() / 2.,
                            hit_translation.x() + hit_size.x() / 2. - 1.,
                            hit_translation.x() + hit_size.x() / 2. - 3.
                        ) {
                            println!("Left");
                            *translation.x_mut() = hit_translation.x() + hit_size.x() / 2. + size.x() / 2.;
                            transform.set_translation(translation);
                            collision_data.left = true

                        }
                    }
                },
                _ => {}
            };
        }
    }
}

fn set_grounded_if_needed(
    event: &res::CollisionEvent,
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

fn is_in_range(value: f32, min: f32, max: f32) -> bool {
    value > min && value < max
}