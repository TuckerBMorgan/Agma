#![allow(dead_code)]
use storm::Context;
use storm::graphics::{Buffer, std140,DrawMode, Shader, ShaderDescriptor,Uniform};
use storm::math::PerspectiveCamera;
use crate::rendering::ModelVertex;
use storm::cgmath::Matrix4;
use crate::AgmaClientApp;

impl ShaderDescriptor<0> for ModelShader {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    const TEXTURE_NAMES: [&'static str; 0] = [];
    type VertexUniformType = ModelUniform;
    type VertexDescriptor = ModelVertex;
}

#[std140::uniform]
#[derive(Copy, Clone)]
pub struct ModelUniform {
    pub ortho: std140::mat4,
}

impl ModelUniform {
    pub fn new(ortho: Matrix4<f32>) -> ModelUniform {
        ModelUniform {
            ortho: ortho.into(),
        }
    }
}

impl From<&mut PerspectiveCamera> for ModelUniform {
    fn from(item: &mut PerspectiveCamera) -> Self {
        ModelUniform::new(item.matrix())
    }
}

pub struct ModelShader {
    shader: Shader<ModelShader, 0>,
}

impl ModelShader {
    pub fn new(ctx: &mut Context<AgmaClientApp>) -> ModelShader {
        ModelShader {
            shader: Shader::new(ctx),
        }
    }

    /// Draws to the screen.
    pub fn draw(&self, uniform: &Uniform<ModelUniform>, buffer: &Buffer<ModelVertex>) {
        self.shader.draw(DrawMode::Triangles, uniform, [], &[buffer]);
    }
}

/*
pub struct ModelShaderPass {
    pub uniform: Uniform<ModelUniform>,
    pub buffer: Buffer<ModelVertex>,
}

impl ModelShaderPass {
    pub fn new(ortho: Matrix4<f32>, ctx: &mut Context<AgmaClientApp>) -> ModelShaderPass {
        ModelShaderPass {
            uniform: Uniform::new(ctx, ModelUniform::new(ortho)),
            buffer: Buffer::new(ctx),
        }
    }

    /// Draws the pass to the screen.
    pub fn draw(&mut self, shader: &ModelShader) {
        shader.draw(&self.uniform, &self.buffer);
    }

    pub fn set_transform(&mut self, transform: Matrix4<f32>) {
        self.uniform.set(ModelUniform::new(transform));
    }
}
*/