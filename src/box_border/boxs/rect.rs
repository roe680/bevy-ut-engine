use crate::{
    box_border::{
        box_moving::{_create_default, BoxMoving, BoxMovingType},
        box_struct::UTBox,
    },
    helpers::easing::Easing,
};

impl UTBox {
    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self(vec![[x, y], [x + w, y], [x + w, y + h], [x, y + h]])
    }
}

pub struct RectBox;

impl RectBox {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> UTBox {
        UTBox(vec![[x, y], [x + w, y], [x + w, y + h], [x, y + h]])
    }

    pub fn default() -> UTBox {
        UTBox::default()
    }
}

//非推奨
impl BoxMoving<RectBox> {
    pub fn move_shape(mut self, x: f32, y: f32, w: f32, h: f32, ease: Easing) -> Self {
        self.0.push(_create_default(BoxMovingType::MoveShape(
            ease,
            None,
            vec![[x, y], [x + w, y], [x + w, y + h], [x, y + h]],
            vec![],
        )));
        self
    }

    pub fn add_shape(mut self, x: f32, y: f32, w: f32, h: f32, ease: Easing) -> Self {
        self.0.push(_create_default(BoxMovingType::AddShape(
            ease,
            vec![[x, y], [x + w, y], [x + w, y + h], [x, y + h]],
            vec![],
        )));
        self
    }
}
