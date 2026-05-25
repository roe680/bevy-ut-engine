use bevy::prelude::*;

use crate::{
    box_border::box_struct::UTBox,
    helpers::time::LifeTimer,
    move_animation::moving::{create_default, Animations},
};

pub enum Shape {
    MoveToShape(
        EaseFunction,
        Option<Vec<[f32; 2]>>,
        Vec<[f32; 2]>,
        Vec<[f32; 2]>,
    ),
    AddToShape(EaseFunction, Vec<[f32; 2]>, Vec<[f32; 2]>),
    AddShapeTranslation(EaseFunction, [f32; 2], [f32; 2]),
    AddRotationAngle(EaseFunction, f32, f32),
    AddShapeRotationAt(EaseFunction, f32, f32, [f32; 2]),
}

impl Animations<Shape> {
    pub fn move_any_shape(mut self, shape: Vec<[f32; 2]>, ease: EaseFunction) -> Self {
        self.0.push(create_default(Shape::MoveToShape(
            ease,
            None,
            shape,
            vec![],
        )));
        self
    }
    pub fn add_any_shape(mut self, shape: Vec<[f32; 2]>, ease: EaseFunction) -> Self {
        self.0
            .push(create_default(Shape::AddToShape(ease, shape, vec![])));
        self
    }
    pub fn add_shape_translation(mut self, to: [f32; 2], ease: EaseFunction) -> Self {
        self.0.push(create_default(Shape::AddShapeTranslation(
            ease,
            to,
            [0.0, 0.0],
        )));
        self
    }
    pub fn add_shape_rotation(mut self, angle: f32, ease: EaseFunction) -> Self {
        self.0
            .push(create_default(Shape::AddRotationAngle(ease, angle, 0.0)));
        self
    }
    pub fn add_shape_rotation_at(mut self, angle: f32, at: [f32; 2], ease: EaseFunction) -> Self {
        self.0.push(create_default(Shape::AddShapeRotationAt(
            ease, angle, 0.0, at,
        )));
        self
    }
}

pub fn animation_shape(mut query: Query<(&mut UTBox, &mut Animations<Shape>, &LifeTimer)>) {
    for (mut utbox, mut animations, timer) in query.iter_mut() {
        let mut remove_indices: Vec<usize> = vec![];
        for (i, (moving_type, delay, duration, start_fraction, end_fraction)) in
            animations.iter_mut().enumerate()
        {
            if *delay <= timer.0 {
                let t = ((timer.0 - *delay) / *duration).clamp(0.0, 1.0)
                    * (*end_fraction - *start_fraction)
                    + *start_fraction;
                match moving_type {
                    Shape::MoveToShape(ease, start, to, memory) => {
                        if start.is_none() {
                            *start = Some(utbox.0.clone());
                        }
                        let mut new_memory = Vec::new();
                        if memory.len() != to.len() {
                            *memory = vec![[0.0, 0.0]; to.len()];
                        }
                        if let Some(start_val) = start {
                            for i in 0..to.len() {
                                // 各頂点も lerp の考え方で補間にゃ
                                let start_x = start_val[i][0];
                                let start_y = start_val[i][1];
                                let to_x = to[i][0];
                                let to_y = to[i][1];

                                let lerp_x = (to_x - start_x) * ease.sample_clamped(t);
                                let lerp_y = (to_y - start_y) * ease.sample_clamped(t);
                                utbox.0[i][0] += lerp_x - memory[i][0];
                                utbox.0[i][1] += lerp_y - memory[i][1];
                                new_memory.push([lerp_x, lerp_y]);
                            }
                        }
                        *memory = new_memory;
                    }
                    Shape::AddToShape(ease, to, memory) => {
                        let mut new_memory = Vec::new();
                        if memory.len() != to.len() {
                            *memory = vec![[0.0, 0.0]; to.len()];
                        }
                        for i in 0..to.len() {
                            let lerp_x = to[i][0] * ease.sample_clamped(t);
                            let lerp_y = to[i][1] * ease.sample_clamped(t);
                            utbox.0[i][0] += lerp_x - memory[i][0];
                            utbox.0[i][1] += lerp_y - memory[i][1];
                            new_memory.push([lerp_x, lerp_y]);
                        }
                        *memory = new_memory;
                    }
                    Shape::AddShapeTranslation(ease, to, memory) => {
                        let lerp_x = to[0] * ease.sample_clamped(t);
                        let lerp_y = to[1] * ease.sample_clamped(t);
                        utbox.iter_mut().for_each(|pos| {
                            *pos = [pos[0] + lerp_x - memory[0], pos[1] + lerp_y - memory[1]];
                        });
                        *memory = [lerp_x, lerp_y];
                    }
                    Shape::AddRotationAngle(ease, angle, memory) => {
                        let lerp_angle = *angle * ease.sample_clamped(t);
                        let sin = (lerp_angle - *memory).sin();
                        let cos = (lerp_angle - *memory).cos();
                        utbox.iter_mut().for_each(|pos| {
                            let x = pos[0];
                            let y = pos[1];
                            let new_x = x * cos - y * sin;
                            let new_y = x * sin + y * cos;
                            *pos = [new_x, new_y];
                        });
                        *memory = lerp_angle;
                    }
                    Shape::AddShapeRotationAt(ease, angle, memory, at) => {
                        let lerp_angle = *angle * ease.sample_clamped(t);
                        let sin = (lerp_angle - *memory).sin();
                        let cos = (lerp_angle - *memory).cos();
                        utbox.iter_mut().for_each(|pos| {
                            let x = pos[0] - at[0];
                            let y = pos[1] - at[1];
                            let new_x = x * cos - y * sin + at[0];
                            let new_y = x * sin + y * cos + at[1];
                            *pos = [new_x, new_y];
                        });
                        *memory = lerp_angle;
                    }
                }
            }
            if *delay + *duration <= timer.0 {
                remove_indices.push(i);
            }
        }
        for &index in remove_indices.iter().rev() {
            animations.0.remove(index);
        }
    }
}
