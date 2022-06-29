use storm::cgmath::*;
use std::ops::Mul;
use crate::rendering::SkinnedAnimation;
use std::collections::HashMap;
use crate::rendering::create_skinned_mesh_from_file;

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
        self.head_node.calculate_final_joint_matrix(self.head_node.transform, output_matrices, inverse_parent_transform, 0, animation_transform);
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

    pub fn calculate_transform(&self, time: f32) -> Option<Matrix4<f32>> {
        
        if self.input_timeline[0] > time {
            return None;
        }

        if self.input_timeline[self.input_timeline.len() - 1] < time {
            return None;
        }

        for i in 0..self.input_timeline.len() {
            if self.input_timeline[i] > time {
                let t =  (time - self.input_timeline[i - 1]) / (self.input_timeline[i] - self.input_timeline[i - 1]);
                let translation = lerp_vector3(self.output_translations[i - 1], self.output_translations[i], t);
                return Some(Matrix4::from_translation(translation));
            }
        }
        return None;
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
    
    pub fn calculate_scale(&self, time: f32) -> Option<Matrix4<f32>> {
        if self.input_timeline[0] > time {
            return None;
        }

        if self.input_timeline[self.input_timeline.len() - 1] < time {
            return None;
        }

        for i in 0..self.input_timeline.len() {
            if self.input_timeline[i] > time {
                let t =  (time - self.input_timeline[i - 1]) / (self.input_timeline[i] - self.input_timeline[i - 1]);

                let scale = lerp_vector3(self.output_scales[i - 1], self.output_scales[i], t);
                return Some(Matrix4::from_nonuniform_scale(scale[0], scale[1], scale[2]));
            }
        }
        return None;
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

    pub fn calculate_rotation(&self, time: f32) -> Option<Matrix4<f32>> {

        if self.input_timeline[0] > time {
            return None;
        }

        if self.input_timeline[self.input_timeline.len() - 1] < time {
            return None;
        }

        for i in 0..self.input_timeline.len() {
            if self.input_timeline[i] > time {
                let t =  (time - self.input_timeline[i - 1]) / (self.input_timeline[i] - self.input_timeline[i - 1]);

                let rotation = self.output_rotations[i - 1].nlerp(self.output_rotations[i], t);
                let rotation_transform : Matrix4<f32> = rotation.into();
                return Some(rotation_transform);
            }
        }
        return None;
    }
}

#[derive(Debug)]
pub struct NodeAnimation {
    pub target: usize,
    pub default_translation: Vector3<f32>,
    pub default_scale: Vector3<f32>,
    pub default_rotation: Quaternion<f32>,    
    pub translations: Vec<Translation>,
    pub scales: Vec<Scale>,
    pub rotations: Vec<RotationAnim>
}

impl NodeAnimation {
    pub fn new(target: usize, default_translation: Vector3<f32>, default_scale: Vector3<f32>, default_rotation: Quaternion<f32>) -> NodeAnimation {
        NodeAnimation {
            target,
            default_translation,
            default_scale,
            default_rotation,
            translations: vec![],
            scales: vec![],
            rotations: vec![]
        }
    }

    pub fn caluclate_rotation(&self, time: f32) -> Option<Matrix4<f32>> {
        let mut final_transform = Matrix4::identity();
        let mut has_animation = false;
        for rotation in self.rotations.iter() {
            match rotation.calculate_rotation(time) {
                Some(rotation) => {
                    final_transform = final_transform.mul(rotation);
                    has_animation = true;
                }
                _ => {

                }
            }
        }
        if has_animation {
            return Some(final_transform);
        }
        return None;
    }

    pub fn calculate_transform(&self, time: f32) -> Option<Matrix4<f32>> {
        let mut final_transform = Matrix4::identity();
        let mut has_animation = false;
        for translation in self.translations.iter() {
            match translation.calculate_transform(time) {
                Some(translation) => {
                    final_transform = final_transform.mul(translation);
                    has_animation = true;
                },
                _ => {

                }
            }
        }

        if has_animation {
            return Some(final_transform);
        }
        return None;
    }

    pub fn calculate_scale(&self, time: f32) -> Option<Matrix4<f32>> {
        let mut final_transform = Matrix4::identity();
        let mut has_animation = false;
        for scale in self.scales.iter() {
            match scale.calculate_scale(time) {
                Some(scale) => {
                    final_transform = final_transform.mul(scale);
                    has_animation = true;
                }
                _ => {

                }
            }
        }

        if has_animation {
            return Some(final_transform);
        }

        return None;
    }

    pub fn caluclate_animation(&self, time: f32) -> Option<Matrix4<f32>> {

        let translation = self.calculate_transform(time);
        let scale = self.calculate_scale(time);
        let rotation = self.caluclate_rotation(time);

        if translation.is_none() && scale.is_none() && rotation.is_none() {
            return None;
        }

        let translation_matrix;
        match translation {
            Some(translation) => {
                translation_matrix = translation;
            },
            _ => {
                translation_matrix = Matrix4::from_translation(self.default_translation);
            }
        }

        let scale_matrix;
        match scale {
            Some(scale) => {
                scale_matrix = scale;
            },
            _ => {
                scale_matrix =Matrix4::from_nonuniform_scale(self.default_scale[0], self.default_scale[1], self.default_scale[2]);
            }
        }

        let rotation_matrix;
        match rotation {
            Some(rotation) => {
                rotation_matrix = rotation;
            },
            _ => {

                rotation_matrix = self.default_rotation.into();
            }
        }

        return Some(translation_matrix.mul(rotation_matrix).mul(scale_matrix));

    }
}

pub struct AnimationLibrary {
    pub loaded_animations: HashMap<String, SkinnedAnimation>,
}

impl AnimationLibrary {
    pub fn new() -> AnimationLibrary {
        AnimationLibrary {
            loaded_animations: HashMap::new()
        }
    }

    pub fn load_animation(&mut self, name: String, path: String) {
        self.loaded_animations.insert(name.clone(), create_skinned_mesh_from_file(path));
    }
}