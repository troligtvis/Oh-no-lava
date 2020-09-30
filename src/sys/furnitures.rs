use bevy::{
    prelude::*,
};

use crate::comp;
use crate::res;
use crate::util;

use rand::{thread_rng, Rng};


pub struct FurniturePlugin;

impl Plugin for FurniturePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(comp::stats::SpawnTimer(
                Timer::from_seconds(0.1, true)
            ))
            .add_system(spawn_furniture_system.system())
            .add_system(despawn_furniture_system.system());
    }
}

fn spawn_furniture_system(
    mut commands: Commands,
    windows: Res<Windows>,
    materials: ResMut<res::ColorMaterialStorage>,
    time: Res<Time>,
    mut spawn_timer: ResMut<comp::stats::SpawnTimer>,
) {
    let scr_size = util::get_window_size(windows);
    
    spawn_timer.0.tick(time.delta_seconds);
    if !spawn_timer.0.finished {
        return;
    }

    let mut rng = thread_rng();
    spawn_timer.0.duration = rng.gen_range(2.9, 4.2);

    let size = Vec2::new(84., 136.); 
    commands
        .spawn(SpriteComponents {
            material: *materials.storage.get(&"Default_Furniture".to_string()).unwrap(),
            transform: Transform::from_translation(Vec3::new(
                scr_size.width / 2. + 200., 
                -scr_size.height / 2. + 50., 
                0.
            )),
            sprite: Sprite {
                size,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(comp::actor::Furniture)
        .with(comp::physics::ColliderBox {
            w: size.x(),
            h: size.y(),
        })
        .with(comp::physics::Velocity(Vec2::new(- 60., 0.)))
        .with(comp::stats::Ground)
        .with(comp::stats::Wall);
}

fn despawn_furniture_system(
    mut commands: Commands,
    windows: Res<Windows>,
    mut query: Query<(Entity, With<comp::actor::Furniture, &Transform>)>,
) {
    let window_size = util::get_window_size(windows);

    for (entity, transform) in &mut query.iter() {
        if transform.translation().x() < -window_size.width / 2. - 200. {
            // https://github.com/bevyengine/bevy/issues/190#a-674447429
            commands.remove_one::<Draw>(entity);
            commands.despawn(entity);
        }
    }
}