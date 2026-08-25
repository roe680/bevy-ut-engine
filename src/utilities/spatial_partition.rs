/// 空間分割（クアッドツリー）関連のヘルパー群。
/// `box_border` 側から利用できるように、描画ロジックとは分離して定義する。
///
/// 主な用途:
/// - 三角形群の AABB を事前計算してキャッシュ
/// - AABB クエリで候補三角形インデックスを高速取得
/// - クリップ対象メッシュ頂点群の AABB を計算して問い合わせ
///
/// 返すインデックスは「三角形配列の添字」。
#[derive(Debug)]
pub struct OptimizedQuadTree {
    root: QuadTreeNode,
    triangle_bboxes: Vec<(f32, f32, f32, f32)>,
}

impl OptimizedQuadTree {
    /// `bounds`: 全体領域 (min_x, max_x, min_y, max_y)
    /// `triangles`: [[[x, y]; 3]; N]
    pub fn new(bounds: (f32, f32, f32, f32), triangles: &[[[f32; 2]; 3]]) -> Self {
        let triangle_bboxes: Vec<(f32, f32, f32, f32)> =
            triangles.iter().map(get_triangle_bbox).collect();

        // 既存実装の挙動に合わせたパラメータ
        let mut root = QuadTreeNode::new(bounds, 8, 5, 0);

        for idx in 0..triangle_bboxes.len() {
            root.insert_optimized(idx, &triangle_bboxes);
        }

        Self {
            root,
            triangle_bboxes,
        }
    }

    /// クエリ AABB と重なる三角形インデックスを返す。
    pub fn query_aabb_indices(&self, query: (f32, f32, f32, f32)) -> Vec<usize> {
        let mut candidates = Vec::new();
        self.root
            .query_aabb_optimized(query, &mut candidates, &self.triangle_bboxes);

        candidates.sort_unstable();
        candidates.dedup();
        candidates
    }

    /// 互換用: `u32` で欲しい場合
    pub fn query_aabb_indices_u32(&self, query: (f32, f32, f32, f32)) -> Vec<u32> {
        self.query_aabb_indices(query)
            .into_iter()
            .map(|i| i as u32)
            .collect()
    }
}

/// 三角形群から全体 AABB を算出。
pub fn calculate_global_bounds(triangles: &[[[f32; 2]; 3]]) -> (f32, f32, f32, f32) {
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

    // 退化対策
    const EPS: f32 = 0.001;
    if (max_x - min_x).abs() < f32::EPSILON {
        max_x += EPS;
    }
    if (max_y - min_y).abs() < f32::EPSILON {
        max_y += EPS;
    }

    (min_x, max_x, min_y, max_y)
}

/// ワールド座標点群の AABB でクエリし、候補三角形インデックスを取得。
pub fn collect_triangle_indices_for_points(
    points_world: &[[f32; 3]],
    quadtree: &OptimizedQuadTree,
) -> Vec<usize> {
    if points_world.is_empty() {
        return Vec::new();
    }

    let (min_x, max_x, min_y, max_y) = compute_points_aabb(points_world);
    quadtree.query_aabb_indices((min_x, max_x, min_y, max_y))
}

/// ワールド座標点群の AABB 計算。
pub fn compute_points_aabb(points_world: &[[f32; 3]]) -> (f32, f32, f32, f32) {
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

    // 退化対策
    const EPS: f32 = 0.001;
    if (max_x - min_x).abs() < f32::EPSILON {
        return (min_x, min_x + EPS, min_y, max_y);
    }
    if (max_y - min_y).abs() < f32::EPSILON {
        return (min_x, max_x, min_y, min_y + EPS);
    }

    (min_x, max_x, min_y, max_y)
}

#[derive(Clone, Debug)]
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

        // overflow はこのノード内で多重交差した要素
        for &idx in &self.overflow {
            if bboxes_intersect(triangle_bboxes[idx], query) {
                out.push(idx);
            }
        }

        if self.children.is_none() {
            for &idx in &self.triangles {
                if bboxes_intersect(triangle_bboxes[idx], query) {
                    out.push(idx);
                }
            }
        } else if let Some(children) = &self.children {
            for child in children {
                child.query_aabb_optimized(query, out, triangle_bboxes);
            }
        }
    }
}

/// 三角形の AABB
pub fn get_triangle_bbox(triangle: &[[f32; 2]; 3]) -> (f32, f32, f32, f32) {
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

/// AABB 同士の交差判定
pub fn bboxes_intersect(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> bool {
    a.0 <= b.1 && a.1 >= b.0 && a.2 <= b.3 && a.3 >= b.2
}
