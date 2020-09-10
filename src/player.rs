use crate::{
    bevy::{
        input::mouse::MouseButtonInput,
        prelude::*,
        sprite::collide_aabb::{collide, Collision},
        window::CursorMoved,
    },
    projectile::*,
    util::*,
    AffectedByGravity, Collider, Force, Gravity, SpawnTimer, Speed, Velocity,
};

const TOTAL_NUMBER_OF_JUMPS: usize = 2;
const FALL_MULTIPLIER: f32 = 2.5;
const LOW_JUMP_MULTIPLIER: f32 = 2.;

#[derive(Default)]
struct MouseState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

pub struct Player {
    speed: Speed,
    air_speed: Speed,
    jump_force: Force,
    num_of_jumps: usize,
    controls: KeyboardControls,
}

struct KeyboardControls {
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,
    jump: KeyCode,
}

// TODO: - rename to spawn_system when moving to separate files
fn spawn_system(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let translation = Translation(Vec3::new(0., -SCR_HEIGHT / 2. + 80., 0.));
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgba(1., 0., 0., 0.8).into()),
            translation,
            sprite: Sprite {
                size: Vec2::new(8., 16.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Player {
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
        })
        .with(Collider::Solid)
        .with(AffectedByGravity { is_grounded: false })
        .with(Velocity(Vec2::zero()));

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            translation: Translation(Vec3::zero()),
            sprite: Sprite {
                size: Vec2::new(5., 5.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Crosshair {
            aim: Vec2::zero(),
            distance: 40.,
        });
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

                let cursor_pos =
                    event.position - Vec2::new(window_size.width / 2., window_size.height / 2.);

                set_aim(
                    &translation.0.truncate(),
                    &cursor_pos,
                    crosshair.distance,
                    &mut c_translation,
                );
                crosshair.aim = cursor_pos;
            }

            if !b_receive_event {
                set_aim(
                    &translation.0.truncate(),
                    &crosshair.aim,
                    crosshair.distance,
                    &mut c_translation,
                );
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
    mut query: Query<(
        &mut Player,
        &mut Translation,
        &mut Velocity,
        &mut AffectedByGravity,
    )>,
) {
    for (mut player, mut translation, mut velocity, mut affected) in &mut query.iter() {
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
            let vel = Vec2::unit_y() * -gravity.0 * (FALL_MULTIPLIER - 1.) * dt;
            velocity.0 += vel;
        } else if velocity.0.y() > 0.0 && !keyboard_input.pressed(player.controls.jump) {
            let vel = Vec2::unit_y() * -gravity.0 * (LOW_JUMP_MULTIPLIER - 1.) * dt;
            velocity.0 += vel;
        }
    }
}

fn player_collision_system(
    mut player_query: Query<(
        &mut Player,
        &mut Translation,
        &mut Velocity,
        &mut AffectedByGravity,
        &Sprite,
    )>,
    mut collider_query: Query<(&Collider, Without<Player, &Velocity>, &Translation, &Sprite)>,
) {
    for (mut player, mut player_translation, mut velocity, mut player_affected, sprite) in
        &mut player_query.iter()
    {
        let player_size = sprite.size;
        let check_translation =
            Translation::new(player_translation.x(), player_translation.0.y() - 0.2, 0.);

        player_affected.is_grounded = false;

        for (_collider, c_velocity, translation, sprite) in &mut collider_query.iter() {
            let collision = collide(check_translation.0, player_size, translation.0, sprite.size);
            if let Some(collision) = collision {
                if let Collision::Top = collision {
                    player_affected.is_grounded = true;

                    // Adjust player to be on top of platform
                    player_translation
                        .0
                        .set_y(translation.y() + sprite.size.y() / 2. + player_size.y() / 2. + 0.1);
                    player.num_of_jumps = TOTAL_NUMBER_OF_JUMPS;
                    // Set players velocity the same as the platform
                    *velocity.0.x_mut() = velocity.0.x() + c_velocity.0.x();
                }
            }
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_system.system())
            .init_resource::<MouseState>()
            .add_resource(SpawnTimer {
                timer: Timer::from_seconds(2.0, true),
            })
            .add_system(player_input_system.system())
            .add_system(adjust_jump_system.system())
            .add_system(player_collision_system.system())
            .add_system(crosshair_system.system())
            .add_stage_before(stage::UPDATE, "spawn_projectile")
            .add_stage_after(stage::UPDATE, "shoot_projectile")
            .add_system_to_stage("spawn_projectile", spawn_projectile_system.system())
            .add_system_to_stage("shoot_projectile", shoot_projectile_system.system());
    }
}
