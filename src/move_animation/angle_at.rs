use bevy::{
    ecs::system::Query,
    math::{Quat, Vec3},
    transform::components::Transform,
};

use crate::{
    helpers::{easing::Easing, time::LifeTimer},
    move_animation::moving::{Animations, create_default},
};

pub enum AngleAt {
    MoveAngleAt(Easing, Option<Quat>, f32, Quat, Vec3, Option<Vec3>),
    AddAngleAt(Easing, f32, Quat, Vec3, Option<Vec3>),
}

impl Animations<AngleAt> {
    pub fn move_angle_at(mut self, angle: f32, at: Vec3, ease: Easing) -> Self {
        self.0.push(create_default(AngleAt::MoveAngleAt(
            ease,
            None,
            angle,
            Quat::IDENTITY,
            at,
            None,
        )));
        self
    }
    pub fn add_angle_at(mut self, angle: f32, at: Vec3, ease: Easing) -> Self {
        self.0.push(create_default(AngleAt::AddAngleAt(
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
    mut query: Query<(&mut Transform, &mut Animations<AngleAt>, &LifeTimer)>,
) {
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
                    AngleAt::MoveAngleAt(ease, start, to, angle_memory, at, at_memory) => {
                        if start.is_none() {
                            *start = Some(transform.rotation);
                        }
                        // 初回のみ at_memory を初期化
                        if at_memory.is_none() {
                            *at_memory = Some(transform.translation - *at);
                        }
                        if let (Some(start_val), Some(prev_pos)) = (start, at_memory) {
                            let to_quat = Quat::from_rotation_z(*to);

                            // 目標回転を球面線形補間で計算
                            let target_rotation =
                                start_val.slerp(to_quat, ease.ease(t)) * start_val.inverse();
                            // 前フレームの回転
                            let prev_rotation = *angle_memory;
                            // 回転も更新
                            let rotation_delta = prev_rotation.inverse() * target_rotation;

                            // 今フレームの回転後座標
                            let new_rotated = rotation_delta * *prev_pos;

                            // 差分だけ加算
                            transform.translation += new_rotated - *prev_pos;

                            transform.rotation = transform.rotation * rotation_delta;

                            // メモリを更新
                            *angle_memory = target_rotation;
                            *prev_pos = new_rotated;
                        }
                    }
                    AngleAt::AddAngleAt(ease, angle, angle_memory, at, at_memory) => {
                        // 初回のみ at_memory を初期化
                        if at_memory.is_none() {
                            *at_memory = Some(transform.translation - *at);
                        }
                        if let Some(prev_pos) = at_memory {
                            let target_rotation = Quat::from_rotation_z(*angle * ease.ease(t));
                            let rotation_delta = angle_memory.inverse() * target_rotation;

                            // 今フレームの回転後座標
                            let curr_rotated = rotation_delta * *prev_pos;

                            // 差分だけ加算
                            transform.translation += curr_rotated - *prev_pos;

                            // 回転も更新
                            transform.rotation = transform.rotation * rotation_delta;

                            // メモリ更新
                            *angle_memory = target_rotation;
                            *at_memory = Some(curr_rotated);
                        }
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
