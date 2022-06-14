use storm::cgmath::*;
use gltf::animation::util::Rotations::F32;
use crate::rendering::ModelVertex;
use std::ops::Mul;
use log::info;
use log::LevelFilter;


#[derive(Debug)]
pub struct SkeletonNode {
    index: usize,
    inverse_bind_transform: Matrix4<f32>,
    transform: Matrix4<f32>,
    childern: Vec<SkeletonNode>,
    output_index: usize
}

impl SkeletonNode {
    pub fn new(index: usize, transform: Matrix4<f32>) -> SkeletonNode {
        SkeletonNode {
            index,
            transform,
            childern: vec![],
            inverse_bind_transform: Matrix4::<f32>::identity(),
            output_index: 255
        }
    }

    pub fn add_child(&mut self, node: SkeletonNode) {
        self.childern.push(node);
    }

    pub fn calculate_final_joint_matrix(&self, parent_transform: Matrix4<f32>, output_matrices: &mut [Matrix4<f32>;256], inverse_global_transform: Matrix4<f32>, depth: usize, animation_transform: &[(bool, Matrix4<f32>);256]) {
        
        let (has_animation, animated_transform) = animation_transform[self.index];

        let use_transform;
        if has_animation {
            use_transform = animated_transform;
        }
        else {
            use_transform = self.transform;
        }
        let global_transform = parent_transform.mul(use_transform);
        let joint_transform = global_transform.mul(self.inverse_bind_transform);
        let joint_transform = inverse_global_transform.mul(joint_transform);


        output_matrices[self.output_index] = joint_transform;
        info!("{:?}", output_matrices[self.output_index]);

        for child in self.childern.iter() {
            child.calculate_final_joint_matrix(global_transform, output_matrices, inverse_global_transform, depth + 1, animation_transform);
        }
    }

    pub fn set_inverse_bind_matrix_and_output_index(&mut self, node_index: usize, matrix: Matrix4<f32>, output_index: usize) {
        if self.index == node_index {
            self.inverse_bind_transform = matrix;
            self.output_index = output_index;
        }
        else {
            for child in self.childern.iter_mut() {
                child.set_inverse_bind_matrix_and_output_index(node_index, matrix, output_index);
            }
        }
    }
    
}

#[derive(Debug)]
pub struct Skeleton {
    head_node: SkeletonNode
}

impl Skeleton {
    pub fn new(head_node: SkeletonNode) -> Skeleton {
        Skeleton {
            head_node
        }
    }

    pub fn calculate_final_joint_matrix(&self, output_matrices: &mut [Matrix4<f32>; 256], animation_transform: &[(bool, Matrix4<f32>);256]) {
        let inverse_parent_transform = self.head_node.transform.invert().unwrap();
        self.head_node.calculate_final_joint_matrix(Matrix4::identity(), output_matrices, inverse_parent_transform, 0, animation_transform);
    }

    pub fn set_inverse_bind_matrix_and_output_index(&mut self, node_index: usize, matrix: Matrix4<f32>, output_index: usize) {
        self.head_node.set_inverse_bind_matrix_and_output_index(node_index, matrix, output_index);
    }
}

fn lerp(v0: f32, v1: f32, t: f32) -> f32{
    return v0 + t * (v1 - v0);
}

pub fn lerp_vector3(a: Vector3<f32>, b: Vector3<f32>, t: f32) -> Vector3<f32> {
    return Vector3::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t), lerp(a.z, b.z, t));
}

#[derive(Debug)]
pub struct Translation {
    pub input_timeline: Vec<f32>,
    pub output_translations: Vec<Vector3<f32>>
}

impl Translation {
    pub fn new(input_timeline: Vec<f32>, output_translations: Vec<Vector3<f32>>) -> Translation {
        Translation {
            input_timeline,
            output_translations
        }
    }

    pub fn calculate_transform(&self, time: f32) -> Matrix4<f32> {
        
        if self.input_timeline[0] > time {
      //      return Matrix4::from_translation(self.output_translations[0]);
            return Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
        }

        if self.input_timeline[self.input_timeline.len() - 1] < time {
            return Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
            return Matrix4::from_translation(self.output_translations[self.output_translations.len() - 1]);
        }

        for i in 0..self.input_timeline.len() {
            if self.input_timeline[i] > time {
                let t =  (time - self.input_timeline[i - 1]) / (self.input_timeline[i] - self.input_timeline[i - 1]);
                let translation = lerp_vector3(self.output_translations[i - 1], self.output_translations[i], t);
                return Matrix4::from_translation(translation);
            }
        }
        return Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
    }
}

#[derive(Debug)]
pub struct Scale {
    pub input_timeline: Vec<f32>,
    pub output_scales: Vec<Vector3<f32>>,
}

impl Scale {
    pub fn new(input_timeline: Vec<f32>, output_scales: Vec<Vector3<f32>>) -> Scale {
        Scale {
            input_timeline,
            output_scales
        }
    }
    
    pub fn calculate_scale(&self, time: f32) -> Matrix4<f32> {
        if self.input_timeline[0] > time {
            return Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
            let scale = self.output_scales[0];
            return Matrix4::from_nonuniform_scale(scale[0], scale[1], scale[2]);        
        }

        if self.input_timeline[self.input_timeline.len() - 1] < time {
            return Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
            let scale = self.output_scales[self.output_scales.len() - 1];
            return Matrix4::from_nonuniform_scale(scale[0], scale[1], scale[2]);
        }

        for i in 0..self.input_timeline.len() {
            if self.input_timeline[i] > time {
                let t =  (time - self.input_timeline[i - 1]) / (self.input_timeline[i] - self.input_timeline[i - 1]);

                let scale = lerp_vector3(self.output_scales[i - 1], self.output_scales[i], t);
                return Matrix4::from_nonuniform_scale(scale[0], scale[1], scale[2]);
            }
        }
        return Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
    }
}

#[derive(Debug)]
pub struct RotationAnim {
    pub input_timeline: Vec<f32>,
    pub output_rotations: Vec<Quaternion<f32>>
}

impl RotationAnim {
    pub fn new(input_timeline: Vec<f32>, output_rotations: Vec<Quaternion<f32>>) -> RotationAnim {
        RotationAnim {
            input_timeline,
            output_rotations
        }
    }

    pub fn calculate_rotation(&self, time: f32) -> Matrix4<f32> {

        if self.input_timeline[0] > time {
            return Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
            let rotation = self.output_rotations[0];
            let rotation_transform = Matrix4::from_axis_angle(rotation.v.normalize().into(), storm::cgmath::Deg(rotation.s * 60.0));
            return rotation_transform;
        }

        if self.input_timeline[self.input_timeline.len() - 1] < time {

            return Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
            let rotation = self.output_rotations[self.output_rotations.len() - 1];
            let rotation_transform = Matrix4::from_axis_angle(rotation.v.normalize().into(), storm::cgmath::Deg(rotation.s * 60.0));
            return rotation_transform;
        }

        for i in 0..self.input_timeline.len() {
            if self.input_timeline[i] > time {
                let t =  (time - self.input_timeline[i - 1]) / (self.input_timeline[i] - self.input_timeline[i - 1]);

                let rotation = self.output_rotations[i - 1].nlerp(self.output_rotations[i], t);
                let rotation_transform = Matrix4::from_axis_angle(rotation.v.normalize().into(), storm::cgmath::Deg(rotation.s * 60.0));
                return rotation_transform;
            }
        }
        return Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
    }
}

#[derive(Debug)]
pub struct NodeAnimation {
    pub target: usize,
    pub translations: Vec<Translation>,
    pub scales: Vec<Scale>,
    pub rotations: Vec<RotationAnim>
}

impl NodeAnimation {
    pub fn new(target: usize) -> NodeAnimation {
        NodeAnimation {
            target,
            translations: vec![],
            scales: vec![],
            rotations: vec![]
        }
    }

    pub fn calculate_matrix(&self, time: f32) -> Matrix4<f32> {
        let mut final_transform = Matrix4::identity();
        for translation in self.translations.iter() {
            final_transform = final_transform.mul(translation.calculate_transform(time));
        }
        for rotation in self.rotations.iter() {
            final_transform = final_transform.mul(rotation.calculate_rotation(time));
        }

        for scale in self.scales.iter() {
          //  final_transform = final_transform.mul(scale.calculate_scale(time));
        }


        return final_transform;
    }
}

pub struct Animation {
    pub model: Vec<ModelVertex>,
    pub input_timeline: Vec<f32>,
    pub output_translations: Vec<Vector3<f32>>,
    pub output_scale: Vec<Vector3<f32>>,
    pub output_rotations: Vec<Quaternion<f32>>,
    pub last_seen_time_in_seconds:f32,
    pub start_index: usize,
    pub end_index: usize,
    pub max_time: f32,
    pub current_time: f32
}

impl Animation {
    pub fn new(model: Vec<ModelVertex>, input_timeline: Vec<f32>, output_translations: Vec<Vector3<f32>>, output_scale: Vec<Vector3<f32>>, output_rotations: Vec<Quaternion<f32>>) -> Animation {
        let max_time = input_timeline[input_timeline.len() - 1];
        
        Animation {
            model,
            input_timeline,
            output_translations,
            output_scale,
            output_rotations,
            last_seen_time_in_seconds: 0.0,
            start_index: 0,
            end_index: 0,
            max_time,
            current_time: 0.0
        }
    }

    fn set_timeline_indicies(&mut self) {
        //if we are behind, for now stay up, can deal with needing to "rewind"
        //an animation later
        if self.current_time < self.input_timeline[self.start_index] {
            self.end_index = self.start_index + 1;
            return;
        }

        if self.current_time > self.input_timeline[self.start_index] {
            if self.current_time > self.input_timeline[self.end_index] {
                //Ok we need to find out bounds again
                loop {
                    self.end_index += 1;
                    if self.end_index == self.input_timeline.len() {
                        self.end_index = 1;
                        self.start_index = 0;
                        break;
                    }
                    if self.input_timeline[self.end_index] > self.current_time {
                        self.start_index = self.end_index - 1;
                        break;
                    }
                }
            }
        }

        //Otheriwse we don't need to update out timeline indicies
    }

    pub fn get_current_transform(&mut self, delta_time: f32) -> Matrix4<f32> {
        self.current_time += delta_time;
        if self.current_time > self.max_time {
            self.current_time = 0.0;
            self.start_index = 0;
            self.end_index = 0;
        }
        self.set_timeline_indicies();

        let t =  (self.current_time - self.input_timeline[self.start_index]) / (self.input_timeline[self.end_index] - self.input_timeline[self.start_index]);

        let translation = lerp_vector3(self.output_translations[self.start_index], self.output_translations[self.end_index], t);
        let scale = lerp_vector3(self.output_scale[self.start_index], self.output_scale[self.end_index], t);
        let rotation = self.output_rotations[self.start_index].nlerp(self.output_rotations[self.end_index], t);

        let transform = Matrix4::from_translation(translation);
        let scale_transform = Matrix4::from_nonuniform_scale(scale[0], scale[1], scale[2]);
        let rotation_transform = Matrix4::from_axis_angle(rotation.v.normalize().into(), storm::cgmath::Deg(rotation.s * 60.0));

        return transform.mul(scale_transform).mul(rotation_transform);

    }
}