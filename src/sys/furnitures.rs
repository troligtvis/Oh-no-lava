use bevy::{
    prelude::*,
};

use crate::comp;
use crate::res;
use crate::util;

use rand::{thread_rng, Rng};

struct Wave;

struct FurnitureSpawner {
    start_position: Vec3,
    end_position: Vec3,
    t_min: f32,
    t_max: f32,
    timer: Timer,
}

pub struct FurniturePlugin;

impl Plugin for FurniturePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(comp::stats::SpawnTimer(
                Timer::from_seconds(0.1, true)
            ))
            .add_startup_system_to_stage("post_startup", setup_furnitures.system())
            .add_system(spawn_system.system())
            .add_system(despawn_system.system());
    }
}

fn setup_furnitures(
    mut commands: Commands,
    materials: ResMut<res::ColorMaterialStorage>,
) {

    let size = Vec2::new(84., 136.); 

    for _ in 0..10 {
        let handle = materials.storage.get(&"Default_Furniture".to_string()).unwrap();
        commands
            .spawn(SpriteComponents {
                material: handle.clone(),
                transform: Transform::from_translation(Vec3::zero()),
                sprite: Sprite {
                    size,
                    ..Default::default()
                },
                draw: Draw {
                    is_visible: false,
                    is_transparent: false,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(comp::actor::Furniture)
            .with(comp::physics::ColliderBox {
                w: size.x(),
                h: size.y(),
            })
            .with(comp::physics::Velocity(Vec2::zero()))
            .with(comp::stats::Ground)
            .with(comp::stats::Wall)
            .with(Wave);
    }
}

fn spawn_system(
    windows: Res<Windows>,
    time: Res<Time>,
    mut spawn_timer: ResMut<comp::stats::SpawnTimer>,
    mut query: Query<(
        With<Wave, &mut Transform>,
        &mut comp::physics::Velocity,
        &mut Draw
    )>
) {
    spawn_timer.0.tick(time.delta_seconds);
    if !spawn_timer.0.finished {
        return;
    }

    let mut rng = thread_rng();
    spawn_timer.0.duration = rng.gen_range(2.9, 4.2);

    let scr_size = util::get_window_size(windows);

    for (mut transform, mut velocity, mut draw) in query.iter_mut() {
        if !draw.is_visible {
            draw.is_visible = true;
            *velocity.0.x_mut() = -60.;
    
            transform.translation = Vec3::new(
                scr_size.width / 2. + 200., 
                -scr_size.height / 2. + 50., 
                0.
            );

            // We only want one
            return;
        }
    }
}

fn despawn_system(
    windows: Res<Windows>,
    mut query: Query<(
        With<Wave, &mut Transform>,
        &mut Draw,
        &mut comp::physics::Velocity,
    )>,
) {
    let window_size = util::get_window_size(windows);

    for (mut transform, mut draw, mut velocity) in query.iter_mut() {
        if transform.translation.x() < -window_size.width / 2. - 200. {
            draw.is_visible = false;

            let start_position = Vec3::new(
                window_size.width / 2. + 200., 
                -window_size.height / 2. + 50., 
                0.
            );

            transform.translation = start_position;

            *velocity.0.x_mut() = 0.;
        }
    }
}