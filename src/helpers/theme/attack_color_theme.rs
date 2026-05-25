use bevy::{
    color::palettes::css::{LIGHT_BLUE, ORANGE, WHITE},
    prelude::*,
};

use crate::helpers::attack::attack_type::AttackType;
#[derive(Resource)]
pub struct AttackColorTheme {
    normal: Color,
    must_move: Color,
    must_not_move: Color,
}

impl Default for AttackColorTheme {
    fn default() -> Self {
        Self {
            normal: WHITE.into(),
            must_move: ORANGE.into(),
            must_not_move: LIGHT_BLUE.into(),
        }
    }
}

impl AttackColorTheme {
    pub fn return_match_color(&self, attack_type: &AttackType) -> Color {
        match attack_type {
            AttackType::Normal => self.normal,
            AttackType::MustMove => self.must_move,
            AttackType::MustNotMove => self.must_not_move,
        }
    }
}
