use bevy::{
    app::{App, Plugin, PreUpdate, Startup, Update},
    ecs::schedule::IntoScheduleConfigs,
    prelude::*,
    render::storage::ShaderBuffer,
    sprite_render::Material2dPlugin,
};

use crate::box_border::{
    animations::shape::animation_shape,
    box_draw::box_draw,
    color_scheme::box_color_scheme::BoxColorScheme,
    make_synthsis::{BoxSynthesis, make_synthsis},
    shader::attack_clip_sharder::AttackClipSharder,
    update_triangle::{BoxTriangle, TrianglesBufferHandle, update_triangle},
};

fn init_triangles_buffer(mut buffers: ResMut<Assets<ShaderBuffer>>, mut cmds: Commands) {
    let handle = buffers.add(Vec::<[[f32; 2]; 3]>::new());
    cmds.insert_resource(TrianglesBufferHandle(handle));
}

pub struct BoxPlugin;

impl Plugin for BoxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoxSynthesis>()
            .init_resource::<BoxTriangle>()
            // 共有三角形バッファの初期化
            .add_systems(PreStartup, init_triangles_buffer)
            // データ更新系は PreUpdate で先に実行
            .add_systems(PreUpdate, make_synthsis)
            .add_systems(PreUpdate, update_triangle.after(make_synthsis))
            .add_plugins(Material2dPlugin::<AttackClipSharder>::default())
            // 描画は Update
            .add_systems(Update, animation_shape)
            .add_systems(Update, box_draw)
            // Box color scheme
            .init_resource::<BoxColorScheme>();
    }
}
