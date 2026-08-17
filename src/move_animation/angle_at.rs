use bevy::prelude::*;

use crate::{
    helpers::time::LifeTimer,
    move_animation::moving::{Animations, create_default},
};

pub enum AngleAtAnim {
    MoveAngleAt(EaseFunction, Option<Quat>, f32, Quat, Vec3, Option<Vec3>),
    AddAngleAt(EaseFunction, f32, Quat, Vec3, Option<Vec3>),
}

impl Animations<AngleAtAnim> {
    pub fn move_angle_at(mut self, angle: f32, at: Vec3, ease: EaseFunction) -> Self {
        self.0.push(create_default(AngleAtAnim::MoveAngleAt(
            ease,
            None,
            angle,
            Quat::IDENTITY,
            at,
            None,
        )));
        self
    }
    pub fn add_angle_at(mut self, angle: f32, at: Vec3, ease: EaseFunction) -> Self {
        self.0.push(create_default(AngleAtAnim::AddAngleAt(
            ease,
            angle,
            Quat::IDENTITY,
            at,
            None,
        )));
        self
    }
}

pub fn animation_angle_at(
    mut query: Query<(&mut Transform, &mut Animations<AngleAtAnim>, &LifeTimer)>,
) {
    for (mut transform, mut animations, timer) in query.iter_mut() {
        for (moving_type, delay, duration, start_fraction, end_fraction) in animations.iter_mut() {
            if *delay <= timer.0 {
                let t = ((timer.0 - *delay) / *duration).clamp(0.0, 1.0)
                    * (*end_fraction - *start_fraction)
                    + *start_fraction;
                match moving_type {
                    AngleAtAnim::MoveAngleAt(ease, start, to, angle_memory, at, at_memory) => {
                        if start.is_none() {
                            *start = Some(transform.rotation);
                        }
                        if at_memory.is_none() {
                            *at_memory = Some(transform.translation - *at);
                        }
                        if let (Some(start_val), Some(prev_pos)) = (start, at_memory) {
                            let to_quat = Quat::from_rotation_z(*to);

                            let target_rotation = start_val.slerp(to_quat, ease.sample_clamped(t))
                                * start_val.inverse();
                            let prev_rotation = *angle_memory;
                            let rotation_delta = prev_rotation.inverse() * target_rotation;

                            let new_rotated = rotation_delta * *prev_pos;

                            transform.translation += new_rotated - *prev_pos;

                            transform.rotation = transform.rotation * rotation_delta;

                            *angle_memory = target_rotation;
                            *prev_pos = new_rotated;
                        }
                    }
                    AngleAtAnim::AddAngleAt(ease, angle, angle_memory, at, at_memory) => {
                        if at_memory.is_none() {
                            *at_memory = Some(transform.translation - *at);
                        }
                        if let Some(prev_pos) = at_memory {
                            let target_rotation =
                                Quat::from_rotation_z(*angle * ease.sample_clamped(t));
                            let rotation_delta = angle_memory.inverse() * target_rotation;

                            let curr_rotated = rotation_delta * *prev_pos;

                            transform.translation += curr_rotated - *prev_pos;

                            transform.rotation = transform.rotation * rotation_delta;

                            *angle_memory = target_rotation;
                            *at_memory = Some(curr_rotated);
                        }
                    }
                }
            }
        }
        animations.0.retain(|(_, delay, duration, _, _)| *delay + *duration > timer.0);
    }
}
