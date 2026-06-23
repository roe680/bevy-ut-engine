use bevy::{
    app::{App, Plugin, PreStartup, PreUpdate, Update},
    ecs::schedule::IntoScheduleConfigs,
};

use crate::box_border::{
    animations::shape::animation_shape,
    box_draw::box_draw,
    color_scheme::box_color_scheme::BoxColorScheme,
    make_synthsis::{BoxSynthesis, make_synthsis},
    shader::attack_clip_sharder::{AttackClipSharder, init_attack_clip_buffer_buffer},
    update_triangle::{BoxTriangle, update_triangle},
};

pub struct BoxPlugin;

impl Plugin for BoxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoxSynthesis>()
            .init_resource::<BoxTriangle>()
            // データ更新系は PreUpdate で先に実行
            .add_systems(PreUpdate, make_synthsis)
            .add_systems(PreStartup, init_attack_clip_buffer_buffer)
            .add_plugins(bevy::sprite_render::Material2dPlugin::<AttackClipSharder>::default())
            .add_systems(PreUpdate, update_triangle.after(make_synthsis))
            // 描画は Update
            .add_systems(Update, animation_shape)
            .add_systems(Update, box_draw)
            //Box color scheme
            .init_resource::<BoxColorScheme>();
    }
}
