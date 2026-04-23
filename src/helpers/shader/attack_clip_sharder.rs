use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer};
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

const ATTACK_CLIP_SHARDER: &str = "attack_clip.wgsl";
//枠内に収まるようにクリップしてくれるシェーダー
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct AttackClipSharder {
    #[storage(0, read_only)]
    triangles: Handle<ShaderStorageBuffer>,
    #[texture(1)]
    #[sampler(2)]
    texture: Handle<Image>,
    #[uniform(3)]
    len: u32,
    #[storage(4, read_only)]
    indexs: Handle<ShaderStorageBuffer>,
}
impl Material2d for AttackClipSharder {
    fn fragment_shader() -> ShaderRef {
        ATTACK_CLIP_SHARDER.into()
    }
}

impl AttackClipSharder {
    pub fn new(
        texture: Handle<Image>,
        buffers: &mut ResMut<Assets<ShaderStorageBuffer>>,
        triangles_buffer: Handle<ShaderStorageBuffer>,
    ) -> Self {
        Self {
            triangles: triangles_buffer,
            texture,
            // 初期状態では個別インデックス未設定なので 0
            len: 0,
            //このシェーダーをつけた、meshをクリッピングする際に、クリッピングに必要な三角形をGPUに送るためのやつ。全ての三角形を対象にすると計算量が膨大でGPUが悲鳴を上げるので。
            indexs: buffers.add(Vec::<u32>::new()), //ちなみに使う三角形の情報でなく、使う三角形のインデックスを送っている。
        }
    }
    pub fn set_len(&mut self, len: u32) {
        self.len = len;
    }

    pub fn get_len(&self) -> u32 {
        self.len
    }

    pub fn get_indices_handle(&self) -> &Handle<ShaderStorageBuffer> {
        &self.indexs
    }
}
//ハンドルだと思われる。ShaderStorageBufferにGPUに渡すすべてのデータを格納するので、目的のデータが欲しい場合Handleを通してアクセスする必要がある。
#[derive(Resource)]
pub struct AttackClipBufferHandle(pub Handle<ShaderStorageBuffer>);

pub fn setup_attack_clip_buffer_buffer(
    mut cmds: Commands,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let triangles: Vec<[[f32; 2]; 3]> = vec![];
    let buffer_handle = buffers.add(triangles); //ここで、GPUとCPUでデータをやりとりするためのバッファを作成している。trianglesは初期値で空のベクタ。
    cmds.insert_resource(AttackClipBufferHandle(buffer_handle));
}
