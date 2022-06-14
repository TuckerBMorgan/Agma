use storm::Context;
use storm::graphics::{Buffer, std140,DrawMode, Shader, ShaderDescriptor,Uniform};
use storm::math::PerspectiveCamera;
use crate::rendering::SkinnedVertex;
use storm::cgmath::Matrix4;
use crate::AgmaClientApp;
use storm::cgmath::Zero;

impl ShaderDescriptor<0> for SkinnedShader {
    const VERTEX_SHADER: &'static str = include_str!("skinned_vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    const TEXTURE_NAMES: [&'static str; 0] = [];
    type VertexUniformType = SkinnedUniform;
    type VertexDescriptor = SkinnedVertex;
}

#[std140::uniform]
#[derive(Copy, Clone)]
pub struct SkinnedUniform {
    pub ortho: std140::mat4,
    pub joint_matrices: [std140::mat4;256],
}

impl SkinnedUniform {
    pub fn new(ortho: Matrix4<f32>, joint_matrices: [Matrix4<f32>;256]) -> SkinnedUniform {

        let joint_matrices : Vec<std140::mat4> = joint_matrices.iter().map(|x|(*x).into()).collect();
        let mut test = [std140::mat4::zero();256];
        for i in 0..256 {
            test[i] = joint_matrices[i];
        }
        SkinnedUniform {
            ortho: ortho.into(),
            joint_matrices: test
        }
    }
}

pub struct SkinnedShader {
    shader: Shader<SkinnedShader, 0>,
}

impl SkinnedShader {
    pub fn new(ctx: &mut Context<AgmaClientApp>) -> SkinnedShader {
        SkinnedShader {
            shader: Shader::new(ctx),
        }
    }

    /// Draws to the screen.
    pub fn draw(&self, uniform: &Uniform<SkinnedUniform>, buffer: &Buffer<SkinnedVertex>) {
        self.shader.draw(DrawMode::Triangles, uniform, [], &[buffer]);
    }
}


pub struct SkinnedShaderPass {
    pub uniform: Uniform<SkinnedUniform>,
    pub buffer: Buffer<SkinnedVertex>,
}

impl SkinnedShaderPass {
    pub fn new(ortho: Matrix4<f32>, ctx: &mut Context<AgmaClientApp>) -> SkinnedShaderPass {
        let mut test = [Matrix4::zero();256];
        SkinnedShaderPass {
            uniform: Uniform::new(ctx, SkinnedUniform::new(ortho, test)),
            buffer: Buffer::new(ctx),
        }
    }

    /// Draws the pass to the screen.
    pub fn draw(&mut self, shader: &SkinnedShader) {
        shader.draw(&self.uniform, &self.buffer);
    }

    pub fn set_uniform(&mut self, transform: Matrix4<f32>, joint_data: [Matrix4<f32>;256]) {
        self.uniform.set(SkinnedUniform::new(transform, joint_data));
    }
}