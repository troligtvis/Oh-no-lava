#![allow(dead_code)]

use bevy::{self, prelude::*};

mod furniture;
mod player;
mod projectile;
mod util;
use util::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Lava Floor".to_string(),
            width: util::SCR_WIDTH as u32,
            height: util::SCR_HEIGHT as u32,
            resizable: false,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(furniture::FurniturePlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(setup.system())
        .add_resource(Gravity(9.82 * 40.))
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());

    // Ground
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            translation: Translation(Vec3::new(0., -SCR_HEIGHT / 2., 0.)),
            sprite: Sprite {
                size: Vec2::new(SCR_WIDTH, 20.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Velocity(Vec2::zero()))
        .with(Collider::Solid);
}

pub struct SpawnTimer {
    pub timer: Timer,
}

#[derive(PartialEq)]
enum Collider {
    Solid,
}

pub struct Despawnable;
pub struct Speed(f32);
pub struct Force(f32);
pub struct Velocity(pub Vec2);

pub struct Gravity(f32);

pub struct GravitationalAttraction {
    is_grounded: bool,
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(velocity_system.system())
            .add_system(gravity_system.system());
    }
}

fn gravity_system(
    gravity: Res<Gravity>,
    time: Res<Time>,
    attraction: &GravitationalAttraction,
    mut velocity: Mut<Velocity>,
) {
    if affected_by_gravity.is_grounded {
        *velocity.0.y_mut() = 0.;
    } else {
        *velocity.0.y_mut() -= gravity.0 * time.delta_seconds;
    }
}

fn velocity_system(time: Res<Time>, mut position: Mut<Translation>, velocity: Mut<Velocity>) {
    let dt = time.delta_seconds;

    *position.0.x_mut() = position.0.x() + velocity.0.x() * dt;
    *position.0.y_mut() = position.0.y() + velocity.0.y() * dt;
}
