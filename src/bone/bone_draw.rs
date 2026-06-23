use bevy::prelude::*;
use bevy::{ecs::system::Query, transform::components::Transform};
use bevy_vector_shapes::prelude::ShapePainter;
use bevy_vector_shapes::shapes::LinePainter;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::float::clip::FloatClip;
use i_overlay::string::clip::ClipRule;

use crate::color_scheme::attack_color_scheme::AttackColorScheme;
use crate::helpers::attack::attack_type::AttackType;
use crate::{
    bone::bone::{Bone, BoneLength},
    box_border::make_synthsis::BoxSynthesis,
    helpers::render_layers::INBOX_ATTACK_LAYER,
};

pub fn bone_draw(
    shapes: Res<BoxSynthesis>,
    attack_color_scheme: Res<AttackColorScheme>,
    bones: Query<(&AttackType, &Transform, &BoneLength), With<Bone>>,
    mut painter: ShapePainter,
) {
    if shapes.is_empty() {
        return;
    }

    let clip_rule = ClipRule {
        invert: false,
        boundary_included: false,
    };

    for (attack_type, transform, bone) in bones.iter() {
        let center = transform.translation.truncate();
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let dir = Vec2::new(angle.cos(), angle.sin());
        let half = bone.length() * 0.5;
        let a = center - dir * half;
        let b = center + dir * half;

        let line = vec![[a.x, a.y], [b.x, b.y]];
        let clipped_paths = line.clip_by(&**shapes, FillRule::EvenOdd, clip_rule);

        painter.reset();
        painter.render_layers = Some(INBOX_ATTACK_LAYER);
        painter.set_color(attack_color_scheme.return_match_color(attack_type));
        painter.thickness = 2.0;

        for path in clipped_paths {
            if path.len() < 2 {
                continue;
            }
            for segment in path.windows(2) {
                let p0 = Vec2::new(segment[0][0], segment[0][1]);
                let p1 = Vec2::new(segment[1][0], segment[1][1]);
                painter.line(p0.extend(0.0), p1.extend(0.0));
            }
        }
    }
}
