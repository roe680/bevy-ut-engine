use bevy::ecs::component::Component;

#[derive(Debug, Clone, PartialEq, Component, Default)]
pub enum BoneType {
    #[default]
    Sans,
    Papyrus,
}
