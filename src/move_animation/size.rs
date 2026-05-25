use bevy::prelude::*;

use crate::{
    helpers::time::LifeTimer,
    move_animation::moving::{create_default, Animations},
};

pub enum Size {
    MoveSize(EaseFunction, Option<Vec3>, Vec3, Vec3),
    AddSize(EaseFunction, Vec3, Vec3),
}

impl Animations<Size> {
    pub fn move_size(mut self, size: Vec3, ease: EaseFunction) -> Self {
        self.0
            .push(create_default(Size::MoveSize(ease, None, size, Vec3::ZERO)));
        self
    }
    pub fn add_size(mut self, size: Vec3, ease: EaseFunction) -> Self {
        self.0
            .push(create_default(Size::AddSize(ease, size, Vec3::ZERO)));
        self
    }
}

pub fn animation_size(mut query: Query<(&mut Transform, &mut Animations<Size>, &LifeTimer)>) {
    for (mut transform, mut animations, timer) in query.iter_mut() {
        let mut remove_indices: Vec<usize> = vec![];
        for (i, (moving_type, delay, duration, start_fraction, end_fraction)) in
            animations.iter_mut().enumerate()
        {
            if *delay <= timer.0 {
                let t = ((timer.0 - *delay) / *duration).clamp(0.0, 1.0)
                    * (*end_fraction - *start_fraction)
                    + *start_fraction;
                match moving_type {
                    Size::MoveSize(ease, start, to, memory) => {
                        if start.is_none() {
                            *start = Some(transform.scale);
                        }
                        if let Some(start_val) = start {
                            let lerp = (*to - *start_val) * ease.sample_clamped(t);
                            transform.scale += lerp - *memory;
                            *memory = lerp;
                        }
                    }
                    Size::AddSize(ease, size, memory) => {
                        let lerp = *size * ease.sample_clamped(t);
                        transform.scale += lerp - *memory;
                        *memory = lerp;
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
