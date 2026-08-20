use bevy::prelude::*;
use bevy_vector_shapes::{prelude::ShapePainter, shapes::DiscPainter};
use i_overlay::mesh::{
    outline::offset::OutlineOffset,
    style::{LineJoin, OutlineStyle},
};

use crate::{
    box_border::make_synthsis::BoxSynthesis, helpers::render_layers::SOUL_LAYER,
    soul::soul_mode::SoulMode,
};

#[derive(Debug, Clone, PartialEq, Component, Default)]
#[require(Transform, SoulMode)]
pub struct Soul;

pub fn move_soul(
    mut soul: Single<(&mut Transform, &SoulMode), With<Soul>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    match soul.1 {
        SoulMode::Red => {
            if input.pressed(KeyCode::ArrowUp) {
                soul.0.translation += Vec3::new(0., 2.5, 0.)
            }
            if input.pressed(KeyCode::ArrowDown) {
                soul.0.translation += Vec3::new(0., -2.5, 0.)
            }
            if input.pressed(KeyCode::ArrowRight) {
                soul.0.translation += Vec3::new(2.5, 0., 0.)
            }
            if input.pressed(KeyCode::ArrowLeft) {
                soul.0.translation += Vec3::new(-2.5, 0., 0.)
            }
        }
        _ => {}
    }
}

pub fn soul_draw(soul: Single<(&mut Transform, &SoulMode), With<Soul>>, mut painter: ShapePainter) {
    painter.reset;
    painter.render_layers = Some(SOUL_LAYER);
    painter.set_color(soul.1.return_color());
    painter.translate(soul.0.translation);
    painter.circle(5.);
}
pub fn soul_setup(mut cmds: Commands) {
    cmds.spawn(Soul);
}
fn is_point_inside(point: [f32; 2], shapes: &[Vec<Vec<[f32; 2]>>]) -> bool {
    let [x, y] = point;
    let mut count = 0;

    for shape in shapes {
        for lines in shape {
            let n_points = lines.len();

            if n_points < 3 {
                continue;
            }

            let mut j = n_points - 1;

            for i in 0..n_points {
                let a = lines[i];
                let b = lines[j];

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

    count % 2 != 0
}

fn point_segment_distance_squared(point: [f32; 2], a: [f32; 2], b: [f32; 2]) -> f32 {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];

    let len_squared = dx * dx + dy * dy;

    if len_squared == 0.0 {
        let dx = point[0] - a[0];
        let dy = point[1] - a[1];

        return dx * dx + dy * dy;
    }

    let t = ((point[0] - a[0]) * dx + (point[1] - a[1]) * dy) / len_squared;

    let t = t.clamp(0.0, 1.0);

    let closest_x = a[0] + t * dx;
    let closest_y = a[1] + t * dy;

    let dx = point[0] - closest_x;
    let dy = point[1] - closest_y;

    dx * dx + dy * dy
}

///
/// 戻り値:
///     Some((shape_index, lines_index, edge_index))
///
/// edge_index は
///     lines[edge_index]
///         ->
///     lines[(edge_index + 1) % lines.len()]
/// の辺。
///
fn closest_edge(point: [f32; 2], shapes: &[Vec<Vec<[f32; 2]>>]) -> Option<(usize, usize, usize)> {
    let mut closest = None;
    let mut closest_distance = f32::INFINITY;

    for (shape_index, shape) in shapes.iter().enumerate() {
        for (lines_index, lines) in shape.iter().enumerate() {
            let n_points = lines.len();

            if n_points < 2 {
                continue;
            }

            for edge_index in 0..n_points {
                let next = (edge_index + 1) % n_points;

                let distance =
                    point_segment_distance_squared(point, lines[edge_index], lines[next]);

                if distance < closest_distance {
                    closest_distance = distance;
                    closest = Some((shape_index, lines_index, edge_index));
                }
            }
        }
    }

    closest
}

pub fn make_in_box(
    shapes: Res<BoxSynthesis>,
    mut soul: Single<(&mut Transform, &SoulMode), With<Soul>>,
) {
    let style = OutlineStyle::new(-5.0).line_join(LineJoin::Round(0.1));

    let offset_shapes = shapes.outline(&style);

    let point = [soul.0.translation.x, soul.0.translation.y];

    let is_inside = is_point_inside(point, &offset_shapes);

    if !is_inside {
        let closest = closest_edge(point, &offset_shapes);

        if let Some((shape_index, lines_index, edge_index)) = closest {
            let lines = &offset_shapes[shape_index][lines_index];

            let a = lines[edge_index];
            let b = lines[(edge_index + 1) % lines.len()];
        }
        // 外側
    }
}
