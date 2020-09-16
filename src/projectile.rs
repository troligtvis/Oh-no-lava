#![allow(dead_code)]

use crate::{bevy::prelude::*, util::*, GravitationalAttraction, Velocity};

use rand::{thread_rng, Rng};

pub struct Projectile {
    direction: Vec2,
    time_to_live: Timer,
}

#[derive(PartialEq)]
enum ProjectileState {
    Idle,
    Active,
    // dead,
}

pub struct Crosshair {
    // Aim direction
    pub aim: Vec2,
    // Distance from center of player
    pub distance: f32,
}

pub fn shoot_projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile, &mut Sprite)>,
) {
    for (entity, mut projectile, mut sprite) in &mut query.iter() {
        projectile.time_to_live.tick(time.delta_seconds);
        if !projectile.time_to_live.finished {
            let procentage =
                1. - (projectile.time_to_live.elapsed) / projectile.time_to_live.duration;
            sprite.size = Vec2::new(5.0, 5.0) * procentage;

            return;
        }
        println!("Despawn projectile");
        commands.despawn(entity);
    }
}

pub fn spawn_projectile_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<(&Crosshair, &Transform)>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    // TOdo add timer as well to not shoot too fast

    for (crosshair, transform) in &mut query.iter() {
        let direction = get_direction(&transform.translation().truncate(), &crosshair.aim);

        let projectile_velocity = direction.normalize() * 200.;

        let upper = 20.;
        let lower = -20.;
        let mut rng = thread_rng();
        let x = rng.gen_range(lower, upper);
        let y = rng.gen_range(lower, upper);

        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.1, 0.5, 0.8).into()),
                transform: Transform::from_translation(transform.translation().clone()),
                sprite: Sprite {
                    size: Vec2::new(5., 5.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Projectile {
                direction,
                time_to_live: Timer::from_seconds(2.0, true),
            })
            .with(ProjectileState::Active)
            .with(GravitationalAttraction::default())
            .with(Velocity(Vec2::new(
                projectile_velocity.x() + x,
                projectile_velocity.y() + y,
            )));
        //.with(Collider::Solid)
    }
}
