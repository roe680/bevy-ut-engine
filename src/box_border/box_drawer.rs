use crate::{
    box_border::{
        box_struct::{BoxType, BoxZIndex, UTBox},
        make_synthsis::BoxSynthesis,
    },
    helpers::{
        render_layers::{BOX_LAYER, BOX_LINE_LAYER},
        shader::attack_clip_sharder::{AttackClipBufferHandle, AttackClipSharder},
    },
};
use bevy::{color::palettes::css::BLACK, prelude::*, render::storage::ShaderStorageBuffer};
use bevy_mesh::VertexAttributeValues;
use bevy_vector_shapes::prelude::*;

use i_triangle::{float::triangulatable::Triangulatable, *};

/// box_draw:
/// CPU負荷削減版の空間分割による三角形フィルタリング
/// - 早期リターン、メモリプール活用、効率化されたクアッドツリーによる最適化
/// - AABBキャッシュとバッチ処理による高速化
pub fn box_draw(
    mut painter: ShapePainter,
    a: Query<(&UTBox, &GlobalTransform, &BoxType, &BoxZIndex, &Visibility)>,
    b: Query<(&UTBox, &GlobalTransform, &BoxType, &BoxZIndex), Without<Visibility>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<AttackClipSharder>>,
    meshes: Res<Assets<Mesh>>,
    clip_entitys: Query<(
        &Mesh2d,
        &MeshMaterial2d<AttackClipSharder>,
        &GlobalTransform,
    )>,
    shapes: Res<BoxSynthesis>,
    triangle_buffer: Res<AttackClipBufferHandle>,
) {
    // 早期リターン：形状がない場合
    if shapes.is_empty() {
        buffers
            .get_mut(&triangle_buffer.0)
            .unwrap()
            .set_data(Vec::<[[f32; 2]; 3]>::new());
        return;
    }

    // 三角形化 & 描画（塗り）
    let triangulation = shapes.triangulate().to_triangulation::<usize>();

    // 早期リターン：三角形がない場合
    if triangulation.indices.is_empty() {
        buffers
            .get_mut(&triangle_buffer.0)
            .unwrap()
            .set_data(Vec::<[[f32; 2]; 3]>::new());
        return;
    }

    let triangle_count = triangulation.indices.len() / 3;
    let mut triangles: Vec<[[f32; 2]; 3]> = Vec::with_capacity(triangle_count);

    painter.reset();
    painter.render_layers = Some(BOX_LAYER);

    for i in 0..triangle_count {
        let v_a: Vec2 = triangulation.points[triangulation.indices[i * 3]].into();
        let v_b: Vec2 = triangulation.points[triangulation.indices[i * 3 + 1]].into();
        let v_c: Vec2 = triangulation.points[triangulation.indices[i * 3 + 2]].into();
        painter.set_color(BLACK);
        painter.triangle(v_a, v_b, v_c);
        triangles.push([[v_a.x, v_a.y], [v_b.x, v_b.y], [v_c.x, v_c.y]]);
    }

    // アウトライン描画
    painter.reset();
    painter.render_layers = Some(BOX_LINE_LAYER);
    painter.set_color(Color::WHITE);
    painter.thickness = 2.0;
    for shape_group in &**shapes {
        for shape in shape_group {
            for i in 0..shape.len() {
                let a: Vec2 = shape[i].into();
                let b: Vec2 = shape[(i + 1) % shape.len()].into();
                painter.line(a.extend(0.), b.extend(0.));
            }
        }
    }

    // クリップエンティティが存在しない場合は、三角形バッファのみ更新して終了
    if clip_entitys.is_empty() {
        buffers
            .get_mut(&triangle_buffer.0)
            .unwrap()
            .set_data(triangles);
        return;
    }

    // 最適化されたクアッドツリー構築
    let global_bounds = calculate_global_bounds(&triangles);
    let quadtree = OptimizedQuadTree::new(global_bounds, &triangles);

    // 単一ループで clip エンティティ処理
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

        // ローカル → ワールド
        for p in points {
            let w = transform.transform_point(Vec3::new(p[0], p[1], 0.0));
            world_pts.push([w.x, w.y, 0.0]);
        }
        let indices_usize = collect_triangle_indices_for_points(&world_pts, &quadtree);
        if indices_usize.is_empty() {
            // 0 の場合も GPU へ反映（長さ更新）
            if let Some(material) = materials.get_mut(&material_handle.0) {
                if let Some(idx_buf) = buffers.get_mut(material.get_indices_handle()) {
                    idx_buf.set_data(Vec::<u32>::new());
                }
                material.set_len(0);
            }
            continue;
        }

        // GPU バッファ用に u32 へ変換
        let indices_u32: Vec<u32> = indices_usize.iter().map(|&i| i as u32).collect();

        if let Some(material) = materials.get_mut(&material_handle.0) {
            if let Some(idx_buf) = buffers.get_mut(material.get_indices_handle()) {
                idx_buf.set_data(indices_u32.clone());
            }
            material.set_len(indices_u32.len() as u32);
        }
    }

    buffers
        .get_mut(&triangle_buffer.0)
        .unwrap()
        .set_data(triangles);
}

// グローバル境界の効率的計算
fn calculate_global_bounds(triangles: &[[[f32; 2]; 3]]) -> (f32, f32, f32, f32) {
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for tri in triangles {
        for v in tri {
            min_x = min_x.min(v[0]);
            max_x = max_x.max(v[0]);
            min_y = min_y.min(v[1]);
            max_y = max_y.max(v[1]);
        }
    }

    const EPS: f32 = 0.001;
    if (max_x - min_x).abs() < f32::EPSILON {
        max_x += EPS;
    }
    if (max_y - min_y).abs() < f32::EPSILON {
        max_y += EPS;
    }

    (min_x, max_x, min_y, max_y)
}

pub fn collect_triangle_indices_for_points(
    points_world: &Vec<[f32; 3]>,
    quadtree: &OptimizedQuadTree,
) -> Vec<usize> {
    if points_world.is_empty() {
        return Vec::new();
    }
    let (min_x, max_x, min_y, max_y) = compute_points_aabb(points_world);
    quadtree.query_aabb_indices((min_x, max_x, min_y, max_y))
}

fn compute_points_aabb(points_world: &[[f32; 3]]) -> (f32, f32, f32, f32) {
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for p in points_world {
        min_x = min_x.min(p[0]);
        max_x = max_x.max(p[0]);
        min_y = min_y.min(p[1]);
        max_y = max_y.max(p[1]);
    }
    const EPS: f32 = 0.001;
    if (max_x - min_x).abs() < f32::EPSILON {
        return (min_x, min_x + EPS, min_y, max_y);
    }
    if (max_y - min_y).abs() < f32::EPSILON {
        return (min_x, max_x, min_y, min_y + EPS);
    }
    (min_x, max_x, min_y, max_y)
}

// ===================== 最適化されたクアッドツリー =====================

pub struct OptimizedQuadTree {
    root: QuadTreeNode,
    triangle_bboxes: Vec<(f32, f32, f32, f32)>, // 三角形のAABBをキャッシュ
}

impl OptimizedQuadTree {
    pub fn new(bounds: (f32, f32, f32, f32), triangles: &[[[f32; 2]; 3]]) -> Self {
        // 三角形のAABBを事前計算
        let triangle_bboxes: Vec<(f32, f32, f32, f32)> =
            triangles.iter().map(|tri| get_triangle_bbox(tri)).collect();

        let mut root = QuadTreeNode::new(bounds, 8, 5, 0); // パラメータ調整で最適化

        // 三角形をツリーに挿入
        for idx in 0..triangles.len() {
            root.insert_optimized(idx, &triangle_bboxes);
        }

        Self {
            root,
            triangle_bboxes,
        }
    }

    pub fn query_aabb_indices(&self, query: (f32, f32, f32, f32)) -> Vec<usize> {
        let mut candidates = Vec::new();
        self.root
            .query_aabb_optimized(query, &mut candidates, &self.triangle_bboxes);

        // 重複除去を効率化
        candidates.sort_unstable();
        candidates.dedup();
        candidates
    }

    fn query_aabb_optimized(&self, query: (f32, f32, f32, f32)) -> Vec<u32> {
        let mut candidates = Vec::new();
        self.root
            .query_aabb_optimized(query, &mut candidates, &self.triangle_bboxes);

        // 重複除去を効率化
        candidates.sort_unstable();
        candidates.dedup();
        candidates.into_iter().map(|i| i as u32).collect()
    }
}

#[derive(Clone)]
struct QuadTreeNode {
    bounds: (f32, f32, f32, f32),
    triangles: Vec<usize>,
    overflow: Vec<usize>,
    children: Option<[Box<QuadTreeNode>; 4]>,
    max_triangles: usize,
    max_depth: usize,
    current_depth: usize,
}

impl QuadTreeNode {
    fn new(
        bounds: (f32, f32, f32, f32),
        max_triangles: usize,
        max_depth: usize,
        current_depth: usize,
    ) -> Self {
        Self {
            bounds,
            triangles: Vec::new(),
            overflow: Vec::new(),
            children: None,
            max_triangles,
            max_depth,
            current_depth,
        }
    }

    fn insert_optimized(&mut self, triangle_idx: usize, triangle_bboxes: &[(f32, f32, f32, f32)]) {
        let tri_bbox = triangle_bboxes[triangle_idx];
        if !bboxes_intersect(self.bounds, tri_bbox) {
            return;
        }

        if self.children.is_none() {
            self.triangles.push(triangle_idx);
            if self.triangles.len() > self.max_triangles && self.current_depth < self.max_depth {
                self.subdivide();
                if let Some(children) = &mut self.children {
                    let to_redist = std::mem::take(&mut self.triangles);
                    for idx in to_redist {
                        let bbox = triangle_bboxes[idx];
                        let mut fitting_child = None;
                        let mut multiple_fits = false;

                        for (ci, child) in children.iter().enumerate() {
                            if bboxes_intersect(child.bounds, bbox) {
                                if fitting_child.is_none() {
                                    fitting_child = Some(ci);
                                } else {
                                    multiple_fits = true;
                                    break;
                                }
                            }
                        }

                        match (fitting_child, multiple_fits) {
                            (Some(ci), false) => {
                                children[ci].insert_optimized(idx, triangle_bboxes)
                            }
                            _ => self.overflow.push(idx),
                        }
                    }
                }
            }
        } else if let Some(children) = &mut self.children {
            let mut fitting_child = None;
            let mut multiple_fits = false;

            for (ci, child) in children.iter().enumerate() {
                if bboxes_intersect(child.bounds, tri_bbox) {
                    if fitting_child.is_none() {
                        fitting_child = Some(ci);
                    } else {
                        multiple_fits = true;
                        break;
                    }
                }
            }

            match (fitting_child, multiple_fits) {
                (Some(ci), false) => children[ci].insert_optimized(triangle_idx, triangle_bboxes),
                _ => self.overflow.push(triangle_idx),
            }
        }
    }

    fn subdivide(&mut self) {
        let (min_x, max_x, min_y, max_y) = self.bounds;
        let mid_x = (min_x + max_x) * 0.5;
        let mid_y = (min_y + max_y) * 0.5;
        self.children = Some([
            Box::new(QuadTreeNode::new(
                (min_x, mid_x, min_y, mid_y),
                self.max_triangles,
                self.max_depth,
                self.current_depth + 1,
            )),
            Box::new(QuadTreeNode::new(
                (mid_x, max_x, min_y, mid_y),
                self.max_triangles,
                self.max_depth,
                self.current_depth + 1,
            )),
            Box::new(QuadTreeNode::new(
                (min_x, mid_x, mid_y, max_y),
                self.max_triangles,
                self.max_depth,
                self.current_depth + 1,
            )),
            Box::new(QuadTreeNode::new(
                (mid_x, max_x, mid_y, max_y),
                self.max_triangles,
                self.max_depth,
                self.current_depth + 1,
            )),
        ]);
    }

    fn query_aabb_optimized(
        &self,
        query: (f32, f32, f32, f32),
        out: &mut Vec<usize>,
        triangle_bboxes: &[(f32, f32, f32, f32)],
    ) {
        if !bboxes_intersect(self.bounds, query) {
            return;
        }

        // overflow検査
        for &idx in &self.overflow {
            if bboxes_intersect(triangle_bboxes[idx], query) {
                out.push(idx);
            }
        }

        if self.children.is_none() {
            // 葉ノード
            for &idx in &self.triangles {
                if bboxes_intersect(triangle_bboxes[idx], query) {
                    out.push(idx);
                }
            }
        } else if let Some(children) = &self.children {
            // 子ノードを再帰的に探索
            for child in children.iter() {
                child.query_aabb_optimized(query, out, triangle_bboxes);
            }
        }
    }
}

// ===================== ユーティリティ関数 =====================

fn get_triangle_bbox(triangle: &[[f32; 2]; 3]) -> (f32, f32, f32, f32) {
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for v in triangle {
        min_x = min_x.min(v[0]);
        max_x = max_x.max(v[0]);
        min_y = min_y.min(v[1]);
        max_y = max_y.max(v[1]);
    }

    (min_x, max_x, min_y, max_y)
}

fn bboxes_intersect(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> bool {
    a.0 <= b.1 && a.1 >= b.0 && a.2 <= b.3 && a.3 >= b.2
}
