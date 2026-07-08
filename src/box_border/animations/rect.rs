use crate::{
    box_border::animations::shape::ShapeAnim,
    move_animation::moving::{Animations, create_default},
};

use bevy::prelude::*;

impl Animations<ShapeAnim> {
    pub fn move_rect(mut self, x: f32, y: f32, w: f32, h: f32, ease: EaseFunction) -> Self {
        self.0.push(create_default(ShapeAnim::MoveToShape(
            ease,
            None,
            vec![[x, y], [x + w, y], [x + w, y + h], [x, y + h]],
            vec![],
        )));
        self
    }

    pub fn add_rect(mut self, x: f32, y: f32, w: f32, h: f32, ease: EaseFunction) -> Self {
        self.0.push(create_default(ShapeAnim::AddToShape(
            ease,
            vec![[x, y], [x + w, y], [x + w, y + h], [x, y + h]],
            vec![],
        )));
        self
    }
}
