use std::sync::Mutex;

use bevy::asset::Asset;
use bevy::image::Image;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{
    encase, AsBindGroup, AsBindGroupError, BindGroupLayout, BindGroupLayoutEntry,
    BindingResources, BindingType, BufferBindingType, BufferInitDescriptor, BufferUsages,
    OwnedBindingResource, SamplerBindingType, ShaderStages, ShaderType, TextureSampleType,
    TextureViewDimension, UnpreparedBindGroup,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::storage::{GpuShaderBuffer, ShaderBuffer};
use bevy::render::texture::{FallbackImage, GpuImage};
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

const ATTACK_CLIP_SHARDER: &str = "attack_clip.wgsl";
//枠内に収まるようにクリップしてくれるシェーダー

/// グローバルに保持される三角形バッファ（wgpu Buffer）
/// RenderApp の Prepare システムが更新し、AsBindGroup 実装が読み取る（Step 2→4）
pub(crate) static TRIANGLE_GPU_BUFFER: Mutex<Option<bevy::render::render_resource::Buffer>> =
    Mutex::new(None);

#[derive(Asset, TypePath, Clone, Debug)]
pub struct AttackClipSharder {
    pub texture: Handle<Image>,
    pub len: u32,
    pub indexs: Handle<ShaderBuffer>,
}

impl Material2d for AttackClipSharder {
    fn fragment_shader() -> ShaderRef {
        ATTACK_CLIP_SHARDER.into()
    }
}

impl AttackClipSharder {
    pub fn new(texture: Handle<Image>, buffers: &mut ResMut<Assets<ShaderBuffer>>) -> Self {
        Self {
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

// Step 4: 手動 AsBindGroup 実装 — グローバルの三角形 Buffer を読み取りバインドグループに設定
impl AsBindGroup for AttackClipSharder {
    type Data = ();

    type Param = (
        bevy::ecs::system::lifetimeless::SRes<RenderAssets<GpuImage>>,
        bevy::ecs::system::lifetimeless::SRes<FallbackImage>,
        bevy::ecs::system::lifetimeless::SRes<RenderAssets<GpuShaderBuffer>>,
    );

    fn label() -> &'static str {
        "AttackClipSharder"
    }

    fn bind_group_layout_entries(
        _render_device: &RenderDevice,
        _force_no_bindless: bool,
    ) -> Vec<BindGroupLayoutEntry> {
        vec![
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    multisampled: false,
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 3,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: Some(<u32 as ShaderType>::min_size()),
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 4,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ]
    }

    fn unprepared_bind_group(
        &self,
        _layout: &BindGroupLayout,
        render_device: &RenderDevice,
        (images, fallback_image, storage_buffers): &mut bevy::ecs::system::SystemParamItem<
            '_,
            '_,
            Self::Param,
        >,
        _force_no_bindless: bool,
    ) -> Result<UnpreparedBindGroup, AsBindGroupError> {
        // binding 0: 三角形バッファ（RenderApp の Prepare が更新したグローバル Buffer）
        let triangle_buffer = {
            let guard = TRIANGLE_GPU_BUFFER
                .lock()
                .map_err(|_| AsBindGroupError::RetryNextUpdate)?;
            guard
                .as_ref()
                .ok_or(AsBindGroupError::RetryNextUpdate)?
                .clone()
        };

        // binding 1: texture + binding 2: sampler
        let (texture_view, sampler) = {
            let handle: Option<&Handle<Image>> = (&self.texture).into();
            if let Some(handle) = handle {
                let image = images
                    .get(handle)
                    .ok_or(AsBindGroupError::RetryNextUpdate)?;
                (image.texture_view.clone(), image.sampler.clone())
            } else {
                (
                    fallback_image.d2.texture_view.clone(),
                    fallback_image.d2.sampler.clone(),
                )
            }
        };

        // binding 3: インデックス長（uniform）
        let len_buffer = {
            let mut wrapper = encase::UniformBuffer::new(Vec::new());
            wrapper.write(&self.len).unwrap();
            render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: None,
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                contents: wrapper.as_ref(),
            })
        };

        // binding 4: インデックスバッファ（Assets<ShaderBuffer> 経由）
        let index_buffer = {
            let handle: &Handle<ShaderBuffer> = &self.indexs;
            storage_buffers
                .get(handle)
                .ok_or(AsBindGroupError::RetryNextUpdate)?
                .buffer
                .clone()
        };

        Ok(UnpreparedBindGroup {
            bindings: BindingResources(vec![
                (0, OwnedBindingResource::Buffer(triangle_buffer)),
                (
                    1,
                    OwnedBindingResource::TextureView(TextureViewDimension::D2, texture_view),
                ),
                (2, OwnedBindingResource::Sampler(SamplerBindingType::Filtering, sampler)),
                (3, OwnedBindingResource::Buffer(len_buffer)),
                (4, OwnedBindingResource::Buffer(index_buffer)),
            ]),
        })
    }

    fn bind_group_data(&self) -> Self::Data {}
}
