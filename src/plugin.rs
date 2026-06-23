use bevy::app::{App, Plugin, PreStartup, Update};
use bevy::prelude::IntoScheduleConfigs;

use crate::bone::bone_plugin::BonePlugin;
use crate::color_scheme::color_scheme_plugin::UTSchemePlugin;
use crate::{
    box_border::box_plugin::BoxPlugin,
    helpers::time::{add_time, remove_entity_on_timer, spawn_entity_on_timer},
    move_animation::ut_animation_plugin::UTAnimationPlugin,
};

pub struct UTEnginePlugin;

impl Plugin for UTEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UTAnimationPlugin)
            .add_plugins(BoxPlugin)
            .add_plugins(UTSchemePlugin)
            .add_plugins(BonePlugin)
            .add_systems(Update, add_time)
            .add_systems(
                Update,
                (spawn_entity_on_timer, remove_entity_on_timer).chain(),
            );
    }
}
