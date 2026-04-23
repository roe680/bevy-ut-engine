use std::ops::{Deref, DerefMut};

use bevy::ecs::component::Component;

#[derive(Debug, Clone, PartialEq, Default, Component)]
pub struct Animations<T>(pub Vec<(T, f32, f32, f32, f32)>);

impl<T> Animations<T> {
    pub fn new() -> Self {
        Animations::<T>(vec![])
    }
    pub fn set_duration(mut self, duration: f32) -> Self {
        self.0.last_mut().expect("addされてないよ!").2 = duration;
        self
    }

    pub fn set_delay(mut self, delay: f32) -> Self {
        self.0.last_mut().expect("addされてないよ!").1 = delay;
        self
    }

    pub fn set_start_fraction(mut self, start_fraction: f32) -> Self {
        self.0.last_mut().expect("addされてないよ!").3 = start_fraction;
        self
    }
    pub fn set_end_fraction(mut self, end_fraction: f32) -> Self {
        self.0.last_mut().expect("addされてないよ!").4 = end_fraction;
        self
    }
}
pub fn create_default<T>(moving: T) -> (T, f32, f32, f32, f32) {
    (moving, 0.0, 0.0, 0.0, 1.0) //moving, delay, duration,start_fraction, end
}

impl<T> Deref for Animations<T> {
    type Target = Vec<(T, f32, f32, f32, f32)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Animations<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
