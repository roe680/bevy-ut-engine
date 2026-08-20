use bevy::prelude::*;

use crate::soul::soul::{move_soul, soul_draw, soul_setup};
pub struct SoulPlugin;

impl Plugin for SoulPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, soul_draw)
            .add_systems(Startup, soul_setup)
            .add_systems(Update, move_soul);
    }
}
