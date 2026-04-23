use bevy::{ecs::system::Query, transform::components::Transform};

use crate::bone::bone::Bone;

pub fn bone_draw(bones: Query<(&Bone, &Transform)>) {
    for (bone, transform) in bones.iter() {
        println!("Bone: {:?}, Transform: {:?}", bone, transform);
    }
}
