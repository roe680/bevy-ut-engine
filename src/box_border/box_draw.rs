use crate::{
    box_border::{
        box_struct::{BoxType, BoxZIndex, UTBox},
        color_scheme::box_color_scheme::BoxColorScheme,
        make_synthsis::BoxSynthesis,
        update_triangle::BoxTriangle,
    },
    utilities::render_layers::{BOX_LAYER, BOX_LINE_LAYER, INBOX_ATTACK_LAYER},
};
use bevy::{color::palettes::css::BLACK, prelude::*};
use bevy_vector_shapes::prelude::*;

/// `box_draw`:
/// - 三角形データの生成やバッファ更新は `update_triangle` 側で実行
/// - ここでは `BoxTriangle` リソースを参照して描画のみを担当
pub fn box_draw(
    mut painter: ShapePainter,
    triangles: Res<BoxTriangle>,
    shapes: Res<BoxSynthesis>,
    color_scheme: Res<BoxColorScheme>,
) {
    // 三角形塗り描画
    painter.reset();
    painter.render_layers = Some(BOX_LAYER);
    painter.set_color(color_scheme.fill_color);

    for tri in triangles.0.iter() {
        let v_a = Vec2::new(tri[0][0], tri[0][1]);
        let v_b = Vec2::new(tri[1][0], tri[1][1]);
        let v_c = Vec2::new(tri[2][0], tri[2][1]);
        painter.triangle(v_a, v_b, v_c);
    }

    // アウトライン描画（既存仕様維持）
    painter.reset();
    painter.render_layers = Some(BOX_LINE_LAYER);
    painter.set_color(color_scheme.line_color);
    painter.thickness = 2.0;

    for shape_group in &**shapes {
        for shape in shape_group {
            if shape.is_empty() {
                continue;
            }

            for i in 0..shape.len() {
                let a: Vec2 = shape[i].into();
                let b: Vec2 = shape[(i + 1) % shape.len()].into();
                painter.line(a.extend(0.0), b.extend(0.0));
            }
        }
    }
}
