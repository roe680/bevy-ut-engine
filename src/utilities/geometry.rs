/// 点が図形の内側にあるか（レイキャスティング法）
pub fn is_point_inside(point: [f32; 2], shapes: &[Vec<Vec<[f32; 2]>>]) -> bool {
    let [x, y] = point;
    let mut count = 0;

    for shape in shapes {
        for lines in shape {
            let n_points = lines.len();

            if n_points < 3 {
                continue;
            }

            // 最後の点 → 最初の点
            let mut j = n_points - 1;

            for i in 0..n_points {
                let a = lines[i];
                let b = lines[j];

                // 水平な光線 y と線分が交差するか
                if (a[1] > y) != (b[1] > y) {
                    let intersection_x = (b[0] - a[0]) * (y - a[1]) / (b[1] - a[1]) + a[0];

                    if x < intersection_x {
                        count += 1;
                    }
                }

                j = i;
            }
        }
    }

    // 奇数なら内側、偶数なら外側
    count % 2 != 0
}

/// 点から線分上の最近点を求める
fn closest_point_on_segment(point: [f32; 2], a: [f32; 2], b: [f32; 2]) -> [f32; 2] {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];

    let length_squared = dx * dx + dy * dy;

    // a == b
    if length_squared == 0.0 {
        return a;
    }

    // point を直線 AB に射影した位置
    let t = ((point[0] - a[0]) * dx + (point[1] - a[1]) * dy) / length_squared;

    // 線分 AB の外側には出さない
    let t = t.clamp(0.0, 1.0);

    [a[0] + dx * t, a[1] + dy * t]
}

/// 全ての線分から最も近い「線分上の点」を返す
pub fn closest_point_on_shapes(point: [f32; 2], shapes: &[Vec<Vec<[f32; 2]>>]) -> Option<[f32; 2]> {
    let mut closest_point = None;
    let mut closest_distance_squared = f32::INFINITY;

    for shape in shapes {
        for lines in shape {
            let n_points = lines.len();

            if n_points < 2 {
                continue;
            }

            // 閉じたポリゴンなので最後 → 最初も調べる
            for i in 0..n_points {
                let j = (i + 1) % n_points;

                let candidate = closest_point_on_segment(point, lines[i], lines[j]);

                let dx = point[0] - candidate[0];
                let dy = point[1] - candidate[1];

                // sqrt は不要。距離の二乗だけ比較する
                let distance_squared = dx * dx + dy * dy;

                if distance_squared < closest_distance_squared {
                    closest_distance_squared = distance_squared;
                    closest_point = Some(candidate);
                }
            }
        }
    }

    closest_point
}