use bevy::prelude::*;
use bevy_vector_shapes::{prelude::ShapePainter, shapes::DiscPainter};
use i_overlay::mesh::{
    outline::offset::OutlineOffset,
    style::{LineJoin, OutlineStyle},
};

use crate::{
    box_border::make_synthsis::BoxSynthesis,
    soul::soul_mode::SoulMode,
    utilities::{
        geometry::{closest_point_on_shapes, is_point_inside},
        render_layers::SOUL_LAYER,
    },
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
    painter.reset();
    painter.render_layers = Some(SOUL_LAYER);
    painter.set_color(soul.1.return_color());
    painter.translate(soul.0.translation);
    painter.circle(5.);
}
pub fn soul_setup(mut cmds: Commands) {
    cmds.spawn(Soul);
}
pub fn make_in_box(
    shapes: Res<BoxSynthesis>,
    mut soul: Single<(&mut Transform, &SoulMode), With<Soul>>,
) {
    let style = OutlineStyle::new(-6.0).line_join(LineJoin::Round(0.1));

    let offset_shapes = shapes.outline(&style);

    let point = [soul.0.translation.x, soul.0.translation.y];

    // 内外判定
    let is_inside = is_point_inside(point, &offset_shapes);

    if !is_inside {
        // 最も近い線分上の点
        let closest_point = closest_point_on_shapes(point, &offset_shapes);

        if let Some(closest) = closest_point {
            soul.0.translation.x = closest[0];
            soul.0.translation.y = closest[1];
        }
        // 外側
    }
    // 内側
}
