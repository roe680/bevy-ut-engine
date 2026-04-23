use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::helpers::{enemy_turn_reset::EnemyTurnReset, time::SpawnDelay};
#[derive(Debug, Clone, PartialEq, Component)]
#[require(Transform {
    translation: Vec3::new(0.0, -60.0, 0.0),
    ..default()
},
EnemyTurnReset,
BoxType,
BoxZIndex,
SpawnDelay,
)]
pub struct UTBox(pub Vec<[f32; 2]>);

impl From<Vec<[f32; 2]>> for UTBox {
    fn from(points: Vec<[f32; 2]>) -> Self {
        Self(points)
    }
}

impl From<UTBox> for Vec<[f32; 2]> {
    fn from(box_: UTBox) -> Self {
        box_.0
    }
}

impl Deref for UTBox {
    type Target = Vec<[f32; 2]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UTBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for UTBox {
    fn default() -> Self {
        Self::rect(-49.0, -49.0, 98.0, 98.0)
    }
}

impl UTBox {
    pub fn add_translation(mut self, x: f32, y: f32) -> Self {
        for point in self.0.iter_mut() {
            point[0] += x;
            point[1] += y;
        }
        self
    }

    pub fn add_rotation(mut self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        for point in self.0.iter_mut() {
            let x = point[0];
            let y = point[1];
            point[0] = x * cos - y * sin;
            point[1] = x * sin + y * cos;
        }
        self
    }
}

#[derive(Debug, Clone, PartialEq, Component, Default)]
pub enum BoxType {
    #[default]
    Union,
    Difference,
}

#[derive(Debug, Clone, PartialEq, Component, Default, Copy)]
pub struct BoxZIndex(pub i32);

impl Deref for BoxZIndex {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxZIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
