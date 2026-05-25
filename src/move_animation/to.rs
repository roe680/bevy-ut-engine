use bevy::prelude::*;

use crate::{
    helpers::time::LifeTimer,
    move_animation::moving::{create_default, Animations},
};

pub enum To {
    MoveTo(EaseFunction, Option<Vec3>, Vec3, Vec3),
    AddTo(EaseFunction, Vec3, Vec3),
}

impl Animations<To> {
    pub fn move_to(mut self, to: Vec3, ease: EaseFunction) -> Self {
        self.0
            .push(create_default(To::MoveTo(ease, None, to, Vec3::ZERO)));
        self
    }
    pub fn add_to(mut self, to: Vec3, ease: EaseFunction) -> Self {
        self.0.push(create_default(To::AddTo(ease, to, Vec3::ZERO)));
        self
    }
}

pub fn animation_to(mut query: Query<(&mut Transform, &mut Animations<To>, &LifeTimer)>) {
    for (mut transform, mut animations, timer) in query.iter_mut() {
        let mut remove_indices: Vec<usize> = vec![];
        for (i, (to, delay, duration, start_fraction, end_fraction)) in
            animations.iter_mut().enumerate()
        {
            if *delay <= timer.0 {
                let t = ((timer.0 - *delay) / *end_fraction).clamp(0.0, 1.0)
                    * (*end_fraction - *start_fraction)
                    + *start_fraction;
                match to {
                    To::MoveTo(ease, start, to, memory) => {
                        if start.is_none() {
                            *start = Some(transform.translation);
                        }
                        if let Some(start_val) = start {
                            let lerp = (*to - *start_val) * ease.sample_clamped(t);
                            transform.translation += lerp - *memory;
                            *memory = lerp;
                        }
                    }
                    To::AddTo(ease, to, memory) => {
                        let lerp = *to * ease.sample_clamped(t);
                        transform.translation += lerp - *memory;
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
