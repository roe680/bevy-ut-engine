use bevy::{
    app::{Plugin, Plugins, Update},
    state::app::AppExtStates,
};

use crate::move_animation::{
    angle::animation_angle, angle_at::animation_angle_at, size::animation_size, to::animation_to,
};

pub struct UTAnimationPlugin;

impl Plugin for UTAnimationPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, animation_angle)
            .add_systems(Update, animation_angle_at)
            .add_systems(Update, animation_to)
            .add_systems(Update, animation_size);
    }
}
