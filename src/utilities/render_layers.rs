use bevy::camera::visibility::RenderLayers;

/// Render layer constants.
///
/// These use the `RenderLayers::layer` constructor (the typical constructor used
/// across Bevy 0.17/0.18). Keep these as simple constants so the rest of the
/// codebase can continue to reference `BOX_LAYER`, `BOX_LINE_LAYER`, etc.
pub const BOX_LAYER: RenderLayers = RenderLayers::layer(1);
pub const BOX_LINE_LAYER: RenderLayers = RenderLayers::layer(2);
pub const INBOX_ATTACK_LAYER: RenderLayers = RenderLayers::layer(3);
pub const FPS_LAYER: RenderLayers = RenderLayers::layer(63);
pub const SOUL_LAYER: RenderLayers = RenderLayers::layer(4);
