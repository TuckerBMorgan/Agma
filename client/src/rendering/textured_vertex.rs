use storm::graphics::{VertexAttribute, VertexDescriptor, VertexInputType, VertexOutputType};
use storm::graphics::VertexInstancing;
/// Configuration settings for a sprite.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TexturedVertex {
    pub vertices: [f32;3],
    pub uv: [f32;2]
}

impl VertexDescriptor for TexturedVertex {
    const INSTANCING: VertexInstancing = VertexInstancing::none();
    const ATTRIBUTES: &'static [VertexAttribute] = &[
        VertexAttribute::new(3, VertexInputType::F32, VertexOutputType::F32),
        VertexAttribute::new(2, VertexInputType::F32, VertexOutputType::F32),
    ];
}

impl Default for TexturedVertex {
    fn default() -> TexturedVertex {
        TexturedVertex {
            vertices: [0.0, 0.0, 0.0],
            uv: [0.0, 0.0]
        }
    }
}