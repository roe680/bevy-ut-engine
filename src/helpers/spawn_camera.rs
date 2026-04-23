use bevy::camera::ScalingMode;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::render::view::Hdr;
pub fn spawn_camera(
    order: isize,
    layers: RenderLayers,
) -> (
    Camera2d,
    Camera,
    Hdr,
    RenderLayers,
    Transform,
    Projection,
    Msaa,
) {
    (
        Camera2d,
        Camera {
            order: order,
            ..default()
        },
        Hdr,
        layers,
        Transform::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 480.0,
            },

            ..OrthographicProjection::default_2d()
        }),
        Msaa::Sample2,
    )
}
