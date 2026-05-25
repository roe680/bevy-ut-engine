use crate::{
    box_border::animations::shape::Shape,
    move_animation::moving::{create_default, Animations},
};

use bevy::prelude::*;

impl Animations<Shape> {
    pub fn move_rect(mut self, x: f32, y: f32, w: f32, h: f32, ease: EaseFunction) -> Self {
        self.0.push(create_default(Shape::MoveToShape(
            ease,
            None,
            vec![[x, y], [x + w, y], [x + w, y + h], [x, y + h]],
            vec![],
        )));
        self
    }

    pub fn add_rect(mut self, x: f32, y: f32, w: f32, h: f32, ease: EaseFunction) -> Self {
        self.0.push(create_default(Shape::AddToShape(
            ease,
            vec![[x, y], [x + w, y], [x + w, y + h], [x, y + h]],
            vec![],
        )));
        self
    }
}
