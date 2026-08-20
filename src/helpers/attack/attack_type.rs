use bevy::ecs::component::Component;

#[derive(Copy, Debug, Clone, PartialEq, Component, Default)]
pub enum AttackType {
    #[default]
    Normal,
    MustMove,
    MustNotMove,
}
