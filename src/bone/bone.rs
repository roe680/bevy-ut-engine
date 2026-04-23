use bevy::{color::Color, ecs::component::Component, transform::components::Transform};
use num_traits::ops::saturating;

use crate::{bone::bone_type::BoneType, helpers::attack::damege::DamageValue};
#[derive(Debug, Clone, PartialEq, Component, Default)]
#[require(Transform, BoneType, DamageValue, BoneLength, BoneColor)]
pub struct Bone;

#[derive(Debug, Clone, PartialEq, Component, Default)]
pub struct BoneLength(f32);
#[derive(Debug, Clone, PartialEq, Component, Default)]
pub struct BoneColor(Color);
