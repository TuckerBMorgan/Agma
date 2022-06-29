use storm::cgmath::*;
use crate::rendering::SkinnedVertex;
use crate::rendering::Skeleton;
use crate::rendering::NodeAnimation;

use std::collections::HashMap;


pub struct SkinnedAnimation {
    pub model: Vec<SkinnedVertex>,
    pub joint_matrices: [Matrix4<f32>;256],
    pub skeleton: Skeleton,
    pub node_animations: HashMap<usize, NodeAnimation>,
    pub length_of_animation: f32
}

impl SkinnedAnimation {
    pub fn new(model: Vec<SkinnedVertex>, joint_matrices: [Matrix4<f32>;256], skeleton: Skeleton, node_animations: HashMap<usize, NodeAnimation>, length_of_animation: f32) -> SkinnedAnimation {        
        SkinnedAnimation {
            model,
            joint_matrices,
            skeleton,
            node_animations,
            length_of_animation
        }
    }

    pub fn calculate_joint_matrix(&mut self, time: f32) -> [Matrix4<f32>;256] {

        let mut base = [Matrix4::identity();256];
        let mut animation = [(false, Matrix4::identity());256];

        for k in self.node_animations.keys() {
            let matrix = self.node_animations[k].caluclate_animation(time);
            match matrix {
                Some(matrix) => {
                    animation[self.node_animations[k].target] = (true, matrix);
                },
                _ => {
                    
                }
            }

        }

        self.skeleton.calculate_final_joint_matrix(&mut base, &animation);
        return base;
    }
}
