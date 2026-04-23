use bevy::ecs::component::Component;

#[derive(Debug, Clone, PartialEq, Component, Default)]
pub enum AttackType {
    #[default]
    Normal,
    MustMove,
    MustNotMove,
}
