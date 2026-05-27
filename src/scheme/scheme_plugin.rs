use bevy::prelude::*;

use crate::scheme::attack_color_scheme::AttackColorScheme;

pub struct UTSchemePlugin;

impl Plugin for UTSchemePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AttackColorScheme>();
    }
}
