use bevy::prelude::*;
use bevy_color::palettes::css::{BLUE, RED};

#[derive(Debug, Clone, PartialEq, Component, Default)]
pub enum SoulMode {
    #[default]
    Red,
    Blue,
}

impl SoulMode {
    pub fn return_color(&self) -> Srgba {
        match self {
            Self::Red => RED,
            Self::Blue => BLUE,
        }
    }
}
