use bevy::ecs::component::Component;

#[derive(Debug, Clone, PartialEq, Component)]
pub struct DamageValue(pub f32);
impl Default for DamageValue {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Debug, Clone, PartialEq, Component, Default)]
enum DamageType {
    #[default]
    Normal,
    SansType,
}
