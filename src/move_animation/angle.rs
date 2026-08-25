use bevy::prelude::*;

use crate::{
    utilities::time::LifeTimer,
    move_animation::moving::{Animations, create_default},
};

pub enum AngleAnim {
    MoveAngle(EaseFunction, Option<Quat>, f32, Quat),
    AddAngle(EaseFunction, f32, Quat),
}

impl Animations<AngleAnim> {
    pub fn move_angle(mut self, angle: f32, ease: EaseFunction) -> Self {
        self.0.push(create_default(AngleAnim::MoveAngle(
            ease,
            None,
            angle,
            Quat::IDENTITY,
        )));
        self
    }
    pub fn add_angle(mut self, angle: f32, ease: EaseFunction) -> Self {
        self.0.push(create_default(AngleAnim::AddAngle(
            ease,
            angle,
            Quat::IDENTITY,
        )));
        self
    }
}

pub fn animation_angle(mut query: Query<(&mut Transform, &mut Animations<AngleAnim>, &LifeTimer)>) {
    for (mut transform, mut animations, timer) in query.iter_mut() {
        for (moving_type, delay, duration, start_fraction, end_fraction) in animations.iter_mut() {
            if *delay <= timer.0 {
                let t = ((timer.0 - *delay) / *duration).clamp(0.0, 1.0)
                    * (*end_fraction - *start_fraction)
                    + *start_fraction;
                match moving_type {
                    AngleAnim::MoveAngle(ease, start, to, memory) => {
                        if start.is_none() {
                            *start = Some(transform.rotation);
                        }
                        if let Some(start_val) = start {
                            let to_quat = Quat::from_rotation_z(*to);

                            let target_rotation = start_val.slerp(to_quat, ease.sample_clamped(t))
                                * start_val.inverse();

                            let rotation_delta = memory.inverse() * target_rotation;

                            transform.rotation = transform.rotation * rotation_delta;

                            *memory = target_rotation;
                        }
                    }
                    AngleAnim::AddAngle(ease, angle, memory) => {
                        let target_rotation =
                            Quat::from_rotation_z(*angle * ease.sample_clamped(t));
                        let rotation_delta = memory.inverse() * target_rotation;
                        transform.rotation = transform.rotation * rotation_delta;
                        *memory = target_rotation;
                    }
                }
            }
        }
        animations.0.retain(|(_, delay, duration, _, _)| *delay + *duration > timer.0);
    }
}
