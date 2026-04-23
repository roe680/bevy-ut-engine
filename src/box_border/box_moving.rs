// use std::ops::{Deref, DerefMut};

// use bevy::{
//     ecs::component::Component,
//     math::{Quat, Vec3},
// };

// use crate::helpers::easing::Easing;
// #[derive(Debug, Clone, PartialEq, Default)]
// pub enum BoxMovingType {
//     //順番  easing, start, to , memoryである
//     //easing: イージングの種類
//     //startについてはデフォルトで、Noneで、これは初めて呼び出された時、その情報を維持するためにある。
//     //toは、MoveToなら移動先、MoveAngleなら回転角度、MoveSizeならサイズ、MoveShapeなら形状の情報が入る    //memoryは、MoveToなら移動前の位置、MoveAngleなら回転前の角度、MoveSizeなら移動前のサイズ、MoveShapeなら移動前の形状の情報が入る。AddToやAddAngleなどは、toに加算する値が入る。MoveAngleAtやAddAngleAtは、atに回転の中心点が入る。
//     //Move
//     #[default]
//     None,
//     MoveTo(Easing, Option<Vec3>, Vec3, Vec3),
//     MoveAngle(Easing, Option<Quat>, f32, Quat),
//     MoveSize(Easing, Option<Vec3>, Vec3, Vec3),
//     MoveShape(Easing, Option<Vec<[f32; 2]>>, Vec<[f32; 2]>, Vec<[f32; 2]>),
//     MoveAngleAt(Easing, Option<Quat>, f32, Quat, Vec3, Option<Vec3>),
//     //Add
//     AddTo(Easing, Vec3, Vec3),
//     AddAngle(Easing, f32, Quat),
//     AddSize(Easing, Vec3, Vec3),
//     AddShape(Easing, Vec<[f32; 2]>, Vec<[f32; 2]>),
//     AddAngleAt(Easing, f32, Quat, Vec3, Option<Vec3>),
//     AddShapeTranslation(Easing, [f32; 2], [f32; 2]),
//     AddRotationAngle(Easing, f32, f32),
//     AddShapeRotationAt(Easing, f32, f32, [f32; 2]),
// }

// #[derive(Debug, PartialEq, Clone)]
// pub struct BoxMoving<T>(
//     pub(super) Vec<(BoxMovingType, f32, f32, f32, f32)>,
//     std::marker::PhantomData<T>,
// );

// pub fn _create_default(moving: BoxMovingType) -> (BoxMovingType, f32, f32, f32, f32) {
//     (moving, 0.0, 0.0, 0.0, 1.0)
// }

// // dslにしたい
// impl<T> BoxMoving<T> {
//     pub fn new() -> Self {
//         BoxMoving::<T>(vec![], std::marker::PhantomData)
//     }

//     pub fn move_to(mut self, to: Vec3, ease: Easing) -> Self {
//         self.0.push(_create_default(BoxMovingType::MoveTo(
//             ease,
//             None,
//             to,
//             Vec3::ZERO,
//         )));
//         self
//     }

//     pub fn move_angle(mut self, angle: f32, ease: Easing) -> Self {
//         self.0.push(_create_default(BoxMovingType::MoveAngle(
//             ease,
//             None,
//             angle,
//             Quat::IDENTITY,
//         )));
//         self
//     }

//     pub fn move_size(mut self, size: Vec3, ease: Easing) -> Self {
//         self.0.push(_create_default(BoxMovingType::MoveSize(
//             ease,
//             None,
//             size,
//             Vec3::ZERO,
//         )));
//         self
//     }

//     pub fn move_angle_at(mut self, angle: f32, at: Vec3, ease: Easing) -> Self {
//         self.0.push(_create_default(BoxMovingType::MoveAngleAt(
//             ease,
//             None,
//             angle,
//             Quat::IDENTITY,
//             at,
//             None,
//         )));
//         self
//     }

//     pub fn add_to(mut self, to: Vec3, ease: Easing) -> Self {
//         self.0
//             .push(_create_default(BoxMovingType::AddTo(ease, to, Vec3::ZERO)));
//         self
//     }

//     pub fn add_angle(mut self, angle: f32, ease: Easing) -> Self {
//         self.0.push(_create_default(BoxMovingType::AddAngle(
//             ease,
//             angle,
//             Quat::IDENTITY,
//         )));
//         self
//     }

//     pub fn add_size(mut self, size: Vec3, ease: Easing) -> Self {
//         self.0.push(_create_default(BoxMovingType::AddSize(
//             ease,
//             size,
//             Vec3::ZERO,
//         )));
//         self
//     }

//     pub fn add_angle_at(mut self, angle: f32, at: Vec3, ease: Easing) -> Self {
//         self.0.push(_create_default(BoxMovingType::AddAngleAt(
//             ease,
//             angle,
//             Quat::IDENTITY,
//             at,
//             None,
//         )));
//         self
//     }

//     pub fn add_shape_translation(mut self, to: [f32; 2], ease: Easing) -> Self {
//         self.0
//             .push(_create_default(BoxMovingType::AddShapeTranslation(
//                 ease,
//                 to,
//                 [0.0, 0.0],
//             )));
//         self
//     }

//     pub fn add_shape_rotation(mut self, angle: f32, ease: Easing) -> Self {
//         self.0.push(_create_default(BoxMovingType::AddRotationAngle(
//             ease, angle, 0.0,
//         )));
//         self
//     }

//     pub fn add_shape_rotation_at(mut self, angle: f32, at: [f32; 2], ease: Easing) -> Self {
//         self.0
//             .push(_create_default(BoxMovingType::AddShapeRotationAt(
//                 ease, angle, 0.0, at,
//             )));
//         self
//     }

//     pub fn duration(mut self, duration: f32) -> Self {
//         self.0.last_mut().expect("addされてないよ!").2 = duration;
//         self
//     }

//     pub fn delay(mut self, delay: f32) -> Self {
//         self.0.last_mut().expect("addされてないよ!").1 = delay;
//         self
//     }

//     pub fn to_component(self) -> BoxMovingComponent {
//         BoxMovingComponent(self.0)
//     }
// }

// #[derive(Debug, Clone, PartialEq, Default, Component)]
// pub struct BoxMovingComponent(pub Vec<(BoxMovingType, f32, f32, f32, f32)>);

// impl Deref for BoxMovingComponent {
//     type Target = Vec<(BoxMovingType, f32, f32, f32, f32)>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for BoxMovingComponent {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
