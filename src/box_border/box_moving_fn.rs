// use crate::{
//     box_border::{
//         box_moving::{BoxMovingComponent, BoxMovingType},
//         box_struct::UTBox,
//     },
//     helpers::time::LifeTimer,
// };
// use bevy::prelude::*;

// // move_box システムを lerp で書き換えるイメージにゃ

// pub fn move_box(
//     mut query: Query<(
//         &mut Transform,
//         &mut UTBox,
//         &mut BoxMovingComponent,
//         &LifeTimer,
//     )>,
// ) {
//     for (mut transform, mut utbox, mut box_movings, life_timer) in query.iter_mut() {
//         let mut remove_indices: Vec<usize> = vec![];

//         for (i, (moving_type, delay, duration, ease_start, ease_end)) in
//             box_movings.0.iter_mut().enumerate()
//         {
//             // アニメーション実行中
//             if *delay <= life_timer.0 {
//                 let t = ((life_timer.0 - *delay) / *duration).clamp(0.0, 1.0)
//                     * (*ease_end - *ease_start)
//                     + *ease_start;

//                 match moving_type {
//                     BoxMovingType::MoveTo(ease, start, to, memory) => {
//                         if start.is_none() {
//                             *start = Some(transform.translation);
//                         }
//                         if let Some(start_val) = start {
//                             let lerp = (*to - *start_val) * ease.ease(t);
//                             transform.translation += lerp - *memory;
//                             *memory = lerp;
//                         }
//                     }
//                     BoxMovingType::MoveAngle(ease, start, to, memory) => {
//                         if start.is_none() {
//                             *start = Some(transform.rotation);
//                         }
//                         if let Some(start_val) = start {
//                             let to_quat = Quat::from_rotation_z(*to);

//                             // 目標回転を球面線形補間で計算
//                             let target_rotation =
//                                 start_val.slerp(to_quat, ease.ease(t)) * start_val.inverse();

//                             // 前フレームからの差分を計算（クォータニオンの差分）
//                             let rotation_delta = memory.inverse() * target_rotation;

//                             // 現在の回転に差分を適用
//                             transform.rotation = transform.rotation * rotation_delta;

//                             // メモリを更新
//                             memory = target_rotation;
//                         }
//                     }
//                     BoxMovingType::MoveSize(ease, start, to, memory) => {
//                         if start.is_none() {
//                             *start = Some(transform.scale);
//                         }
//                         if let Some(start_val) = start {
//                             // scale も lerp が使えるにゃ
//                             let lerp = (*to - *start_val) * ease.ease(t);
//                             transform.scale += lerp - *memory;
//                             *memory = lerp;
//                         }
//                     }
//                     BoxMovingType::MoveShape(ease, start, to, memory) => {
//                         if start.is_none() {
//                             *start = Some(utbox.0.clone());
//                         }
//                         let mut new_memory = Vec::new();
//                         if memory.len() != to.len() {
//                             memory = vec![[0.0, 0.0]; to.len()];
//                         }
//                         if let Some(start_val) = start {
//                             for i in 0..to.len() {
//                                 // 各頂点も lerp の考え方で補間にゃ
//                                 let start_x = start_val[i][0];
//                                 let start_y = start_val[i][1];
//                                 let to_x = to[i][0];
//                                 let to_y = to[i][1];

//                                 let lerp_x = (to_x - start_x) * ease.ease(t);
//                                 let lerp_y = (to_y - start_y) * ease.ease(t);
//                                 utbox.0[i][0] += lerp_x - memory[i][0];
//                                 utbox.0[i][1] += lerp_y - memory[i][1];
//                                 new_memory.push([lerp_x, lerp_y]);
//                             }
//                         }
//                         memory = new_memory;
//                     }
//                     BoxMovingType::MoveAngleAt(ease, start, to, angle_memory, at, at_memory) => {
//                         if start.is_none() {
//                             *start = Some(transform.rotation);
//                         }
//                         // 初回のみ at_memory を初期化
//                         if at_memory.is_none() {
//                             *at_memory = Some(transform.translation - *at);
//                         }
//                         if let (Some(start_val), Some(prev_pos)) = (start, at_memory) {
//                             let to_quat = Quat::from_rotation_z(*to);

//                             // 目標回転を球面線形補間で計算
//                             let target_rotation =
//                                 start_val.slerp(to_quat, ease.ease(t)) * start_val.inverse();
//                             // 前フレームの回転
//                             let prev_rotation = *angle_memory;
//                             // 回転も更新
//                             let rotation_delta = prev_rotation.inverse() * target_rotation;

//                             // 今フレームの回転後座標
//                             let new_rotated = rotation_delta * *prev_pos;

//                             // 差分だけ加算
//                             transform.translation += new_rotated - *prev_pos;

//                             transform.rotation = transform.rotation * rotation_delta;

//                             // メモリを更新
//                             angle_memory = target_rotation;
//                             *prev_pos = new_rotated;
//                         }
//                     }

//                     BoxMovingType::AddTo(ease, to, memory) => {
//                         let lerp = *to * ease.ease(t);
//                         transform.translation += lerp - *memory;
//                         *memory = lerp;
//                     }
//                     BoxMovingType::AddAngle(ease, angle, memory) => {
//                         let target_rotation = Quat::from_rotation_z(*angle * ease.ease(t));
//                         let rotation_delta = memory.inverse() * target_rotation;
//                         transform.rotation = transform.rotation * rotation_delta;
//                         memory = target_rotation;
//                     }
//                     BoxMovingType::AddSize(ease, size, memory) => {
//                         let lerp = *size * ease.ease(t);
//                         transform.scale += lerp - *memory;
//                         *memory = lerp;
//                     }
//                     BoxMovingType::AddShape(ease, to, memory) => {
//                         let mut new_memory = Vec::new();
//                         if memory.len() != to.len() {
//                             memory = vec![[0.0, 0.0]; to.len()];
//                         }
//                         for i in 0..to.len() {
//                             let lerp_x = to[i][0] * ease.ease(t);
//                             let lerp_y = to[i][1] * ease.ease(t);
//                             utbox.0[i][0] += lerp_x - memory[i][0];
//                             utbox.0[i][1] += lerp_y - memory[i][1];
//                             new_memory.push([lerp_x, lerp_y]);
//                         }
//                         memory = new_memory;
//                     }
//                     BoxMovingType::AddAngleAt(ease, angle, angle_memory, at, at_memory) => {
//                         // 初回のみ at_memory を初期化
//                         if at_memory.is_none() {
//                             *at_memory = Some(transform.translation - *at);
//                         }
//                         if let Some(prev_pos) = at_memory {
//                             let target_rotation = Quat::from_rotation_z(*angle * ease.ease(t));
//                             let rotation_delta = angle_memory.inverse() * target_rotation;

//                             // 今フレームの回転後座標
//                             let curr_rotated = rotation_delta * *prev_pos;

//                             // 差分だけ加算
//                             transform.translation += curr_rotated - *prev_pos;

//                             // 回転も更新
//                             transform.rotation = transform.rotation * rotation_delta;

//                             // メモリ更新
//                             angle_memory = target_rotation;
//                             *at_memory = Some(curr_rotated);
//                         }
//                     }

//                     BoxMovingType::AddShapeTranslation(ease, to, memory) => {
//                         let lerp_x = to[0] * ease.ease(t);
//                         let lerp_y = to[1] * ease.ease(t);
//                         utbox.iter_mut().for_each(|pos| {
//                             *pos = [pos[0] + lerp_x - memory[0], pos[1] + lerp_y - memory[1]];
//                         });
//                         *memory = [lerp_x, lerp_y];
//                     }

//                     BoxMovingType::AddRotationAngle(ease, angle, memory) => {
//                         let lerp_angle = *angle * ease.ease(t);
//                         let sin = (lerp_angle - *memory).sin();
//                         let cos = (lerp_angle - *memory).cos();
//                         utbox.iter_mut().for_each(|pos| {
//                             let x = pos[0];
//                             let y = pos[1];
//                             let new_x = x * cos - y * sin;
//                             let new_y = x * sin + y * cos;
//                             *pos = [new_x, new_y];
//                         });
//                         *memory = lerp_angle;
//                     }
//                     BoxMovingType::AddShapeRotationAt(ease, angle, memory, at) => {
//                         let lerp_angle = *angle * ease.ease(t);
//                         let sin = (lerp_angle - *memory).sin();
//                         let cos = (lerp_angle - *memory).cos();
//                         utbox.iter_mut().for_each(|pos| {
//                             let x = pos[0] - at[0];
//                             let y = pos[1] - at[1];
//                             let new_x = x * cos - y * sin + at[0];
//                             let new_y = x * sin + y * cos + at[1];
//                             *pos = [new_x, new_y];
//                         });
//                         *memory = lerp_angle;
//                     }
//                     _ => {}
//                 }
//             }
//             if *delay + *duration <= life_timer.0 {
//                 remove_indices.push(i);
//             }
//         }
//         for &index in remove_indices.iter().rev() {
//             box_movings.0.remove(index);
//         }
//     }
// }
