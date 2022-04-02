use crate::rendering::{ModelVertex, TexturedVertex};
pub const  G_VERTEX_BUF32F32ER_DATA : [f32; 108] = [
    //Top Face
    1.0f32, 1.0f32, -1.0f32,
    -1.0f32, 1.0f32, 1.0f32,
    1.0f32, 1.0f32, 1.0f32,    
    -1.0f32, 1.0f32, -1.0f32,
    -1.0f32, 1.0f32, 1.0f32,
    1.0f32, 1.0f32, -1.0f32,


    //Bottom Face
    1.0f32, -1.0f32, 1.0f32,
    -1.0f32, -1.0f32, 1.0f32,
    1.0f32, -1.0f32, -1.0f32,
    1.0f32, -1.0f32, -1.0f32,
    -1.0f32, -1.0f32, 1.0f32,
    -1.0f32, -1.0f32, -1.0f32,

    //Front Face
    1.0f32, 1.0f32, 1.0f32,
    -1.0f32, 1.0f32, 1.0f32,
    1.0f32, -1.0f32, 1.0f32,
    -1.0f32, -1.0f32, 1.0f32,
    1.0f32, -1.0f32, 1.0f32,
    -1.0f32, 1.0f32, 1.0f32,

    //Back f32ace
    1.0f32, -1.0f32, -1.0f32,
    -1.0f32, 1.0f32, -1.0f32,
    1.0f32, 1.0f32, -1.0f32,
    -1.0f32, 1.0f32, -1.0f32,
    1.0f32, -1.0f32, -1.0f32,
    -1.0f32, -1.0f32, -1.0f32,

    //Right Face
    1.0f32, -1.0f32, 1.0f32,
    1.0f32, 1.0f32, -1.0f32,
    1.0f32, 1.0f32, 1.0f32,
    1.0f32, 1.0f32, -1.0f32,
    1.0f32, -1.0f32, 1.0f32,
    1.0f32, -1.0f32, -1.0f32,
    
    //Lef32t Face

    -1.0f32, 1.0f32, 1.0f32,
    -1.0f32, 1.0f32, -1.0f32,
    -1.0f32, -1.0f32, 1.0f32,
    -1.0f32, -1.0f32, -1.0f32,
    -1.0f32, -1.0f32, 1.0f32,
    -1.0f32, 1.0f32, -1.0f32,
];

#[allow(dead_code)]
// One color f32or each vertex. They were generated randomly.
pub const G_COLOR_BUF32F32ER_DATA : [f32; 108] = [
    1.0,     0.0,     0.0,
    1.0,     0.0,     0.0,
    1.0,     0.0,     0.0,
    1.0,     0.0,     0.0,
    1.0,     0.0,     0.0,
    1.0,     0.0,     0.0,

    1.0,     0.5,     0.0,
    1.0,     0.5,     0.0,
    1.0,     0.5,     0.0,
    1.0,     0.5,     0.0,
    1.0,     0.5,     0.0,
    1.0,     0.5,     0.0,

    0.0,     1.0,     0.0,
    0.0,     1.0,     0.0,
    0.0,     1.0,     0.0,
    0.0,     1.0,     0.0,
    0.0,     1.0,     0.0,
    0.0,     1.0,     0.0,

    0.0,     1.0,     0.5,
    0.0,     1.0,     0.5,
    0.0,     1.0,     0.5,
    0.0,     1.0,     0.5,
    0.0,     1.0,     0.5,
    0.0,     1.0,     0.5,

    0.0,     0.0,     0.75,
    0.0,     0.0,     0.75,
    0.0,     0.0,     0.75,
    0.0,     0.0,     0.75,
    0.0,     0.0,     0.75,
    0.0,     0.0,     0.75,

    0.5,     0.0,     1.0,
    0.5,     0.0,     1.0,
    0.5,     0.0,     1.0,
    0.5,     0.0,     1.0,
    0.5,     0.0,     1.0,
    0.5,     0.0,     1.0,
];
/*
static const GLf32loat g_normal_buf32f32er_data[] = {
    0.0f32, 1.0f32, 0.0f32,
    0.0f32, 1.0f32, 0.0f32,
    0.0f32, 1.0f32, 0.0f32,
    0.0f32, 1.0f32, 0.0f32,
    0.0f32, 1.0f32, 0.0f32,
    0.0f32, 1.0f32, 0.0f32,

    0.0f32, -1.0f32, 0.0f32,
    0.0f32, -1.0f32, 0.0f32,
    0.0f32, -1.0f32, 0.0f32,
    0.0f32, -1.0f32, 0.0f32,
    0.0f32, -1.0f32, 0.0f32,
    0.0f32, -1.0f32, 0.0f32,

    0.0f32, 0.0f32, 1.0f32,
    0.0f32, 0.0f32, 1.0f32,
    0.0f32, 0.0f32, 1.0f32,
    0.0f32, 0.0f32, 1.0f32,
    0.0f32, 0.0f32, 1.0f32,
    0.0f32, 0.0f32, 1.0f32,

    0.0f32, 0.0f32, -1.0f32,
    0.0f32, 0.0f32, -1.0f32,
    0.0f32, 0.0f32, -1.0f32,
    0.0f32, 0.0f32, -1.0f32,
    0.0f32, 0.0f32, -1.0f32,
    0.0f32, 0.0f32, -1.0f32,

    1.0f32, 0.0f32, 0.0f32,
    1.0f32, 0.0f32, 0.0f32,
    1.0f32, 0.0f32, 0.0f32,
    1.0f32, 0.0f32, 0.0f32,
    1.0f32, 0.0f32, 0.0f32,
    1.0f32, 0.0f32, 0.0f32,

    -1.0f32, 0.0f32, 0.0f32,
    -1.0f32, 0.0f32, 0.0f32,
    -1.0f32, 0.0f32, 0.0f32,
    -1.0f32, 0.0f32, 0.0f32,
    -1.0f32, 0.0f32, 0.0f32,
    -1.0f32, 0.0f32, 0.0f32,

};
*/
#[allow(dead_code)]
pub fn create_triangle() -> [ModelVertex; 3] {
    let mut cube_data = [ModelVertex::default();3];
    cube_data[0].vertices = [0.0, 1.0, -1.0];
    cube_data[0].vertex_color = [1.0, 0.0, 0.0];
    cube_data[1].vertices = [0.0, 1.0, 1.0];
    cube_data[1].vertex_color = [1.0, 0.0, 0.0];
    cube_data[2].vertices = [0.0, 0.0, 0.0];
    cube_data[2].vertex_color = [1.0, 0.0, 0.0];
    cube_data
}

#[allow(dead_code)]
pub fn create_plane() -> [TexturedVertex; 6] {
    let mut plane_data = [TexturedVertex::default();6];
    
    plane_data[0].vertices = [1.0f32, 0.0f32, -1.0f32];
    plane_data[0].uv = [1.0, 0.0];
    plane_data[1].vertices = [-1.0f32, 0.0f32, 1.0f32];
    plane_data[1].uv = [0.0, 1.0];
    plane_data[2].vertices = [1.0f32, 0.0f32, 1.0f32];
    plane_data[2].uv = [0.0, 0.0];

    plane_data[3].vertices = [-1.0f32, 0.0f32, -1.0f32];
    plane_data[3].uv = [1.0, 1.0];
    plane_data[4].vertices = [-1.0f32, 0.0f32, 1.0f32];
    plane_data[4].uv = [0.0, 1.0];
    plane_data[5].vertices = [1.0f32, 0.0f32, -1.0f32];
    plane_data[5].uv = [1.0, 0.0];
    plane_data
}

#[allow(dead_code)]
pub fn create_cube() -> [ModelVertex;36] {
    let mut cube_data = [ModelVertex::default();36];
    for i in 0..36 {
        let offset = i * 3;
        cube_data[i].vertices = [G_VERTEX_BUF32F32ER_DATA[offset] , G_VERTEX_BUF32F32ER_DATA[offset + 1] , G_VERTEX_BUF32F32ER_DATA[offset + 2]];
        cube_data[i].vertex_color = [G_COLOR_BUF32F32ER_DATA[offset], G_COLOR_BUF32F32ER_DATA[offset + 1], G_COLOR_BUF32F32ER_DATA[offset + 2]];
    }
    cube_data
}