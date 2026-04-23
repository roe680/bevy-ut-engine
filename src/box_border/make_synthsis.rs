use std::ops::{Deref, DerefMut};

use bevy::{
    camera::visibility::Visibility,
    ecs::{
        query::Without,
        resource::Resource,
        system::{Query, ResMut},
    },
    math::Vec3,
    transform::components::GlobalTransform,
};
use i_overlay::{
    core::{fill_rule::FillRule, overlay_rule::OverlayRule},
    float::single::SingleFloatOverlay,
};

use crate::box_border::box_struct::{BoxType, BoxZIndex, UTBox};

#[derive(Debug, Clone, PartialEq, Resource, Default)]
pub struct BoxSynthesis(Vec<Vec<Vec<[f32; 2]>>>);

impl Deref for BoxSynthesis {
    type Target = Vec<Vec<Vec<[f32; 2]>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxSynthesis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn make_synthsis(
    a: Query<(&UTBox, &GlobalTransform, &BoxType, &BoxZIndex, &Visibility)>,
    b: Query<(&UTBox, &GlobalTransform, &BoxType, &BoxZIndex), Without<Visibility>>,
    mut resource: ResMut<BoxSynthesis>,
) {
    let mut ut_boxes: Vec<(&UTBox, &GlobalTransform, &BoxType, &BoxZIndex)> = b.iter().collect();
    ut_boxes.extend(
        a.iter()
            .filter(|(_, _, _, _, visibility)| matches!(visibility, Visibility::Visible))
            .map(|(utbox, trans, boxtype, zindex, _)| (utbox, trans, boxtype, zindex)),
    );

    // 早期リターン：ボックスがない場合
    if ut_boxes.is_empty() {
        resource.0.clear();
        return;
    }

    ut_boxes.sort_by_key(|(_, _, _, z)| z.0);

    // CSG 合成
    let mut shapes: Vec<Vec<Vec<[f32; 2]>>> = vec![];
    for (ut_box, transform, box_type, _) in ut_boxes {
        let source = ut_box
            .0
            .iter()
            .map(|[x, y]| {
                let vx = Vec3::new(*x, *y, 0.0); //つまりここで、global transform を適用している。これにより、ローカル座標からワールド座標への変換が行われる。
                let vtx = transform.transform_point(vx);
                [vtx.x, vtx.y]
            })
            .collect::<Vec<[f32; 2]>>();
        shapes = match box_type {
            BoxType::Union => shapes.overlay(&source, OverlayRule::Union, FillRule::EvenOdd),
            BoxType::Difference => {
                shapes.overlay(&source, OverlayRule::Difference, FillRule::EvenOdd)
            }
        };
    }
    resource.0 = shapes;
}
