use crate::{bevy::prelude::*};
use std::collections::HashMap;

pub trait AnimationStateDescriptor {
    fn name(&self) -> &str;
}

pub struct Animation {
    data: HashMap<String, AnimationData>,
    current_anim: String,
}

impl Animation {
    pub fn new(data: Vec<AnimationData>, start_anim: &str) -> Self {
        Self {
            data: data.into_iter().map(|x| (x.get_name(), x)).into_iter().collect(),
            current_anim: start_anim.to_string(), 
        }
    }

    fn get_current_data(&self) -> Option<&AnimationData> {
        self.data.get(&self.current_anim)
    }

    pub fn set_anim(&mut self, name: &str) {
        self.current_anim = name.to_string();
    }
}

pub struct AnimationData {
    name: String,
    start_index: usize,
    frames_count: usize,
}

impl AnimationData {
    pub fn new(name: &str, start_index: usize, frames_count: usize) -> Self {
        Self {
            name: name.to_string(),
            start_index,
            frames_count,
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_index(&self, i: u32) -> u32 {
        ((i + 1) % self.frames_count as u32) + self.start_index as u32
    }
}

fn animate_sprite_system(
    animation: Res<Animation>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>,
) {
    if let Some(animation_data) = animation.get_current_data() {
        for (timer, mut sprite) in &mut query.iter() {
            if timer.finished {
                sprite.index = animation_data.get_index(sprite.index);
            }
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(animate_sprite_system.system());
    }
}