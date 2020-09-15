use crate::{bevy::prelude::*};

pub struct Animation {
    pub data: Vec<AnimationData>,
    pub current_index: usize,
}

impl Animation {
    pub fn get_current_data(&self) -> &AnimationData {
        &self.data[self.current_index]
    }
}

pub struct AnimationData {
    pub start_index: usize,
    pub frames_count: usize,
}

impl AnimationData {
    pub fn get_index(&self, i: u32) -> u32 {
        ((i + 1) % self.frames_count as u32) + self.start_index as u32
    }
}

fn animate_sprite_system(
    animation: Res<Animation>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>,
) {
    let animation_data = animation.get_current_data();
    for (timer, mut sprite) in &mut query.iter() {
        if timer.finished {
            sprite.index = animation_data.get_index(sprite.index);
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(animate_sprite_system.system());
    }
}