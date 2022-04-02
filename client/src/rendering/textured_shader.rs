use crate::{
    graphics::{
        std140, Buffer, DrawMode, Shader, ShaderDescriptor, Texture, Uniform,
    },
    App, Context,
};

use crate::ModelUniform;
use crate::TexturedVertex;

impl ShaderDescriptor<1> for TextureShader {
    const VERTEX_SHADER: &'static str = include_str!("textured_vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("textured_fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 1] = ["tex"];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    type VertexUniformType = ModelUniform;
    type VertexDescriptor = TexturedVertex;
}

/// Shader object for sprites. This holds no mutable state, so it's recommended to reuse this as
/// much as possible.
pub struct TextureShader {
    shader: Shader<TextureShader, 1>,
}

impl TextureShader {
    /// Creates a new sprite shader.
    pub fn new(ctx: &Context<impl App>) -> TextureShader {
        TextureShader {
            shader: Shader::new(ctx),
        }
    }

    /// Helper function to draw sprites to the screen.
    pub fn draw(&self, uniform: &Uniform<ModelUniform>, atlas: &Texture, buffers: &Buffer<TexturedVertex>) {
        self.shader.draw(DrawMode::Triangles, uniform, [atlas], &[buffers]);
    }
}
