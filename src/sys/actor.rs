use bevy::prelude::*;
use crate::comp::{actor, physics, stats};
use crate::res;
use crate::animation::{Animation, AnimCommonState, AnimStateDescriptor};
use crate::util;
use crate::sys;

use rand::{thread_rng, Rng};

pub struct GameActorPlugin;

impl Plugin for GameActorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<res::JumpEvent>()
            .add_event::<res::WallJumpEvent>()
            .add_event::<res::ShootEvent>()
            .init_resource::<res::JumpListenerState>()
            .init_resource::<res::WallJumpListenerState>()
            .init_resource::<res::ShootListenerState>()
            .add_system(process_commands_system.system())
            .add_system_to_stage(stage::EVENT_UPDATE, jump_system.system())
            .add_system_to_stage(stage::EVENT_UPDATE, wall_jump_system.system())            
            .add_system(process_crosshair_system.system())
            .add_system(shoot_projectile_system.system())
            .add_system_to_stage(stage::POST_UPDATE, clean_projectile_system.system());
            // .add_system(stretch_to_normal_system.system());
    }
}

// MARK - Systems

pub fn process_commands_system(
    mut jump_command_event: ResMut<Events<res::JumpEvent>>,
    mut wall_jump_command_event: ResMut<Events<res::WallJumpEvent>>,
    mut shoot_command_event: ResMut<Events<res::ShootEvent>>,
    mut animation: ResMut<Animation>,
    mut query: Query<(
        &mut actor::Controller,    
        &mut physics::Velocity,
        &mut Transform,
        &stats::MovementSpeed,
        &mut stats::Facing,
        &physics::CollisionData,
    )>,
) {
    for (
        mut controller, 
        mut velocity, 
        mut transform, 
        speed, 
        mut facing,
        _collision_data,
    ) in &mut query.iter() {
        let movement = if controller.movement.x() + controller.movement.y() != 0.0 {
            controller.movement.normalize()
        } else {
            controller.movement
        };

        let facing_direction =  clamp(movement.x(), -1., 1.);
        if facing_direction != 0. {
            facing.0 = facing_direction;

            flip_sprite(&mut transform, facing_direction);
        }

        for command in controller.action.drain(..) {
            match command {
                actor::ControllerAction::Shoot => {
                    shoot_command_event.send(res::ShootEvent);
                },
                actor::ControllerAction::Jump => {
                    jump_command_event.send(res::JumpEvent);
                },
                actor::ControllerAction::WallJump => {
                    wall_jump_command_event.send(res::WallJumpEvent);
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

        if movement.x().abs() > 0. {
            animation.set_anim(AnimCommonState::Run.name());
        } else {
            // TODO: - Figure out a way to detect riding on moving objects should work
            // with updating animations
            //if velocity.x().abs() < 8. || collision_data.either_side() {
                animation.set_anim(AnimCommonState::Idle.name());
            //}
        }

        controller.reset_movement();
    }
}

pub fn jump_system(
    jump_event: Res<Events<res::JumpEvent>>,
    mut jump_event_reader: ResMut<res::JumpListenerState>,
    mut query: Query<(
        With<actor::Player, &mut Transform>, 
        &mut physics::Velocity,
        &stats::JumpForce,
        &mut physics::GravitationalAttraction,
        &mut stats::Grounded,
        &mut stats::StretchTimer,
    )>,
) {
    for _event in jump_event_reader.event_reader.iter(&jump_event) {
        for (
            mut transform, 
            mut velocity, 
            jump_force, 
            mut attraction,
            mut grounded,
            mut stretch_timer,
        ) in &mut query.iter() {
            // Move the position of the player a bit up to 
            // avoid colliding with object before jumping
            let mut translation = transform.translation();
            *translation.y_mut() += 2.;
            transform.set_translation(translation);

            attraction.is_active = true;
            grounded.0 = false;

            velocity.0.set_y(jump_force.0);

            // stretch_sprite(&mut stretch_timer, &mut transform);
        }
    }
}

pub fn wall_jump_system(
    mut commands: Commands,
    mut materials: ResMut<res::ColorMaterialStorage>,
    event: Res<Events<res::WallJumpEvent>>,
    mut event_reader: ResMut<res::WallJumpListenerState>,
    mut query: Query<(
        With<actor::Player, &mut Transform>,
        &mut physics::Velocity,
        &stats::JumpForce,
        &mut physics::GravitationalAttraction,
        &mut physics::CollisionData,
        &physics::ColliderBox,
        &stats::Facing,
        &mut stats::StretchTimer,
    )>,
) {
    for _ in event_reader.0.iter(&event) {
        for (
            mut transform, 
            mut velocity, 
            force, 
            mut attraction,
            mut collision_data,
            body,
            facing,
            mut stretch_timer,
        ) in &mut query.iter() {
            let mut translation = transform.translation();
            let mut position = transform.translation().truncate();

            if collision_data.left {
                *translation.x_mut() += 8.;
                transform.set_translation(translation);

                attraction.is_active = true;

                velocity.0.set_x(force.0 * 1.4);                
                velocity.0.set_y(force.0 * 1.2);
                
                *position.x_mut() -= body.get_size().x() / 2.; 

                sys::particles::spawn_wall_dust_particle(
                    &mut commands, 
                    &mut materials, 
                    position, 
                    Vec2::new(1., 1.),  
                    facing.0,
                    10,
                );

                // stretch_sprite(&mut stretch_timer, &mut transform);

                collision_data.left = false;

            } else if collision_data.right {
                *translation.x_mut() -= 8.;
                transform.set_translation(translation);

                attraction.is_active = true;

                velocity.0.set_x(-force.0 * 1.4);                
                velocity.0.set_y(force.0 * 1.2);

                *position.x_mut() += body.get_size().x() / 2.; 

                sys::particles::spawn_wall_dust_particle(
                    &mut commands, 
                    &mut materials, 
                    position, 
                    Vec2::new(-1., 1.),  
                    facing.0,
                    10,
                );

                // stretch_sprite(&mut stretch_timer, &mut transform);

                collision_data.right = false;
            }
        }
    }
}

fn stretch_to_normal_system(
        time: Res<Time>,
        mut query: Query<(With<actor::Player, &mut Transform>, &mut stats::StretchTimer)>
    ) {
        for (mut transform, mut timer) in &mut query.iter() {
            timer.0.tick(time.delta_seconds);
    
            if timer.0.finished {
                transform.set_non_uniform_scale(Vec3::one());
            }
    }
}

fn stretch_sprite(
    timer: &mut stats::StretchTimer,
    transform: &mut Transform,
) {
    timer.0.reset();
    timer.0.duration = 0.4;
                
    transform.set_non_uniform_scale(Vec3::new(0.9, 1.1, 1.)); 
}

/// Spawn and shoot proectile
pub fn shoot_projectile_system(
    mut commands: Commands,
    materials: ResMut<res::ColorMaterialStorage>,
    shoot_event: Res<Events<res::ShootEvent>>,
    mut shoot_event_reader: ResMut<res::ShootListenerState>,
    mut query_1: Query<With<actor::Crosshair, &Transform>>,
    mut query_2: Query<With<actor::Player, &Transform>>
) {
    for _event in shoot_event_reader.event_reader.iter(&shoot_event) {
        for transform in &mut query_1.iter() {
            for other_transform in &mut query_2.iter() {
                let direction = util::get_direction(&other_transform.translation().truncate(), &transform.translation().truncate());
                let projectile_velocity = direction.normalize() * 200.; // TODO - remove magic value

                let upper = 20.;
                let lower = -20.;
                let mut rng = thread_rng();
                let x = rng.gen_range(lower, upper);
                let y = rng.gen_range(lower, upper);

                commands
                    .spawn(SpriteComponents {
                        material: *materials.storage.get(&"Projectile".to_string()).unwrap(),
                        transform: Transform::from_translation(transform.translation().clone()),
                        sprite: Sprite {
                            size: Vec2::new(5., 5.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with(actor::Projectile {
                        direction,
                    })
                    .with(stats::TimeToLive(Timer::from_seconds(2.0, true)))
                    .with(physics::GravitationalAttraction::default())
                    .with(physics::Velocity(Vec2::new(
                        projectile_velocity.x() + x,
                        projectile_velocity.y() + y,
                    )));
            }
        }
    }
}

/// Shrink sprite over time then despawn it
/// In case of out of bounds it will get despawned early
pub fn clean_projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    windows: Res<Windows>,
    mut query: Query<(
        Entity, 
        With<actor::Projectile, &mut stats::TimeToLive>, 
        &mut Sprite, 
        &Transform
    )>,
) {
    let size = util::get_window_size(windows);
    for (entity, mut timer, mut sprite, transform) in &mut query.iter() {
        // Check if still within bounds
        if transform.translation().y() < -size.height {
            println!("Despawn projectile, out of bounds");
            commands.despawn(entity);
            
            return;
        }

        timer.0.tick(time.delta_seconds);
        if !timer.0.finished {
            let procentage =
                1. - (timer.0.elapsed) / timer.0.duration;
            sprite.size = Vec2::new(5.0, 5.0) * procentage;

            return;
        }

        println!("Despawn projectile, after time");
        commands.remove_one::<Draw>(entity);
        commands.despawn(entity);
    }
}

pub fn process_crosshair_system(
    mut query_1: Query<(&actor::Player, &actor::Controller, &Transform)>,
    mut query_2: Query<(&actor::Crosshair, &mut Transform,)>,
) {
    for (_, controller, player_transform) in &mut query_1.iter() {
        for (crosshair, mut crosshair_transform) in &mut query_2.iter() {            
            set_aim(
                &player_transform.translation().truncate(),
                &controller.cursor_position,
                crosshair.distance,
                &mut crosshair_transform,
            );
        }
    }
}

// MARK - Helper functions

/// Flip the transform depending on facing direction
fn flip_sprite(
    transform: &mut Transform,
    direction: f32,
) {
    let pi = std::f32::consts::PI;
    let rotation = if direction > 
    0. {
        Quat::identity()
    } else {
        Quat::from_rotation_y(pi)
    };

    transform.set_rotation(rotation);
}

/// Clamp value betwen min and max
fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value > 0. {
        max
    } else if value < 0. {
        min
    } else {
        0.
    }
}

fn set_aim(a: &Vec2, b: &Vec2, distance: f32, transform: &mut Transform) {
    let direction = util::get_direction(a, b);
    let norm = direction.normalize() * distance;
    let aim = Vec2::new(a.x() + norm.x(), a.y() + norm.y()).extend(0.);

    transform.set_translation(aim);
}