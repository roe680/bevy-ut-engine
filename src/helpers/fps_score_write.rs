use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;

use crate::helpers::render_layers::FPS_LAYER;

pub fn update_fps(time: Res<Time>, mut texts: Query<&mut Text2d, With<FpsComponent>>) {
    for mut text in texts.iter_mut() {
        text.0 = format!("fps: {}", 1. / time.delta_secs());
    }
}

#[derive(Debug, Default, Clone, Component)]
pub struct FpsComponent;

pub fn setup_fps(mut cmds: Commands, fonts: ResMut<AssetServer>) {
    let font: Handle<Font> = fonts.load("OpenSans-Regular.ttf");
    cmds.spawn((
        FpsComponent,
        Text2d("Not up date".to_string()),
        Transform {
            translation: Vec3::new(240., 220., 0.),
            ..default()
        },
        TextFont {
            font: font,
            font_size: 20.,
            ..default()
        },
        TextColor(WHITE.into()),
        FPS_LAYER,
    ));
}
