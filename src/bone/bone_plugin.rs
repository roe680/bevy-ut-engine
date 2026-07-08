use bevy::prelude::*;

use crate::bone::{
    animations::animation_bone_length, bone_draw::bone_draw, bone_type::init_bone_style_mesh_handle,
};
pub struct BonePlugin;

impl Plugin for BonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animation_bone_length)
            .add_systems(Update, bone_draw.after(animation_bone_length))
            .add_systems(PreStartup, init_bone_style_mesh_handle);
    }
}
