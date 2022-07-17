use crate::*;
use rendering::*;

pub struct RenderState {
    pub floor_buffer: Buffer<TexturedVertex>,    
    pub floor: Vec<TexturedVertex>,
    pub texture_shader: TextureShader,
    pub floor_texture: Texture,
    pub particle_buffer: Buffer<SkinnedVertex>,
    pub skinned_shader_pass: SkinnedShaderPass,
    pub skinned_animation_library: AnimationLibrary,
    pub model_shader: SkinnedShader,
}

impl RenderState {
    pub fn new(ctx: &mut Context<AgmaClientApp>) -> RenderState {
        let mut skinned_animation_library = AnimationLibrary::new();
        
        skinned_animation_library.load_animation(String::from("Idle"), String::from("./src/resources/animations/layered_animation.glb"));
        skinned_animation_library.load_animation(String::from("Attack"), String::from("./src/resources/animations/Attack.glb"));
        skinned_animation_library.load_animation(String::from("Running"), String::from("./src/resources/animations/Running.glb"));

        RenderState {
            skinned_shader_pass: SkinnedShaderPass::new(Matrix4::zero(), ctx),
            model_shader: SkinnedShader::new(ctx),
            skinned_animation_library,
            particle_buffer: Buffer::new(ctx),
            floor_buffer: Buffer::new(ctx),    
            floor: create_plane().to_vec(),
            texture_shader: TextureShader::new(ctx),
            floor_texture: Texture::from_png(ctx, TEXTURE_A, TextureFiltering::none())
        }
    }
}