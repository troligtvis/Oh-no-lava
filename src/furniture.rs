use crate::{bevy::prelude::*, util::*, Collider, Despawnable, SpawnTimer, Speed, Velocity};

use rand::{thread_rng, Rng};

pub struct FurnitureSpawnOptions {
    min_time: f32,
    max_time: f32,
    speed: Speed,
}

// Different types of spawning furnitures
enum FurnitureShape {
    Chair,
    Table,
    Sofa,
    Refrigerator,
    TV,
    Lamp,
}

struct Furniture {
    shape: FurnitureShape,
    size: Vec2,
}

fn spawn_furniture_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    spawn_options: Res<FurnitureSpawnOptions>,
    furnitures: Res<Vec<Furniture>>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    spawn_timer.timer.tick(time.delta_seconds);
    if !spawn_timer.timer.finished {
        return;
    }

    let mut rng = thread_rng();
    spawn_timer.timer.duration = rng.gen_range(spawn_options.min_time, spawn_options.max_time);

    let idx = rng.gen_range(0, furnitures.len());
    let furniture = &furnitures[idx];

    let r = rng.gen_range(0, 255) as f32 / 255.;
    let g = rng.gen_range(0, 255) as f32 / 255.;
    let b = rng.gen_range(0, 255) as f32 / 255.;

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(r, g, b).into()),
            transform: Transform::from_translation(Vec3::new(
                SCR_WIDTH / 2. + 200.,
                -SCR_HEIGHT / 2. + furniture.size.y() / 2.,
                0.,
            )),
            sprite: Sprite {
                size: furniture.size,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Velocity(Vec2::new(-spawn_options.speed.0, 0.)))
        .with(Collider::Solid)
        .with(Despawnable);

    //println!("Spawn furniture");
}

fn despawn_furniture_system(
    mut commands: Commands,
    windows: Res<Windows>,
    mut query: Query<(Entity, &Transform, &Despawnable)>,
) {
    let window_size = get_window_size(windows);

    for (entity, transform, _despawnable) in &mut query.iter() {
        if transform.translation().x() < -window_size.width / 2. - 200. {
            commands.despawn(entity);
            //println!("Despawn furniture");
        }
    }
}

pub struct FurniturePlugin;

impl Plugin for FurniturePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let furnitures = vec![
            Furniture {
                shape: FurnitureShape::Chair,
                size: Vec2::new(20., 36.),
            },
            Furniture {
                shape: FurnitureShape::Table,
                size: Vec2::new(64., 28.),
            },
            Furniture {
                shape: FurnitureShape::Sofa,
                size: Vec2::new(64., 36.),
            },
            Furniture {
                shape: FurnitureShape::Refrigerator,
                size: Vec2::new(44., 68.),
            },
            Furniture {
                shape: FurnitureShape::TV,
                size: Vec2::new(52., 32.),
            },
            Furniture {
                shape: FurnitureShape::Lamp,
                size: Vec2::new(32., 44.),
            },
        ];

        app.add_resource(SpawnTimer {
            timer: Timer::from_seconds(2.0, true),
        })
        .add_resource(FurnitureSpawnOptions {
            min_time: 1.9,
            max_time: 3.2,
            speed: Speed(68.),
        })
        .add_resource(furnitures)
        .add_system(spawn_furniture_system.system())
        .add_system(despawn_furniture_system.system());
    }
}
