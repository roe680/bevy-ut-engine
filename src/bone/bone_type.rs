use bevy::ecs::{component::Component, lifecycle::Add, observer::On};

use crate::bone::bone::Bone;
use bevy::prelude::*;

//bone の画像部分のメッシュのハンドル

#[derive(Debug, Clone, PartialEq, Resource, Default)]
pub struct BoneStyleMeshHandle(pub Handle<Mesh>);
/*  */
#[derive(Debug, Clone, PartialEq, Component, Default)]
pub enum BoneStyle {
    #[default]
    Sans,
    Papyrus,
}

pub fn init_bone_style_mesh_handle(mut cmds: Commands, mut mesh: ResMut<Assets<Mesh>>) {
    let rect = mesh.add(Rectangle::new(3., 3.));
    cmds.insert_resource(BoneStyleMeshHandle(rect));
}

pub fn on_add_attach_bone_style(
    mut cmds: Commands,
    asset_server: AssetServer,
    trigger: On<Add, BoneStyle>,
    query: Query<&BoneStyle, With<Bone>>,
    mesh_handle: Res<BoneStyleMeshHandle>,
) {
    let entity = trigger.entity;
    if let Ok(bone_style) = query.get(entity) {
        let picture: Handle<Image> = match bone_style {
            BoneStyle::Sans => asset_server.load("bone.png"),
            BoneStyle::Papyrus => asset_server.load("bone2.png"),
        };
        let child = cmds.spawn(Mesh2d(mesh_handle.0.clone())).id();
        cmds.entity(trigger.entity).add_child(child);
    }
}

//イベントは、まとめて次のフレームで処理する
//
