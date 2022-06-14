use storm::cgmath::*;
use gltf::animation::util::Rotations::F32;
use crate::rendering::SkinnedVertex;
use crate::rendering::Skeleton;
use crate::rendering::NodeAnimation;
use std::ops::Mul;
use log::info;
use log::LevelFilter;

use std::collections::HashMap;


pub struct SkinnedAnimation {
    pub model: Vec<SkinnedVertex>,
    pub joint_matrices: [Matrix4<f32>;256],
    pub skeleton: Skeleton,
    pub node_animations: HashMap<usize, NodeAnimation>
}

impl SkinnedAnimation {
    pub fn new(model: Vec<SkinnedVertex>, joint_matrices: [Matrix4<f32>;256], skeleton: Skeleton, node_animations: HashMap<usize, NodeAnimation>) -> SkinnedAnimation {        
        SkinnedAnimation {
            model,
            joint_matrices,
            skeleton,
            node_animations
        }
    }

    pub fn calculate_joint_matrix(&self, time: f32) -> [Matrix4<f32>;256] {
        let mut base = [Matrix4::identity();256];
        let mut animation = [(false, Matrix4::identity());256];

        for k in self.node_animations.keys() {
            let matrix = self.node_animations[k].calculate_matrix(time);
            animation[self.node_animations[k].target] = (true, matrix);
        }

        self.skeleton.calculate_final_joint_matrix(&mut base, &animation);

        return base;
    }
}
