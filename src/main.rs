//test
use bevy::prelude::*;
use bevy::{render::storage::ShaderStorageBuffer, window::WindowResolution};
use bevy_vector_shapes::prelude::*;
use std::f32::consts::PI;

use crate::{
    box_border::{
        // box_moving::BoxMoving,
        box_struct::{BoxType, BoxZIndex},
        boxs::rect::RectBox,
        shader::attack_clip_sharder::{AttackClipBufferHandle, AttackClipSharder},
    },
    helpers::{
        helpers::spawn_vecs,
        render_layers::{BOX_LAYER, BOX_LINE_LAYER, FPS_LAYER, INBOX_ATTACK_LAYER},
        spawn_camera::spawn_camera,
        time::SpawnDelay,
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
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>, //GPUに送られる、データたち(配列系は特にここに入れられる(ちなみに全部詰まってるから鍵(handle)が必要))
    triangle_buffer: Res<AttackClipBufferHandle>,     //↑の鍵,triangleを元に、クリップするので
) {
    let test2 = color_materials.add(ColorMaterial {
        color: Color::WHITE,
        ..default()
    });
    //仕組み解説
    // まぁつまり、形にシェーダー貼り付けて描画する。
    let test_mesh = meshes.add(Rectangle::new(80., 80.)); //描画するためのmeshを作る。今回は四角形。
                                                          //この、spawn_batchは、イテレータにできるものを効率的にスポーンするためのもの。今回は、spawn_vecsという、fromとtoを作って、回数と増え幅を指定し、deltaからcomponentを返すやつ。
    cmds.spawn_batch(spawn_vecs(0., 360. / 20., 20, |delta| {
        (
            Mesh2d(test_mesh.clone()), //同じものを使えば効率的なので、ちなみにこれはハンドルをクローンしている
            INBOX_ATTACK_LAYER,        //枠の内側に収まるように描画するためのレイヤー
            MeshMaterial2d(attack_shader.add(AttackClipSharder::new(
                assets.load("test4.png"), //シェーダーに送る画像、Meshにこの画像が貼り付けられる。
                &mut buffers,             //このフレームワークで共有されるbufferたち
                triangle_buffer.0.clone(), //鍵(Handle)
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
}
