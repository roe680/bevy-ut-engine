use bevy::core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin};
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::ShaderType;
use bevy::shader::ShaderRef;
use bevy::prelude::*;
use bevy::core_pipeline::Core2d;

pub struct FullscreenEffectPlugin;

impl Plugin for FullscreenEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FullscreenMaterialPlugin::<FullscreenEffect>::default())
            .add_systems(Update, (update_intensity, toggle_effect));
    }
}

fn update_intensity(
    mut effects: Query<&mut FullscreenEffect>,
    time: Res<Time>,
    mut last_intensity: Local<f32>,
    mut phase_offset: Local<f32>,
) {
    let t = time.elapsed_secs();
    let freq = FullscreenEffect::FREQUENCY;
    let max = FullscreenEffect::MAX_INTENSITY;

    for mut effect in &mut effects {
        // Check if the intensity was modified externally since last frame.
        // This ensures that when intensity is modified, this system recalculates
        // the phase offset to avoid intensity jumps.
        if effect.intensity != *last_intensity {
            // Map the target intensity back to sine range [-1, 1]
            let target_sine = (effect.intensity / max) * 2.0 - 1.0;
            // Compute a phase offset so that `ops::sin(t * freq + offset) == target_sine`
            *phase_offset = ops::asin(target_sine) - t * freq;
        }

        // Compute the new intensity from the (possibly adjusted) phase offset
        let phase = t * freq + *phase_offset;
        // Make it loop periodically
        let mut intensity = ops::sin(phase);

        // We need to remap the intensity to be between 0 and 1 instead of -1 and 1
        intensity = (intensity + 1.0) / 2.0;
        *last_intensity = intensity * max;

        effect.intensity = *last_intensity;
    }
}

fn toggle_effect(
    keys: Res<ButtonInput<KeyCode>>,
    camera: Single<(Entity, Option<&FullscreenEffect>), With<Camera2d>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        let (entity, effect) = *camera;

        if effect.is_some() {
            commands.entity(entity).remove::<FullscreenEffect>();
        } else {
            commands.entity(entity).insert(FullscreenEffect::new(0.0));
        }
    }
}

#[derive(Component, ExtractComponent, Clone, Copy, ShaderType, Default)]
pub struct FullscreenEffect {
    pub intensity: f32,
    // WebGL2 structs must be 16 byte aligned.
    // Intensity is an `f32`, which is 4 bytes, so 12 more bytes (3 floats) are needed.
    #[cfg(feature = "webgl2")]
    _webgl2_padding: Vec3,
}

impl FullscreenEffect {
    const FREQUENCY: f32 = 2.0;
    const MAX_INTENSITY: f32 = 0.015;

    pub fn new(intensity: f32) -> Self {
        Self {
            intensity,
            ..Default::default()
        }
    }
}

impl FullscreenMaterial for FullscreenEffect {
    fn fragment_shader() -> ShaderRef {
        "shader/rgb.wgsl".into()
    }

    // The `FullscreenMaterial` uses 3d schedules by default.
    // To make this work in 2d, you would need to schedule to
    // run in `Core2d` and in a `Core2dSystems` set.
    //
    fn schedule() -> impl bevy::ecs::schedule::ScheduleLabel + Clone {
        Core2d
    }
    fn schedule_configs(
        system: bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::BoxedSystem>,
    ) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::BoxedSystem> {
        system
            .in_set(bevy::core_pipeline::Core2dSystems::PostProcess)
            .before(bevy::core_pipeline::tonemapping::tonemapping)
    }
}
