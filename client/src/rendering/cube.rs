use crate::rendering::{ModelVertex, TexturedVertex, SkinnedAnimation, SkinnedVertex, NodeAnimation, Translation, Scale, RotationAnim, SkeletonNode, Skeleton};
use gltf::*;
use gltf::animation::util::Rotations::F32;
use storm::cgmath::*;
use std::collections::HashMap;
use std::ops::Mul;

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

fn generate_base_skeleton(node: &Node) -> SkeletonNode {
    let test = node.transform().matrix();
    let transform = Matrix4::from_cols(test[0].into(), test[1].into(), test[2].into(), test[3].into());
    let mut skelenton_node = SkeletonNode::new(node.index(), transform);
    for child in node.children() {
        skelenton_node.add_child(generate_base_skeleton(&child));
    }

    
    return skelenton_node;
}

/*
fn gen_skel_2(node: &Node, nodes: &mut Vec<JointNode>, parent_index: usize) {
    let test = node.transform().matrix();
    let transform = Matrix4::from_cols(test[0].into(), test[1].into(), test[2].into(), test[3].into());

    let mut new_node = JointNode::new(transform, parent_index, node.index(), Matrix4::identity());
    nodes.push(new_node);
    let parent = nodes.len() - 1;
    for child in node.childern() {
        gen_skel_2(&child, nodes, parent);
  
    }
}
*/


fn calculate_world_transform(cached_calculated_transform: &mut HashMap<usize, Matrix4<f32>>, node: &Node) {
    let test = node.transform().matrix();
    let transform = Matrix4::from_cols(test[0].into(), test[1].into(), test[2].into(), test[3].into());

    cached_calculated_transform.insert(node.index(), transform.clone());

    if node.children().len() > 0 {
        for child in node.children() {
            calculate_world_transform(cached_calculated_transform,  &child);
        }
    }
}

//"./src/resources/animations/layered_animation.glb"

pub fn create_skinned_mesh_from_file(path: String) -> SkinnedAnimation {
    let (gltf, buffers, _) = gltf::import(path).unwrap();
    let mut root_node_index = 0;
    let mut transforms = HashMap::new();
    let mut check = [Matrix4::identity();256];

    let mut skinned_verteices = vec![];
    for mesh in gltf.meshes() {
        for prim in mesh.primitives() {
            let reader = prim.reader(|buffer| Some(&buffers[buffer.index()]));
            let verticies : Vec<_>= reader.read_positions().unwrap().collect();
            let joints : Vec<_> = reader.read_joints(0).unwrap().into_u16().collect();
            let weights : Vec<_> = reader.read_weights(0).unwrap().into_f32().collect();
            let indices : Vec<_> = reader.read_indices().unwrap().into_u32().collect();

            for index in indices {
                let index = index as usize;
                let sv = SkinnedVertex{vertices: Vector4::new(verticies[index][0], verticies[index][1], verticies[index][2], 1.0), joints: Vector4::new(joints[index][0], joints[index][1], joints[index][2], joints[index][3]), weights: Vector4::new(weights[index][0], weights[index][1], weights[index][2], weights[index][3])};
                skinned_verteices.push(sv);
            }
        }
    }

    let mut skeleton = None;
    for scene in gltf.scenes() {
        for node in scene.nodes() {
            calculate_world_transform( &mut transforms, &node);
            root_node_index = node.index();
            let root_skeleton_node = generate_base_skeleton(&node);
            skeleton = Some(Skeleton::new(root_skeleton_node));
        }
    }

    for skin in gltf.skins() {
        match skin.skeleton() {
            Some(_node) => {

            },
            None => {

                let non_cgmath = transforms[&root_node_index];
                let inverse_transform = Matrix4::from_cols(non_cgmath[0].into(), non_cgmath[1].into(), non_cgmath[2].into(), non_cgmath[3].into());
                let inverse_transform = inverse_transform.invert().unwrap();

                let skin_reader = skin.reader(|buffer| Some(&buffers[buffer.index()]));
                let inverse_bind_transform : Vec<_> = skin_reader.read_inverse_bind_matrices().unwrap().map(|x|Matrix4::from_cols(x[0].into(), x[1].into(), x[2].into(), x[3].into())).collect();
                for (index, joint) in skin.joints().enumerate() {
                    let final_matrix = inverse_transform.mul(transforms[&joint.index()]).mul(inverse_bind_transform[index]);
                    //TODO: THIS IS A BAD, WRITTEN AS A "GET THIS DONE"
                    skeleton.as_mut().unwrap().set_inverse_bind_matrix_and_output_index(joint.index(), inverse_bind_transform[index], index);
                    check[index] = final_matrix;
                }
            }
        }
    }

    let mut animations = HashMap::new();
    let mut longest_animation = 0.0f32;
    for animation in gltf.animations() {

        for channel in animation.channels() {
            let reader = channel.reader(|buffer|Some(&buffers[buffer.index()]));
            let target = channel.target().node().index();
            if animations.contains_key(&target) == false {
                let (translation, rotation, scale) = channel.target().node().transform().decomposed();
                let translation = Vector3::new(translation[0], translation[1], translation[2]);
                let scale = Vector3::new(scale[0], scale[1], scale[2]);
                let rotation = Quaternion::new(rotation[3], rotation[0], rotation[1], rotation[2]);
                animations.insert(target, NodeAnimation::new(target, translation, scale, rotation));
            }

            let animation =  animations.get_mut(&target);
            let animation = animation.unwrap();
            if let Some(iter) = reader.read_outputs() {
                match iter {
                    gltf::animation::util::ReadOutputs::Translations(data) => {
                        let translations : Vec<Vector3<f32>> = data.map(|x|Vector3::new(x[0], x[1], x[2])).collect();
                        let timeline : Vec<f32> = reader.read_inputs().unwrap().collect();
                        if timeline[timeline.len() - 1] > longest_animation{
                            longest_animation = timeline[timeline.len() - 1];
                        }
                        let translation = Translation::new(timeline, translations);
                        animation.translations.push(translation);
                    },
                    gltf::animation::util::ReadOutputs::Scales(data) => {
                        let scales : Vec<Vector3<f32>> = data.map(|x|Vector3::new(x[0], x[1], x[2])).collect();
                        let timeline : Vec<f32> = reader.read_inputs().unwrap().collect();
                        if timeline[timeline.len() - 1] > longest_animation{
                            longest_animation = timeline[timeline.len() - 1];
                        }
                        let scale = Scale::new(timeline, scales);
                        animation.scales.push(scale);
                    },
                    gltf::animation::util::ReadOutputs::Rotations(data) => {
                        match data {
                            F32(data) => {
                                let rotations : Vec<Quaternion<f32>> = data.map(|x|Quaternion::new(x[3], x[0], x[1], x[2])).collect();

                                let timeline : Vec<f32> = reader.read_inputs().unwrap().collect();
                                if timeline[timeline.len() - 1] > longest_animation{
                                    longest_animation = timeline[timeline.len() - 1];
                                }
                                let rotation = RotationAnim::new(timeline, rotations);
                                animation.rotations.push(rotation);
                            },
                            _ => {
                                panic!("we are ignoring rotation data");
                            }
                        }
                    }
                    _=> {
                        
                    }

                }
            }
        }
    }


    return SkinnedAnimation::new(skinned_verteices, check, skeleton.unwrap(), animations, longest_animation);
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