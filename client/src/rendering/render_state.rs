use crate::*;
use storm::*;
use rendering::*;

pub struct RenderState {
    pub floor_buffer: Buffer<TexturedVertex>,    
    pub floor: Vec<TexturedVertex>,
    pub texture_shader: TextureShader,
    pub floor_texture: Texture,
    pub particle_buffer: Buffer<SkinnedVertex>,
    pub skinned_shader_pass: SkinnedShaderPass,
    pub animation: SkinnedAnimation,
    pub model_shader: SkinnedShader,
}


impl RenderState {
    pub fn new(ctx: &mut Context<AgmaClientApp>) -> RenderState {
        RenderState {
            skinned_shader_pass: SkinnedShaderPass::new(Matrix4::zero(), ctx),
            model_shader: SkinnedShader::new(ctx),
            animation: create_skinned_mesh_from_file(),
            particle_buffer: Buffer::new(ctx),
            floor_buffer: Buffer::new(ctx),    
            floor: create_plane().to_vec(),
            texture_shader: TextureShader::new(ctx),
            floor_texture: Texture::from_png(ctx, TEXTURE_A, TextureFiltering::none())
        }
    }
}