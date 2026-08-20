mod color_scheme;
mod fullscreen_shader;
mod soul;
use bevy::window::WindowResolution;
use bevy::{prelude::*, render::storage::ShaderBuffer};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_vector_shapes::prelude::*;
use std::f32::consts::PI;

use crate::helpers::render_layers::SOUL_LAYER;
use crate::{
    bone::{
        animations::bone_length::BoneLengthAnim,
        bone::{Bone, BoneLength},
    },
    box_border::{
        // box_moving::BoxMoving,
        box_struct::{BoxType, BoxZIndex},
        boxs::rect::RectBox,
        shader::attack_clip_sharder::AttackClipSharder,
        update_triangle::TrianglesBufferHandle,
    },
    fullscreen_shader::effect::{FullscreenEffect, FullscreenEffectPlugin},
    helpers::{
        attack::attack_type::AttackType,
        helpers::spawn_vecs,
        render_layers::{BOX_LAYER, BOX_LINE_LAYER, FPS_LAYER, INBOX_ATTACK_LAYER},
        spawn_camera::spawn_camera,
        time::{LiveDuration, SpawnDelay},
    },
    move_animation::moving::Animations,
    plugin::UTEnginePlugin,
};
mod bone;
mod box_border;
mod helpers;
mod move_animation;
mod plugin;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        name: Some("BoxBorder".to_string()),
                        title: "Box Border Example".to_string(),
                        resolution: WindowResolution::new(640, 480).with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            FullscreenEffectPlugin,
        ))
        .add_plugins(FramepacePlugin)
        .insert_resource(FramepaceSettings::default().with_limiter(Limiter::from_framerate(60.0)))
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(UTEnginePlugin)
        .add_systems(Startup, (camera_setup, test))
        .add_systems(Startup, crate::helpers::fps_score_write::setup_fps)
        .add_systems(Update, crate::helpers::fps_score_write::update_fps)
        .run();
}
fn camera_setup(mut cmds: Commands) {
    cmds.spawn(spawn_camera(1, BOX_LAYER));
    cmds.spawn(spawn_camera(2, INBOX_ATTACK_LAYER));
    cmds.spawn(spawn_camera(3, BOX_LINE_LAYER));
    cmds.spawn(spawn_camera(4, SOUL_LAYER));
    // cmds.spawn((spawn_camera(60, FPS_LAYER), FullscreenEffect::new(0.0)));
}
//AssetServerはファイル、AssetsはRustコードとかのやつ
fn test(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut attack_shader: ResMut<Assets<AttackClipSharder>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>, //GPUに送られる、データたち(配列系は特にここに入れられる(ちなみに全部詰まってるから鍵(handle)が必要))
    triangles_handle: Res<TrianglesBufferHandle>,
) {
    let test2 = color_materials.add(ColorMaterial {
        color: Color::WHITE,
        ..default()
    });
    //仕組み解説
    // // まぁつまり、形にシェーダー貼り付けて描画する。
    // let test_mesh = meshes.add(Rectangle::new(80., 80.)); //描画するためのmeshを作る。今回は四角形。かみげーやね
    // // この、spawn_batchは、イテレータにできるものを効率的にスポーンするためのもの。今回は、spawn_vecsという、fromとtoを作って、回数と増え幅を指定し、deltaからcomponentを返すやつ。
    // cmds.spawn_batch(spawn_vecs(0., 360. / 360., 360, |delta| {
    //     (
    //         Mesh2d(test_mesh.clone()), //同じものを使えば効率的なので、ちなみにこれはハンドルをクローンしている
    //         INBOX_ATTACK_LAYER,        //枠の内側に収まるように描画するためのレイヤー
    //         MeshMaterial2d(attack_shader.add(AttackClipSharder::new(
    //             assets.load("test/test4.png"), //シェーダーに送る画像、Meshにこの画像が貼り付けられる。
    //             &mut buffers,                  //インデックスバッファ作成用
    //             triangles_handle.0.clone(),    //共有三角形バッファのハンドル
    //         ))),
    //         Transform {
    //             translation: Vec3::new(
    //                 delta.to_radians().sin() * 100.,
    //                 delta.to_radians().cos() * 100.,
    //                 delta,
    //             ),
    //             ..default()
    //         },
    //     )
    // }));
    //BoxZIndexは順番です。小さい方からUnionやDifferenceされます
    cmds.spawn((
        RectBox::new(-50., -50., 100., 100.).add_translation(40., 30.),
        BoxType::Union,
        BoxZIndex(1),
        Animations::new()
            .move_rect(-100., -100., 200., 200., EaseFunction::Linear)
            .set_duration(2.),
        Animations::new()
            .add_angle(-2. * PI * 400., EaseFunction::Linear)
            .set_delay(6.)
            .set_duration(3200.),
    ));

    cmds.spawn((
        RectBox::default(),
        BoxType::Difference,
        BoxZIndex(2), //2の方が大きいから、優先順位高い。
        Transform {
            translation: Vec3::new(-30.0, -30.0, 0.0),
            rotation: Quat::from_rotation_z(45.0_f32.to_radians()),
            ..default()
        },
        Animations::new()
            .move_angle(360.0_f32.to_radians(), EaseFunction::Linear)
            .set_duration(2.),
        Animations::new()
            .add_to(Vec3::new(30., -30., 0.), EaseFunction::SineIn)
            .set_delay(1.)
            .set_duration(1.),
    ));

    cmds.spawn_batch(spawn_vecs(0., 360. / 6., 6, |delta| {
        (
            BoxZIndex(-1),
            RectBox::new(-25., -25., 50., 50.),
            Transform {
                translation: Vec3::new(0., -60., 0.),
                rotation: Quat::from_rotation_z(delta.to_radians()),
                ..default()
            },
            Animations::new()
                .add_to(
                    Vec3::new(
                        150. * delta.to_radians().cos(),
                        150. * delta.to_radians().sin(),
                        0.,
                    ),
                    EaseFunction::Linear,
                )
                .set_duration(1.),
            Animations::new()
                .add_angle_at(
                    2. * PI * 400.,
                    Vec3::new(0., -60., 0.),
                    EaseFunction::Linear,
                )
                .set_delay(1.)
                .set_duration(3200.),
            Animations::new()
                .move_rect(-35., -35., 70., 70., EaseFunction::BounceOut)
                .set_delay(1.5)
                .set_duration(1.),
            SpawnDelay(4.5),
        )
    }));

    cmds.spawn_batch(spawn_vecs(0., 90., 2, |delta| {
        (
            BoxZIndex(4),
            Transform {
                translation: Vec3::new(0., -60., 0.),
                rotation: Quat::from_rotation_z(delta.to_radians()),
                ..default()
            },
            RectBox::new(-0., -0., 0., 0.),
            Animations::new()
                .add_angle(360.0_f32.to_radians(), EaseFunction::Linear)
                .set_duration(2.)
                .set_delay(2.5),
            Animations::new()
                .move_rect(-25., -100.1, 50., 200.2, EaseFunction::BounceIn)
                .set_duration(2.),
            SpawnDelay(1.),
            BoxType::Difference,
        )
    }));

    cmds.spawn_batch(spawn_vecs(30., 360. / 6., 6, |delta| {
        (
            BoxZIndex(3),
            BoxType::Difference,
            RectBox::new(-25., -25., 50., 50.),
            Transform {
                translation: Vec3::new(0., -60., 0.),
                rotation: Quat::from_rotation_z(delta.to_radians()),
                ..default()
            },
            Animations::new()
                .add_to(
                    Vec3::new(
                        130. * delta.to_radians().cos(),
                        130. * delta.to_radians().sin(),
                        0.,
                    ),
                    EaseFunction::Linear,
                )
                .set_duration(1.),
            Animations::new()
                .add_angle_at(
                    -2. * PI * 400.,
                    Vec3::new(0., -60., 0.),
                    EaseFunction::Linear,
                )
                .set_delay(1.)
                .set_duration(3200.),
            Animations::new()
                .move_rect(-30., -30., 60., 60., EaseFunction::BounceOut)
                .set_delay(1.5)
                .set_duration(1.),
            SpawnDelay(4.5),
        )
    }));

    // === 弾幕①: 二重星バースト (360本, 白/青交互の反対渦) ===
    // 中心から波面を描きながら放射し、飛び切った後は
    // 隣同士が逆向きに公転して万華鏡のような渦を作る
    cmds.spawn_batch(spawn_vecs(0., 1., 360, |delta| {
        let rad = delta.to_radians();
        let wave = delta / 360. * 2.5; // 波面の遅延
        let even = delta as i32 % 2 == 0;
        let spin = if even { 6. * PI } else { -6. * PI };
        (
            Bone,
            if even {
                AttackType::Normal
            } else {
                AttackType::MustNotMove
            },
            Transform {
                translation: Vec3::ZERO,
                rotation: Quat::from_rotation_z(rad),
                ..default()
            },
            BoneLength(4.0),
            LiveDuration(9.5),
            SpawnDelay(1.5),
            Animations::<BoneLengthAnim>::new()
                .move_len(85.0, EaseFunction::BackOut)
                .set_delay(wave)
                .set_duration(1.6)
                .add_len(-30.0, EaseFunction::SineInOut)
                .set_delay(wave + 2.0)
                .set_duration(1.4),
            Animations::new()
                .add_to(
                    Vec3::new(rad.cos() * 235., rad.sin() * 235., 0.),
                    EaseFunction::QuadraticOut,
                )
                .set_delay(wave)
                .set_duration(1.9),
            Animations::new()
                .add_angle(spin, EaseFunction::Linear)
                .set_delay(wave)
                .set_duration(2.2),
            Animations::new()
                .add_angle_at(spin * 0.25, Vec3::ZERO, EaseFunction::SineInOut)
                .set_delay(wave + 2.0)
                .set_duration(6.5),
        )
    }));

    // === 弾幕②: 多色リングの呼吸 (5重, 計180本) ===
    // 外側ほど本数が増える光輪。色はリングごとに違い、
    // 隣り合うリングは逆方向へ回り、脈動が輪を一周していく
    for ring in 0..5 {
        let radius = 55. + ring as f32 * 38.;
        let count = 12 * (ring + 1);
        let dir = if ring % 2 == 0 { 1.0 } else { -1.0 };
        let color = match ring % 4 {
            0 => AttackType::MustNotMove,
            1 => AttackType::Normal,
            2 => AttackType::MustMove,
            _ => AttackType::Normal,
        };
        cmds.spawn_batch(spawn_vecs(0., 360. / count as f32, count, move |delta| {
            let rad = delta.to_radians();
            let inner_delay = delta / count as f32 * 0.5;
            let pulse = delta / count as f32; // 輪を伝わる脈動の位相
            (
                Bone,
                color,
                Transform {
                    translation: Vec3::new(radius * rad.cos(), radius * rad.sin(), 0.),
                    rotation: Quat::from_rotation_z(rad),
                    ..default()
                },
                BoneLength(5.0),
                LiveDuration(10.0),
                SpawnDelay(5.0 + ring as f32 * 0.5),
                Animations::<BoneLengthAnim>::new()
                    .move_len(30., EaseFunction::BackOut)
                    .set_delay(inner_delay)
                    .set_duration(1.0 + ring as f32 * 0.2)
                    .add_len(13., EaseFunction::SineInOut)
                    .set_delay(1.2 + pulse * 0.8)
                    .set_duration(1.0)
                    .add_len(-25., EaseFunction::SineInOut)
                    .set_delay(2.6)
                    .set_duration(1.8),
                Animations::new()
                    .add_angle_at(
                        dir * (2.0 + ring as f32 * 0.4) * PI,
                        Vec3::ZERO,
                        EaseFunction::SineInOut,
                    )
                    .set_delay(0.4 + inner_delay)
                    .set_duration(7.0),
                Animations::new()
                    .add_angle(dir * 2. * PI, EaseFunction::Linear)
                    .set_delay(inner_delay)
                    .set_duration(2.5),
            )
        }));
    }

    // === 弾幕③: 双螺旋 (200本, 互い違いに巻き合う2色の渦) ===
    // 前半(青)と後半(白)が180°ずれて螺旋を組み、
    // 飛び出した後は逆方向に公転してDNAのように絡み合う
    cmds.spawn_batch(spawn_vecs(0., 1., 200, |delta| {
        let t = delta / 200.;
        let helix = delta >= 100.;
        let t2 = if helix { t - 0.5 } else { t };
        let rad = (t2 * 360. * 2.5).to_radians() + if helix { PI } else { 0.0 };
        let r = 40. + t2 * 130.;
        let stagger = t * 2.5;
        let dir = if helix { -1.0 } else { 1.0 };
        (
            Bone,
            if helix {
                AttackType::Normal
            } else {
                AttackType::MustNotMove
            },
            Transform {
                translation: Vec3::new(r * rad.cos(), r * rad.sin(), 0.),
                rotation: Quat::from_rotation_z(rad),
                ..default()
            },
            BoneLength(5.0),
            LiveDuration(9.0),
            SpawnDelay(13.0),
            Animations::<BoneLengthAnim>::new()
                .move_len(18. + t2 * 45., EaseFunction::BackOut)
                .set_delay(stagger)
                .set_duration(1.8)
                .add_len(-12., EaseFunction::SineInOut)
                .set_delay(stagger + 2.2)
                .set_duration(1.2),
            Animations::new()
                .add_angle(dir * 3. * PI, EaseFunction::Linear)
                .set_delay(stagger)
                .set_duration(2.0),
            Animations::new()
                .add_angle_at(dir * 1.4 * PI, Vec3::ZERO, EaseFunction::SineInOut)
                .set_delay(stagger + 2.2)
                .set_duration(5.5),
        )
    }));

    // === 弾幕④: 弾ける光輪 (20本, 交互に脈打つリング) ===
    // 白と橙が交互に長さをパチパチと弾ませ、
    // 膨らんだり縮んだりしながらゆっくり回る
    cmds.spawn_batch(spawn_vecs(0., 360. / 20., 20, |delta| {
        let rad = delta.to_radians();
        let even = delta as i32 % 2 == 0;
        let breathe = if even { 1.0 } else { -1.0 };
        (
            Bone,
            if even {
                AttackType::Normal
            } else {
                AttackType::MustMove
            },
            Transform {
                translation: Vec3::new(180. * rad.cos(), 180. * rad.sin(), 0.),
                rotation: Quat::from_rotation_z(rad),
                ..default()
            },
            BoneLength(5.0),
            LiveDuration(5.0),
            SpawnDelay(19.0),
            Animations::<BoneLengthAnim>::new()
                .move_len(65., EaseFunction::SineInOut)
                .set_duration(0.18)
                .move_len(8., EaseFunction::SineInOut)
                .set_delay(0.2)
                .set_duration(0.14)
                .move_len(55., EaseFunction::SineInOut)
                .set_delay(0.36)
                .set_duration(0.18)
                .move_len(5., EaseFunction::SineInOut)
                .set_delay(0.56)
                .set_duration(0.14),
            Animations::new()
                .add_to(
                    Vec3::new(-breathe * rad.cos() * 40., -breathe * rad.sin() * 40., 0.),
                    EaseFunction::SineInOut,
                )
                .set_delay(0.8)
                .set_duration(0.9)
                .add_to(
                    Vec3::new(breathe * rad.cos() * 48., breathe * rad.sin() * 48., 0.),
                    EaseFunction::SineInOut,
                )
                .set_delay(1.9)
                .set_duration(0.9),
            Animations::new()
                .add_angle_at(breathe * 0.8 * PI, Vec3::ZERO, EaseFunction::Linear)
                .set_delay(1.0)
                .set_duration(3.0),
        )
    }));
}
