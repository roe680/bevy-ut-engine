use bevy::prelude::*;

use crate::bone::{
    bone_draw::bone_draw,
    bone_type::{BoneStyleMeshHandle, init_bone_style_mesh_handle},
};
pub struct BonePlugin;

impl Plugin for BonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bone_draw)
            .add_systems(PreStartup, init_bone_style_mesh_handle);
    }
}
