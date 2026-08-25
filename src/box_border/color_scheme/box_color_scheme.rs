use bevy::{
    color::palettes::css::{BLACK, LIGHT_BLUE, ORANGE, WHITE},
    prelude::*,
};

use crate::utilities::attack::attack_type::AttackType;
#[derive(Resource)]
pub struct BoxColorScheme {
    pub line_color: Color,
    pub fill_color: Color,
}

impl Default for BoxColorScheme {
    fn default() -> Self {
        Self {
            line_color: WHITE.into(),
            fill_color: BLACK.into(),
        }
    }
}
