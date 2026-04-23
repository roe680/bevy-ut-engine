use bevy::{
    app::{App, Plugin, PreStartup, PreUpdate, Update},
    ecs::schedule::IntoScheduleConfigs,
};

use crate::{
    box_border::{
        box_drawer::box_draw,
        box_moving_fn::move_box,
        make_synthsis::{BoxSynthesis, make_synthsis},
    },
    helpers::{
        shader::attack_clip_sharder::{AttackClipSharder, setup_attack_clip_buffer_buffer},
        time::{add_time, remove_entity_on_timer, spawn_entity_on_timer},
    },
};

pub struct UTEnginePlugin;

impl Plugin for UTEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, box_draw)
            .add_systems(Update, add_time)
            .add_systems(
                Update,
                (spawn_entity_on_timer, remove_entity_on_timer).chain(),
            )
            .add_systems(PreStartup, setup_attack_clip_buffer_buffer)
            .add_plugins(bevy::sprite_render::Material2dPlugin::<AttackClipSharder>::default())
            .add_systems(Update, move_box)
            .add_systems(Update, remove_entity_on_timer)
            .init_resource::<BoxSynthesis>()
            .add_systems(PreUpdate, make_synthsis);
    }
}
