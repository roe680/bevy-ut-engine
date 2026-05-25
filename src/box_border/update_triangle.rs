use crate::{
    box_border::{
        make_synthsis::BoxSynthesis,
        shader::attack_clip_sharder::{AttackClipBufferHandle, AttackClipSharder},
    },
    helpers::spatial_partition::{
        calculate_global_bounds, collect_triangle_indices_for_points, OptimizedQuadTree,
    },
};
use bevy::{prelude::*, render::storage::ShaderStorageBuffer};
use bevy_mesh::VertexAttributeValues;
use i_triangle::float::triangulatable::Triangulatable;

/// CPU 側に保持する三角形キャッシュ。
/// 描画側 (`box_drawer`) はこの Resource を参照して描画する。
#[derive(Resource, Debug, Clone, Default)]
pub struct BoxTriangle(pub Vec<[[f32; 2]; 3]>);

/// 三角形データを生成し、
/// - `BoxTriangle` (CPU キャッシュ) を更新
/// - `AttackClipBufferHandle` が指す三角形バッファ (GPU) を更新
/// - 各 `AttackClipSharder` のインデックスバッファを更新
pub fn update_triangle(
    shapes: Res<BoxSynthesis>,
    mut box_triangle: ResMut<BoxTriangle>,
    triangle_buffer: Res<AttackClipBufferHandle>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<AttackClipSharder>>,
    meshes: Res<Assets<Mesh>>,
    clip_entitys: Query<(
        &Mesh2d,
        &MeshMaterial2d<AttackClipSharder>,
        &GlobalTransform,
    )>,
) {
    // 形状がなければ全クリア
    if shapes.is_empty() {
        box_triangle.0.clear();

        if let Some(buf) = buffers.get_mut(&triangle_buffer.0) {
            buf.set_data(Vec::<[[f32; 2]; 3]>::new());
        }

        for (_, material_handle, _) in clip_entitys.iter() {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                if let Some(idx_buf) = buffers.get_mut(material.get_indices_handle()) {
                    idx_buf.set_data(Vec::<u32>::new());
                }
                material.set_len(0);
            }
        }
        return;
    }

    // 三角形化
    let triangulation = shapes.triangulate().to_triangulation::<usize>();
    if triangulation.indices.is_empty() {
        box_triangle.0.clear();

        if let Some(buf) = buffers.get_mut(&triangle_buffer.0) {
            buf.set_data(Vec::<[[f32; 2]; 3]>::new());
        }

        for (_, material_handle, _) in clip_entitys.iter() {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                if let Some(idx_buf) = buffers.get_mut(material.get_indices_handle()) {
                    idx_buf.set_data(Vec::<u32>::new());
                }
                material.set_len(0);
            }
        }
        return;
    }

    // 三角形配列化（CPU）
    let triangle_count = triangulation.indices.len() / 3;
    let mut triangles: Vec<[[f32; 2]; 3]> = Vec::with_capacity(triangle_count);

    for i in 0..triangle_count {
        let a = triangulation.points[triangulation.indices[i * 3]];
        let b = triangulation.points[triangulation.indices[i * 3 + 1]];
        let c = triangulation.points[triangulation.indices[i * 3 + 2]];
        triangles.push([[a[0], a[1]], [b[0], b[1]], [c[0], c[1]]]);
    }

    // CPU キャッシュ更新（描画側が参照）
    box_triangle.0 = triangles.clone();

    // GPU 三角形バッファ更新（クリップ用）
    if let Some(buf) = buffers.get_mut(&triangle_buffer.0) {
        buf.set_data(triangles.clone());
    }

    // クリップ対象がなければここまで
    if clip_entitys.is_empty() {
        return;
    }

    // 空間分割で候補三角形を高速抽出
    let global_bounds = calculate_global_bounds(&triangles);
    let quadtree = OptimizedQuadTree::new(global_bounds, &triangles);

    let mut world_pts: Vec<[f32; 3]> = Vec::new();

    for (mesh_handle, material_handle, transform) in clip_entitys.iter() {
        let Some(mesh) = meshes.get(&mesh_handle.0) else {
            continue;
        };
        let Some(VertexAttributeValues::Float32x3(points)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            continue;
        };

        world_pts.clear();
        world_pts.reserve(points.len());

        // ローカル座標 -> ワールド座標
        for p in points {
            let w = transform.transform_point(Vec3::new(p[0], p[1], 0.0));
            world_pts.push([w.x, w.y, 0.0]);
        }

        let indices_usize = collect_triangle_indices_for_points(&world_pts, &quadtree);
        let indices_u32: Vec<u32> = indices_usize.into_iter().map(|i| i as u32).collect();

        if let Some(material) = materials.get_mut(&material_handle.0) {
            if let Some(idx_buf) = buffers.get_mut(material.get_indices_handle()) {
                idx_buf.set_data(indices_u32.clone());
            }
            material.set_len(indices_u32.len() as u32);
        }
    }
}
