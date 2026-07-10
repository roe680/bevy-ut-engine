use bevy::{
    asset::Asset,
    image::Image,
    prelude::*,
    reflect::TypePath,
    render::{
        render_resource::AsBindGroup,
        storage::ShaderBuffer,
    },
    shader::ShaderRef,
    sprite_render::Material2d,
};

const ATTACK_CLIP_SHARDER: &str = "attack_clip.wgsl";

#[derive(Asset, TypePath, Clone, Debug, AsBindGroup)]
pub struct AttackClipSharder {
    #[storage(0, read_only)]
    pub triangles: Handle<ShaderBuffer>,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
    #[uniform(3)]
    pub len: u32,
    #[storage(4, read_only)]
    pub indexs: Handle<ShaderBuffer>,
}

impl Material2d for AttackClipSharder {
    fn fragment_shader() -> ShaderRef {
        ATTACK_CLIP_SHARDER.into()
    }
}

impl AttackClipSharder {
    pub fn new(
        texture: Handle<Image>,
        buffers: &mut ResMut<Assets<ShaderBuffer>>,
        triangles: Handle<ShaderBuffer>,
    ) -> Self {
        Self {
            triangles,
            texture,
            len: 0,
            indexs: buffers.add(Vec::<u32>::new()),
        }
    }
    pub fn set_len(&mut self, len: u32) {
        self.len = len;
    }

    pub fn get_len(&self) -> u32 {
        self.len
    }

    pub fn get_indices_handle(&self) -> &Handle<ShaderBuffer> {
        &self.indexs
    }
}
