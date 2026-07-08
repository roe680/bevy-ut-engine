use crate::box_border::shader::attack_clip_sharder::{AttackClipSharder, TRIANGLE_GPU_BUFFER};
use crate::box_border::update_triangle::{BoxTriangle, GpuTriangle};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::render::{
    extract_resource::ExtractResourcePlugin,
    render_asset::prepare_assets,
    render_resource::{BufferUsages, RawBufferVec},
    renderer::{RenderDevice, RenderQueue},
    Render, RenderApp, RenderSystems,
};
use bevy::sprite_render::PreparedMaterial2d;

/// レンダーワールドに保持される三角形バッファリソース（Step 2）
/// `RawBufferVec<GpuTriangle>` — 動的サイズの配列を効率的に GPU へ転送
#[derive(Resource)]
pub struct GpuTriangleBuffer {
    pub buffer: RawBufferVec<GpuTriangle>,
}

/// StorageBuffer にデータを書き込み、GPU へアップロード + グローバル static に Buffer を格納（Step 3: Prepare）
fn prepare_triangle_buffer(
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    mut gpu_triangle: ResMut<GpuTriangleBuffer>,
    box_triangle: Res<BoxTriangle>,
) {
    gpu_triangle.buffer.clear();

    for t in &box_triangle.0 {
        gpu_triangle.buffer.push(GpuTriangle {
            p0: Vec2::from(t[0]),
            p1: Vec2::from(t[1]),
            p2: Vec2::from(t[2]),
        });
    }

    gpu_triangle.buffer.write_buffer(&device, &queue);

    // アップロードされた生の wgpu Buffer をグローバル static に格納
    // AttackClipSharder の AsBindGroup 実装がこれを読み取ってバインドグループに使用する（Step 4）
    if let Some(buf) = gpu_triangle.buffer.buffer() {
        if let Ok(mut guard) = TRIANGLE_GPU_BUFFER.lock() {
            *guard = Some(buf.clone());
        }
    }
}

pub struct GpuTrianglePlugin;

impl Plugin for GpuTrianglePlugin {
    fn build(&self, app: &mut App) {
        // ExtractResourcePlugin が BoxTriangle をレンダーワールドに自動抽出（Step 3: Extract）
        app.add_plugins(ExtractResourcePlugin::<BoxTriangle>::default());

        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .insert_resource(GpuTriangleBuffer {
                    buffer: RawBufferVec::new(BufferUsages::STORAGE),
                })
                .add_systems(
                    Render,
                    prepare_triangle_buffer
                        .in_set(RenderSystems::PrepareAssets)
                        .before(prepare_assets::<PreparedMaterial2d<AttackClipSharder>>),
                );
        }
    }
}
