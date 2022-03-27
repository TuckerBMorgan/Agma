use storm::graphics::{VertexAttribute, VertexDescriptor, VertexInputType, VertexOutputType};
use storm::graphics::VertexInstancing;
/// Configuration settings for a sprite.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ModelVertex {
    pub vertices: [f32;3],
    pub vertex_color: [f32;3]
}

impl VertexDescriptor for ModelVertex {
    const INSTANCING: VertexInstancing = VertexInstancing::none();
    const ATTRIBUTES: &'static [VertexAttribute] = &[
        VertexAttribute::new(3, VertexInputType::F32, VertexOutputType::F32),
        VertexAttribute::new(3, VertexInputType::F32, VertexOutputType::F32),
    ];
}

impl Default for ModelVertex {
    fn default() -> ModelVertex {
        ModelVertex {
            vertices: [0.0, 0.0, 0.0],
            vertex_color: [0.0, 0.0, 0.0]
        }
    }
}