use crate::*;
use storm::*;
use rendering::*;

pub struct RenderState {
    pub floor_buffer: Buffer<TexturedVertex>,    
    pub floor: Vec<TexturedVertex>,
    pub texture_shader: TextureShader,
    pub floor_texture: Texture,
    pub particle_buffer: Buffer<ModelVertex>,
    pub cube: Vec<ModelVertex>,
    pub model_shader: ModelShader,
}


impl RenderState {
    pub fn new(ctx: &mut Context<AgmaClientApp>) -> RenderState {
        RenderState {
            model_shader: ModelShader::new(ctx),
            cube: create_cube().to_vec(),
            particle_buffer: Buffer::new(ctx),
            floor_buffer: Buffer::new(ctx),    
            floor: create_plane().to_vec(),
            texture_shader: TextureShader::new(ctx),
            floor_texture: Texture::from_png(ctx, TEXTURE_A, TextureFiltering::none())
        }
    }
}