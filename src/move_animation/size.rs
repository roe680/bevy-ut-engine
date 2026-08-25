use bevy::prelude::*;

use crate::{
    utilities::time::LifeTimer,
    move_animation::moving::{Animations, create_default},
};

pub enum SizeAnim {
    MoveSize(EaseFunction, Option<Vec3>, Vec3, Vec3),
    AddSize(EaseFunction, Vec3, Vec3),
}

impl Animations<SizeAnim> {
    pub fn move_size(mut self, size: Vec3, ease: EaseFunction) -> Self {
        self.0.push(create_default(SizeAnim::MoveSize(
            ease,
            None,
            size,
            Vec3::ZERO,
        )));
        self
    }
    pub fn add_size(mut self, size: Vec3, ease: EaseFunction) -> Self {
        self.0
            .push(create_default(SizeAnim::AddSize(ease, size, Vec3::ZERO)));
        self
    }
}

pub fn animation_size(mut query: Query<(&mut Transform, &mut Animations<SizeAnim>, &LifeTimer)>) {
    for (mut transform, mut animations, timer) in query.iter_mut() {
        for (moving_type, delay, duration, start_fraction, end_fraction) in animations.iter_mut() {
            if *delay <= timer.0 {
                let t = ((timer.0 - *delay) / *duration).clamp(0.0, 1.0)
                    * (*end_fraction - *start_fraction)
                    + *start_fraction;
                match moving_type {
                    SizeAnim::MoveSize(ease, start, to, memory) => {
                        if start.is_none() {
                            *start = Some(transform.scale);
                        }
                        if let Some(start_val) = start {
                            let lerp = (*to - *start_val) * ease.sample_clamped(t);
                            transform.scale += lerp - *memory;
                            *memory = lerp;
                        }
                    }
                    SizeAnim::AddSize(ease, size, memory) => {
                        let lerp = *size * ease.sample_clamped(t);
                        transform.scale += lerp - *memory;
                        *memory = lerp;
                    }
                }
            }
        }
        animations.0.retain(|(_, delay, duration, _, _)| *delay + *duration > timer.0);
    }
}
