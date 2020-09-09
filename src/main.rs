#![allow(dead_code)]

use bevy::{
    self,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    window::CursorMoved, 
    input::mouse::{MouseButtonInput}
};
use rand::{thread_rng, Rng};

// due to not being able to access windows from a `startup_system`
// fixed values will be needed for screen size during startup
// https://github.com/bevyengine/bevy/issues/175
const SCR_WIDTH: f32 = 800.0;
const SCR_HEIGHT: f32 = 600.0;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Lava Floor".to_string(),
            width: SCR_WIDTH as u32,
            height: SCR_HEIGHT as u32,
            resizable: false,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(FurniturePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(setup.system())
        .add_resource(Gravity(9.82 * 40.))
        .run();
}

#[derive(Default)]
struct MouseState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

struct Crosshair {
    // Aim direction
    aim: Vec2, 
    // Distance from center of player
    distance: f32,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dComponents::default());

    // Ground
    commands.spawn(SpriteComponents {
        material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
        translation: Translation(Vec3::new(0., -SCR_HEIGHT / 2., 0.)),
        sprite: Sprite {
            size: Vec2::new(SCR_WIDTH, 20.),
        },
        ..Default::default()
    })
    .with(Velocity(Vec2::zero()))
    .with(Collider::Solid);
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

pub struct SpawnTimer {
    pub timer: Timer,
    pub last_position: f32,
}

pub struct FurnitureSpawnOptions {
    min_time: f32,
    max_time: f32,
    speed: Speed,
}

#[derive(PartialEq)]
enum Collider {
    Solid,
}

pub struct Despawnable;

pub struct Speed(f32);
pub struct Force(f32);

pub struct FurniturePlugin;

impl Plugin for FurniturePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let furnitures = vec![
            Furniture { shape: FurnitureShape::Chair, size: Vec2::new(20., 36.) },
            Furniture { shape: FurnitureShape::Table, size: Vec2::new(64., 28.) },
            Furniture { shape: FurnitureShape::Sofa, size: Vec2::new(64., 36.) },
            Furniture { shape: FurnitureShape::Refrigerator, size: Vec2::new(44., 68.) },
            Furniture { shape: FurnitureShape::TV, size: Vec2::new(52., 32.) },
            Furniture { shape: FurnitureShape::Lamp, size: Vec2::new(32., 44.) },
        ];

        app.add_resource(SpawnTimer {
                timer: Timer::from_seconds(2.0, true),
                last_position: 0.5,
            })
            .add_resource(FurnitureSpawnOptions {
                min_time: 1.9,
                max_time: 3.2,
                speed: Speed(68.)
            })
            .add_resource(furnitures)
            .add_system(spawn_furniture_system.system())
            .add_system(despawn_furniture_system.system());
    }
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
            translation: Translation(Vec3::new(SCR_WIDTH / 2. + 200., -SCR_HEIGHT / 2. + furniture.size.y() / 2., 0.)),
            sprite: Sprite {
                size: furniture.size,
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
    mut query: Query<(Entity, &Translation, &Despawnable)>,
) {
    let window_size = get_window_size(windows);

    for (entity, translation, _despawnable) in &mut query.iter() {
        if translation.0.x() < -window_size.width / 2. - 200. {
            commands.despawn(entity);                
            //println!("Despawn furniture");
        }
    }
}

struct KeyboardControls {
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,
    jump: KeyCode,
}

const TOTAL_NUMBER_OF_JUMPS: usize = 2;
pub struct Player {
    speed: Speed,
    air_speed: Speed,
    jump_force: Force,
    num_of_jumps: usize,
    controls: KeyboardControls,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(player_spawn_system.system())
            .init_resource::<MouseState>()
            .add_system(player_input_system.system())
            .add_system(adjust_jump_system.system())
            .add_system(player_collision_system.system())
            .add_system(crosshair_system.system());
    }
}

// TODO: - rename to spawn_system when moving to separate files
fn player_spawn_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1., 0., 0.).into()),
            translation: Translation(Vec3::new(0., -SCR_HEIGHT/ 2. + 80.,0.)),
            sprite: Sprite {
                size: Vec2::new(12., 20.),
            },
            ..Default::default()
        })
        .with(
            Player {
                speed: Speed(200.),
                air_speed: Speed(180.),
                jump_force: Force(10.),
                num_of_jumps: TOTAL_NUMBER_OF_JUMPS,
                controls: KeyboardControls {
                    left: KeyCode::A,
                    right: KeyCode::D,
                    up: KeyCode::W,
                    down: KeyCode::S,
                    jump: KeyCode::Space,
                },
            }
        )
        .with(Collider::Solid)
        .with(AffectedByGravity {
            is_grounded: false,
        })
        .with(Velocity(Vec2::zero()));

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            translation: Translation(Vec3::zero()),
            sprite: Sprite {
                size: Vec2::new(5., 5.),
            },
            ..Default::default()
        })
        .with(Crosshair { 
            aim: Vec2::zero(),
            distance: 40.,
        });
}


fn get_distance(a: &Vec2, b: &Vec2) -> f32 {
    (a.x().powi(2) - b.x().powi(2) + a.y().powi(2) - b.y().powi(2)).sqrt()
}

fn get_direction(a: &Vec2, b: &Vec2) -> Vec2 {
    Vec2::new(b.x() - a.x(), b.y() - a.y())
}

fn get_window_size(windows: Res<Windows>) -> Size {
    if let Some(window) = windows.get_primary() {
        Size::new(window.height as f32, window.height as f32)
    } else {
        Size::new(SCR_WIDTH, SCR_HEIGHT)
    }
}

fn crosshair_system(
    windows: Res<Windows>,
    mut state: ResMut<MouseState>,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut query: Query<(&Player, &Translation)>,
    mut c_query: Query<(&mut Crosshair, &mut Translation, &mut Sprite)>,
) {
    let window_size = get_window_size(windows);

    for (_player, translation) in &mut query.iter() {
        for (mut crosshair, mut c_translation, _) in &mut c_query.iter() {
            
            let mut b_receive_event = false;
            for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
                b_receive_event = true;

                let cursor_pos = event.position - Vec2::new(window_size.width / 2., window_size.height / 2.);

                set_aim(&translation.0.truncate(), &cursor_pos, crosshair.distance, &mut c_translation);
                crosshair.aim = cursor_pos;
            }

            if !b_receive_event {
                set_aim(&translation.0.truncate(), &crosshair.aim, crosshair.distance, &mut c_translation);
            }
        }
    }

    for (_, _, mut sprite) in &mut c_query.iter() {
        sprite.size = if mouse_button_input.pressed(MouseButton::Left) {
            Vec2::new(10., 10.)
        } else {
            Vec2::new(5., 5.)
        };
    }
}

fn set_aim(a: &Vec2, b: &Vec2, distance: f32, translation: &mut Translation) {
    let direction = get_direction(a, b);
    let norm = direction.normalize() * distance;
    let aim = Vec2::new(a.x() + norm.x(), a.y() + norm.y());

    *translation.x_mut() = aim.x();
    *translation.y_mut() = aim.y();
}

fn player_input_system(
    _time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,    
    mut query: Query<(&mut Player, &mut Translation, &mut Velocity, &mut AffectedByGravity)>,
) {
    for (mut player ,mut translation, mut velocity, mut affected) in &mut query.iter() {
        if keyboard_input.just_pressed(player.controls.jump) && player.num_of_jumps > 0 {
            affected.is_grounded = false;

            // Move tha position of the player up a bit to avoid colliding with object before jumping
            *translation.0.y_mut() = translation.0.y() + 0.2;

            // Adjust jump force depending on how many jumps player has already made
            let adjuster: f32 = if TOTAL_NUMBER_OF_JUMPS == player.num_of_jumps {
                1.
            } else {
                0.7
            };

            velocity.0.set_y(player.jump_force.0 * 20. * adjuster);

            player.num_of_jumps -= 1;
        }

        let mut direction = Vec2::zero();
        if keyboard_input.pressed(player.controls.left) {
            let x = if affected.is_grounded {
                player.speed.0
            } else {
                player.air_speed.0
            };
            direction.set_x(-x);
        }

        if keyboard_input.pressed(player.controls.right) {
            let x = if affected.is_grounded {
                player.speed.0
            } else {
                player.air_speed.0
            };
            direction.set_x(x);
        }

        let direction: Vec3 = direction.extend(0.);
        velocity.0.set_x(direction.x());
    }
}

const FALL_MULTIPLIER: f32 = 2.5;
const LOW_JUMP_MULTIPLIER: f32 = 2.;

fn adjust_jump_system(
    time: Res<Time>,
    gravity: Res<Gravity>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Velocity, &AffectedByGravity)>,
) {
    let dt = time.delta_seconds;
    
    for (player, mut velocity, affected) in &mut query.iter() {
        if affected.is_grounded {
            break;
        }

        // Better jumping
        if velocity.0.y() < 0.0 {
            let vel = Vec2::unit_y() * -gravity.0 * (FALL_MULTIPLIER -1.) * dt;
            velocity.0 += vel;
        } else if velocity.0.y() > 0.0 && !keyboard_input.pressed(player.controls.jump) {
            let vel = Vec2::unit_y() * -gravity.0 * (LOW_JUMP_MULTIPLIER -1.) * dt;
            velocity.0 += vel;
        }
    }
}

fn player_collision_system(
    mut player_query: Query<(&mut Player, &mut Translation, &mut Velocity, &mut AffectedByGravity, &Sprite)>,
    mut collider_query: Query<(&Collider, Without<Player, &Velocity>, &Translation, &Sprite)>,
) {
    for (mut player, mut player_translation, mut velocity, mut player_affected, sprite) in &mut player_query.iter() {
        let player_size = sprite.size;

        let check_translation = Translation::new(player_translation.x(), player_translation.0.y() - 0.2, 0.) ;
        player_affected.is_grounded = false;
        for (_collider, c_velocity, translation, sprite) in &mut collider_query.iter() {
            let collision = collide(check_translation.0, player_size, translation.0, sprite.size);
            if let Some(collision) = collision {
                match collision {
                    Collision::Top => { 
                        // let is_grounded = *collider == Collider::Solid;
                        player_affected.is_grounded = true;

                        // Adjust player to be on top of platform
                        player_translation.0.set_y(translation.y() + sprite.size.y() / 2. + player_size.y() / 2. + 0.1);
                        player.num_of_jumps = TOTAL_NUMBER_OF_JUMPS;
                        // Set players velocity the same as the platform
                        *velocity.0.x_mut() = velocity.0.x() + c_velocity.0.x();

                    },
                    _ => {}
                };
            }
        }
    }
}

pub struct Velocity(pub Vec2);

pub struct Gravity(f32);

impl Default for Gravity {
    fn default() -> Self {
        Self(9.82 * 40.)
    }
}

pub struct AffectedByGravity {
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
    affected_by_gravity: &AffectedByGravity,
    mut velocity: Mut<Velocity>,
) {
    if affected_by_gravity.is_grounded {
        *velocity.0.y_mut() = 0.;
    } else {
        *velocity.0.y_mut() -= gravity.0 * time.delta_seconds;
    }
}

fn velocity_system(
    time: Res<Time>,
    mut position: Mut<Translation>,
    velocity: Mut<Velocity>
) {
    let y = position.0.y();
    let x = position.0.x();
    let dt = time.delta_seconds;

    position.0.set_y(y + velocity.0.y() * dt);
    position.0.set_x(x + velocity.0.x() * dt);
}