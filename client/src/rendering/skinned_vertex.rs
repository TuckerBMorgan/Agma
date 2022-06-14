use storm::graphics::{VertexAttribute, VertexDescriptor, VertexInputType, VertexOutputType};
use storm::graphics::VertexInstancing;
use storm::cgmath::*;
/// Configuration settings for a sprite.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SkinnedVertex {
    pub vertices: Vector4<f32>,
    pub joints: Vector4<u32>,
    pub weights: Vector4<f32>
}

impl VertexDescriptor for SkinnedVertex {
    const INSTANCING: VertexInstancing = VertexInstancing::none();
    const ATTRIBUTES: &'static [VertexAttribute] = &[
        VertexAttribute::new(3, VertexInputType::F32, VertexOutputType::F32),
        VertexAttribute::new(4, VertexInputType::U16, VertexOutputType::I32),
        VertexAttribute::new(4, VertexInputType::F32, VertexOutputType::F32),
    ];
}

impl Default for SkinnedVertex {
    fn default() -> SkinnedVertex {
        SkinnedVertex {
            vertices: Vector4::new(0.0, 0.0, 0.0, 0.0),
            joints: Vector4::new(0, 0, 0, 0),
            weights: Vector4::new(0.0, 0.0, 0.0, 0.0)
        }
    }
}