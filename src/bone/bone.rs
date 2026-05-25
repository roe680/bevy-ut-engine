use bevy::{color::Color, ecs::component::Component, transform::components::Transform};
use num_traits::ops::saturating;

use crate::{bone::bone_type::BoneStyle, helpers::attack::damege::DamageValue};
#[derive(Debug, Clone, PartialEq, Component, Default)]
#[require(Transform, BoneStyle, DamageValue, BoneLength)]
pub struct Bone;

#[derive(Debug, Clone, PartialEq, Component, Default)]
pub struct BoneLength(f32);

impl BoneLength {
    pub fn length(&self) -> f32 {
        self.0
    }
}
