use bevy::prelude::*;

use crate::helpers::theme::attack_color_theme::AttackColorTheme;

pub struct UTThemePlugin;

impl Plugin for UTThemePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AttackColorTheme>();
    }
}
