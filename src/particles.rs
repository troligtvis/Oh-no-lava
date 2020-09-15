use crate::{
    bevy::prelude::*,
    Velocity, GravitationalAttraction,
};
use rand::{thread_rng, Rng};

pub struct DustParticle {
    size: Vec2,
    time_to_live: Timer,
}

impl Default for DustParticle {
    fn default() -> Self {
        Self {
            size: Vec2::new(2., 2.),
            time_to_live: Timer::from_seconds(0.3, false),
        }
    }
}

pub fn spawn_dust_particle(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
) {
    let upper = 140.;
    let lower = -140.;
    let mut rng = thread_rng();

    for _ in 0..5 {
        let x = rng.gen_range(lower, upper);
    
        let particle = DustParticle::default();
        commands.spawn(SpriteComponents {
            material: materials.add(Color::rgba(1., 1., 1., 0.6).into()),
            translation: Translation(position.extend(0.)),
            sprite: Sprite {
                size: particle.size,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(particle)
        .with(GravitationalAttraction::default())
        .with(Velocity(Vec2::new(
            0. + x,
            60.,
        )));
    }
}

pub fn dust_particle_cleanup_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DustParticle, &mut Sprite)> 
) {
    for (entity, mut particle, mut sprite) in &mut query.iter() {
        particle.time_to_live.tick(time.delta_seconds);
        if !particle.time_to_live.finished {
            let procentage =
                1. - (particle.time_to_live.elapsed) / particle.time_to_live.duration;
            sprite.size = particle.size * procentage;

            return;
        }
        commands.despawn(entity);
    }
}