use crate::{
    bevy::prelude::*,
};
use crate::comp;
use crate::res;
use rand::{thread_rng, Rng};

pub struct ParticlePlugin; 

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(shrinkable_particle_cleanup_system.system());
    }
}

pub fn shrinkable_particle_cleanup_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity, 
        With<comp::particles::Particle, &mut Sprite>, 
        With<comp::particles::Shrinkable, &mut comp::stats::TimeToLive>
    )>
) {
    for (
        entity, 
        mut sprite, 
        mut timer
    ) in &mut query.iter() {
        timer.0.tick(time.delta_seconds);
        if !timer.0.finished {
            let procentage = 1. - (timer.0.elapsed) / timer.0.duration;
            sprite.size *= procentage;

            return;
        }
        commands.despawn(entity);
    }
}

pub fn spawn_dust_particle(
    commands: &mut Commands,
    materials: &mut res::ColorMaterialStorage,
    position: Vec2,
) {
    let upper = 140.;
    let lower = -140.;
    let mut rng = thread_rng();

    for _ in 0..5 {
        let x = rng.gen_range(lower, upper);
    
        let particle = comp::particles::DustParticle::default();
        commands.spawn(SpriteComponents {
            material:  *materials.storage.get(&"Dust".to_string()).unwrap(),
            transform: Transform::from_translation(position.extend(0.)),
            sprite: Sprite {
                size: particle.size,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(particle)
        .with(comp::physics::GravitationalAttraction::default())
        .with(comp::physics::Velocity(Vec2::new(
            0. + x,
            60.,
        )));
    }
}

pub fn spawn_wall_dust_particle(
    commands: &mut Commands,
    materials: &mut res::ColorMaterialStorage,
    position: Vec2,
    direction: Vec2,
    facing: f32,
    particle_count: i32,
) {
    let upper = 20.;
    let lower = -20.;
    let mut rng = thread_rng();

    for _ in 0..particle_count {
        let rnd = rng.gen_range(lower, upper);
    
        let particle = comp::particles::DustParticle::default();
        commands.spawn(SpriteComponents {
            material:  *materials.storage.get(&"Dust".to_string()).unwrap(),
            transform: Transform::from_translation(position.extend(0.)),
            sprite: Sprite {
                size: particle.size,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(particle)
        .with(comp::physics::GravitationalAttraction::default())
        .with(comp::physics::Velocity(Vec2::new(
            direction.x() + (rnd * facing),
            direction.y() + (rnd * facing),
        )));
    }
}