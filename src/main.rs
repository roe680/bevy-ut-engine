mod color_scheme;
use bevy::prelude::*;
use bevy::render::storage::ShaderBuffer;
use bevy::window::WindowResolution;
use bevy_framepace::FramepacePlugin;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::PI;

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
        .add_plugins(
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
        )
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
    cmds.spawn(spawn_camera(60, FPS_LAYER));
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
    // まぁつまり、形にシェーダー貼り付けて描画する。
    let test_mesh = meshes.add(Rectangle::new(80., 80.)); //描画するためのmeshを作る。今回は四角形。
    // この、spawn_batchは、イテレータにできるものを効率的にスポーンするためのもの。今回は、spawn_vecsという、fromとtoを作って、回数と増え幅を指定し、deltaからcomponentを返すやつ。
    cmds.spawn_batch(spawn_vecs(0., 360. / 360., 360, |delta| {
        (
            Mesh2d(test_mesh.clone()), //同じものを使えば効率的なので、ちなみにこれはハンドルをクローンしている
            INBOX_ATTACK_LAYER,        //枠の内側に収まるように描画するためのレイヤー
            MeshMaterial2d(attack_shader.add(AttackClipSharder::new(
                assets.load("test4.png"), //シェーダーに送る画像、Meshにこの画像が貼り付けられる。
                &mut buffers,             //インデックスバッファ作成用
                triangles_handle.0.clone(), //共有三角形バッファのハンドル
            ))),
            Transform {
                translation: Vec3::new(
                    delta.to_radians().sin() * 100.,
                    delta.to_radians().cos() * 100.,
                    delta,
                ),
                ..default()
            },
        )
    }));
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

    // === 負荷テスト: 放射状波バースト (360本, LiveDuration付き) ===
    // 全ボーンが中心から波状に放射、古いものは自動消滅
    cmds.spawn_batch(spawn_vecs(0., 1., 360, |delta| {
        let rad = (delta * 360. / 360.).to_radians();
        let wave = delta / 360. * 2.5;
        (
            Bone,
            AttackType::Normal,
            Transform {
                translation: Vec3::ZERO,
                rotation: Quat::from_rotation_z(rad),
                ..default()
            },
            BoneLength(3.0),
            LiveDuration(9.0),
            Animations::<BoneLengthAnim>::new()
                .move_len(90.0, EaseFunction::SineOut)
                .set_delay(wave)
                .set_duration(2.0)
                .add_len(-40.0, EaseFunction::SineInOut)
                .set_delay(wave + 2.5)
                .set_duration(1.5)
                .add_len(30.0, EaseFunction::SineInOut)
                .set_delay(wave + 4.5)
                .set_duration(1.0),
            Animations::new()
                .add_to(
                    Vec3::new(rad.cos() * 200., rad.sin() * 200., 0.),
                    EaseFunction::SineOut,
                )
                .set_delay(wave)
                .set_duration(2.0),
            Animations::new()
                .add_angle(8. * PI, EaseFunction::Linear)
                .set_delay(wave)
                .set_duration(6.0),
        )
    }));

    // === 負荷テスト: 多重リング公転 (5重, 計180本, 段階出現) ===
    for ring in 0..5 {
        let radius = 60. + ring as f32 * 35.;
        let count = 12 * (ring + 1);
        let ring_delay = ring as f32 * 1.5;
        cmds.spawn_batch(spawn_vecs(0., 360. / count as f32, count, |delta| {
            let rad = delta.to_radians();
            let inner_delay = delta / count as f32 * 0.5;
            (
                Bone,
                AttackType::MustMove,
                Transform {
                    translation: Vec3::new(radius * rad.cos(), radius * rad.sin(), 0.),
                    rotation: Quat::from_rotation_z(rad),
                    ..default()
                },
                BoneLength(5.0),
                LiveDuration(12.0),
                Animations::<BoneLengthAnim>::new()
                    .move_len(35., EaseFunction::SineInOut)
                    .set_delay(ring_delay + inner_delay)
                    .set_duration(1.0 + ring as f32 * 0.2)
                    .add_len(-15., EaseFunction::SineInOut)
                    .set_delay(ring_delay + inner_delay + 1.0)
                    .set_duration(1.0),
                Animations::new()
                    .add_angle_at(
                        (3. - ring as f32) * 2. * PI,
                        Vec3::ZERO,
                        EaseFunction::Linear,
                    )
                    .set_delay(ring_delay + inner_delay)
                    .set_duration(4. + ring as f32 * 0.5),
                Animations::new()
                    .add_angle(4. * PI, EaseFunction::Linear)
                    .set_delay(ring_delay + inner_delay)
                    .set_duration(3.0),
            )
        }));
    }

    // === 負荷テスト: スパイラル波 (200本, 連続出現) ===
    cmds.spawn_batch(spawn_vecs(0., 1., 200, |delta| {
        let t = delta / 200.;
        let angle = t * 360. * 3.;
        let rad = angle.to_radians();
        let r = 30. + t * 140.;
        let stagger = delta / 200. * 3.0;
        (
            Bone,
            AttackType::MustNotMove,
            Transform {
                translation: Vec3::new(r * rad.cos(), r * rad.sin(), 0.),
                rotation: Quat::from_rotation_z(rad),
                ..default()
            },
            BoneLength(5.0),
            LiveDuration(8.0),
            Animations::<BoneLengthAnim>::new()
                .move_len(20. + t * 30., EaseFunction::SineOut)
                .set_delay(stagger)
                .set_duration(2.0)
                .add_len(-15., EaseFunction::SineInOut)
                .set_delay(stagger + 2.5)
                .set_duration(1.5),
            Animations::new()
                .add_angle(6. * PI, EaseFunction::Linear)
                .set_delay(stagger)
                .set_duration(4.0),
        )
    }));

    // === 負荷テスト: 高速振動リング (20本, 短寿命) ===
    cmds.spawn_batch(spawn_vecs(0., 360. / 20., 20, |delta| {
        let rad = delta.to_radians();
        (
            Bone,
            AttackType::Normal,
            Transform {
                translation: Vec3::new(180. * rad.cos(), 180. * rad.sin(), 0.),
                rotation: Quat::from_rotation_z(rad),
                ..default()
            },
            BoneLength(5.0),
            LiveDuration(4.0),
            Animations::<BoneLengthAnim>::new()
                .move_len(80., EaseFunction::SineInOut)
                .set_duration(0.3)
                .move_len(5., EaseFunction::SineInOut)
                .set_duration(0.3)
                .move_len(80., EaseFunction::SineInOut)
                .set_duration(0.3)
                .move_len(5., EaseFunction::SineInOut)
                .set_duration(0.3),
        )
    }));
}
