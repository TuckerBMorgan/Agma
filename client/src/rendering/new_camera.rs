use cgmath_dolly::prelude::*;
use storm::graphics::Uniform;
use crate::Vector3;
use storm::math::PerspectiveCamera;
use crate::rendering::ModelUniform;
use storm::cgmath::InnerSpace;
use crate::AgmaClientApp;
use storm::Context;
use std::ops::Mul;

pub struct NewCamera {
    pub transform: PerspectiveCamera,
    pub uniform: Uniform<ModelUniform>,
    rig: CameraRig,
    last_position: Vector3<f32>,
    pub vertical_speed: f32,
    pub forward_speed: f32,
    pub multiplier: f32,
    pub strafe_speed: f32

}

impl NewCamera {
    pub fn new(ctx: &mut Context<AgmaClientApp>) -> NewCamera {
        let camera : CameraRig = CameraRig::builder()
            .with(Position::new(Vector3::new(0.0, 0.0, 0.0)))
            .with(Smooth::new_position(1.5).predictive(true))
            .with(Arm::new(cgmath_dolly::cgmath::Vector3::unit_z() * 5.0))
            .with(Arm::new(cgmath_dolly::cgmath::Vector3::unit_y() * 5.0))        
            .with(YawPitch::new().yaw_degrees(90.0))
            .with(Smooth::new_position(2.5))
            .with(
                LookAt::new(Vector3::new(0.0, 0.0, 0.0))
                    .tracking_smoothness(2.0)
                    .tracking_predictive(true),
            )

            .build();
        let mut transform = PerspectiveCamera::new(ctx.window_logical_size());

        let uniform = Uniform::new(ctx, &mut transform);
        NewCamera {
            transform,
            uniform,
            rig:camera,
            last_position: Vector3::new(0.0, 0.0, 0.0),
            vertical_speed: 0.0,
            forward_speed: 0.0,
            multiplier: 0.0,
            strafe_speed: 0.0
        }
    }

    pub fn update_player_position(&mut self, player_position: Vector3<f32>) {
        self.rig.driver_mut::<Position>().position = player_position;
        self.rig.driver_mut::<LookAt>().target = player_position;
    }

    pub fn update(&mut self, delta_time: f32) {
        let camera_xform = self.rig.update(delta_time);
        self.last_position = camera_xform.position;
        self.transform.set_eye(camera_xform.position);
        self.transform.set_direction(camera_xform.forward());
        self.uniform.set(&mut self.transform);
    }

    pub fn model_view_projection_uniform(&mut self, model_transform: &storm::cgmath::Matrix4<f32>) -> &Uniform<ModelUniform> { 
        let _ = &self.uniform.set(ModelUniform::new(self.transform.matrix().mul(model_transform)));
        &self.uniform
    }

    pub fn point_on_floor_plane(&mut self, screen_normalized_points: storm::cgmath::Vector2<f32>) -> storm::cgmath::Vector3<f32> {
        let world_point = self.transform.screen_to_world_pos(screen_normalized_points);

        let camera_point = self.last_position;
        let direction = (world_point - camera_point).normalize();
        let t = -(world_point.dot(storm::cgmath::Vector3::<f32>::unit_y())) / direction.dot(storm::cgmath::Vector3::<f32>::unit_y());
        let plane_intercept = world_point + (t * direction);
        //Due to API bullshit
        let return_vector = storm::cgmath::Vector3::new(plane_intercept.x, plane_intercept.y, plane_intercept.z);
        return return_vector;
    }
}