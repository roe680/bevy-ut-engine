use bevy::prelude::*;

use crate::{
    utilities::time::LifeTimer,
    move_animation::moving::{create_default, Animations},
};

pub enum BoneLengthAnim {
    MoveLenAnim(EaseFunction, Option<f32>, f32, f32),
    AddLenAnim(EaseFunction, f32, f32),
}

impl Animations<BoneLengthAnim> {
    pub fn move_len(mut self, to: f32, ease: EaseFunction) -> Self {
        self.0
            .push(create_default(BoneLengthAnim::MoveLenAnim(ease, None, to, 0.0)));
        self
    }

    pub fn add_len(mut self, to: f32, ease: EaseFunction) -> Self {
        self.0
            .push(create_default(BoneLengthAnim::AddLenAnim(ease, to, 0.0)));
        self
    }
}

pub fn animation_bone_length(
    mut query: Query<(
        &mut crate::bone::bone::BoneLength,
        &mut Animations<BoneLengthAnim>,
        &LifeTimer,
    )>,
) {
    for (mut bone_length, mut animations, timer) in query.iter_mut() {
        for (anim, delay, duration, start_fraction, end_fraction) in animations.iter_mut() {
            if *delay <= timer.0 {
                let t = ((timer.0 - *delay) / *duration).clamp(0.0, 1.0)
                    * (*end_fraction - *start_fraction)
                    + *start_fraction;

                match anim {
                    BoneLengthAnim::MoveLenAnim(ease, start, to, memory) => {
                        if start.is_none() {
                            *start = Some(bone_length.length());
                        }
                        if let Some(start_val) = start {
                            let lerp = (*to - *start_val) * ease.sample_clamped(t);
                            bone_length.add_length(lerp - *memory);
                            *memory = lerp;
                        }
                    }
                    BoneLengthAnim::AddLenAnim(ease, to, memory) => {
                        let lerp = *to * ease.sample_clamped(t);
                        bone_length.add_length(lerp - *memory);
                        *memory = lerp;
                    }
                }
            }
        }
        animations.0.retain(|(_, delay, duration, _, _)| *delay + *duration > timer.0);
    }
}
