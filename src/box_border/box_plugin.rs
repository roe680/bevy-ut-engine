use bevy::{
    app::{App, Plugin, PreUpdate, Update},
    ecs::schedule::IntoScheduleConfigs,
    sprite_render::Material2dPlugin,
};

use crate::box_border::{
    animations::shape::animation_shape,
    box_draw::box_draw,
    color_scheme::box_color_scheme::BoxColorScheme,
    gpu_triangle_plugin::GpuTrianglePlugin,
    make_synthsis::{BoxSynthesis, make_synthsis},
    shader::attack_clip_sharder::AttackClipSharder,
    update_triangle::{BoxTriangle, update_triangle},
};

pub struct BoxPlugin;

impl Plugin for BoxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoxSynthesis>()
            .init_resource::<BoxTriangle>()
            // データ更新系は PreUpdate で先に実行
            .add_systems(PreUpdate, make_synthsis)
            .add_systems(PreUpdate, update_triangle.after(make_synthsis))
            .add_plugins(Material2dPlugin::<AttackClipSharder>::default())
            .add_plugins(GpuTrianglePlugin)
            // 描画は Update
            .add_systems(Update, animation_shape)
            .add_systems(Update, box_draw)
            // Box color scheme
            .init_resource::<BoxColorScheme>();
    }
}
