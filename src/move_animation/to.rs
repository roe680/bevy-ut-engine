use bevy::prelude::*;

use crate::{
    helpers::time::LifeTimer,
    move_animation::moving::{Animations, create_default},
};

pub enum ToAnim {
    MoveTo(EaseFunction, Option<Vec3>, Vec3, Vec3),
    AddTo(EaseFunction, Vec3, Vec3),
}

impl Animations<ToAnim> {
    pub fn move_to(mut self, to: Vec3, ease: EaseFunction) -> Self {
        self.0
            .push(create_default(ToAnim::MoveTo(ease, None, to, Vec3::ZERO)));
        self
    }
    pub fn add_to(mut self, to: Vec3, ease: EaseFunction) -> Self {
        self.0
            .push(create_default(ToAnim::AddTo(ease, to, Vec3::ZERO)));
        self
    }
}

pub fn animation_to(mut query: Query<(&mut Transform, &mut Animations<ToAnim>, &LifeTimer)>) {
    for (mut transform, mut animations, timer) in query.iter_mut() {
        for (to, delay, duration, start_fraction, end_fraction) in animations.iter_mut() {
            if *delay <= timer.0 {
                let t = ((timer.0 - *delay) / *duration).clamp(0.0, 1.0)
                    * (*end_fraction - *start_fraction)
                    + *start_fraction;
                match to {
                    ToAnim::MoveTo(ease, start, to, memory) => {
                        if start.is_none() {
                            *start = Some(transform.translation);
                        }
                        if let Some(start_val) = start {
                            let lerp = (*to - *start_val) * ease.sample_clamped(t);
                            transform.translation += lerp - *memory;
                            *memory = lerp;
                        }
                    }
                    ToAnim::AddTo(ease, to, memory) => {
                        let lerp = *to * ease.sample_clamped(t);
                        transform.translation += lerp - *memory;
                        *memory = lerp;
                    }
                }
            }
        }
        animations.0.retain(|(_, delay, duration, _, _)| *delay + *duration > timer.0);
    }
}
