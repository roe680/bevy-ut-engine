use bevy::ecs::component::Component;
use bevy::transform::components::Transform;

use crate::{
    bone::bone_type::BoneStyle,
    utilities::attack::damage::DamageValue,
    utilities::time::SpawnDelay,
};

#[derive(Debug, Clone, PartialEq, Component, Default)]
#[require(Transform, BoneStyle, DamageValue, BoneLength, SpawnDelay)]
pub struct Bone;

#[derive(Debug, Clone, PartialEq, Component, Default)]
pub struct BoneLength(pub f32);

impl BoneLength {
    pub fn length(&self) -> f32 {
        self.0
    }

    pub fn set_length(&mut self, length: f32) {
        self.0 = length;
    }

    pub fn add_length(&mut self, delta: f32) {
        self.0 += delta;
    }
}
