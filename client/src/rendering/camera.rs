use storm::*;
use storm::math::PerspectiveCamera;
use storm::cgmath::*;
use storm::event::*;
use storm::graphics::{Uniform};
use std::ops::{Mul};
use storm::math::Float;
use crate::rendering::*;
use crate::AgmaClientApp;
use log::*;

pub struct Camera {
    /// Transform matix.
    pub transform: PerspectiveCamera,
    /// Transform uniform.
    pub uniform: Uniform<ModelUniform>,
    /// Position vector.
    pub pos: Vector3<f32>,
    /// Unnormalized direction vector.
    pub dir: Vector3<f32>,
    /// Normalized horizontal xz plane direction vector.
    pub forward: Vector2<f32>,
    pub yaw: f32,
    pub pitch: f32,
    /// Positive is forward.
    pub forward_speed: f32,
    /// Positive is right.
    pub strafe_speed: f32,
    /// Positive is up.
    pub vertical_speed: f32,
    pub multiplier: f32,
}

impl Camera {
    pub fn new(ctx: &mut Context<AgmaClientApp>) -> Camera {
        let mut transform = PerspectiveCamera::new(ctx.window_logical_size());
        let uniform = Uniform::new(ctx, &mut transform);
        Camera {
            transform,
            uniform,
            pos: Vector3::new(0.0, 10.0, 10.0),
            dir: Vector3::zero(),
            forward: Vector2::zero(),
            yaw: 0.0,
            pitch: 0.0,
            forward_speed: 0.0,
            strafe_speed: 0.0,
            vertical_speed: 0.0,
            multiplier: 2.0,
        }
    }

    pub fn resize(&mut self, logical_size: Vector2<f32>) {
        self.transform.set_size(logical_size);
        self.uniform.set(&mut self.transform);
    }

    pub fn look(&mut self, cursor_delta: Vector2<f32>) {
        const SENSITIVITY: f32 = 0.12; // Degrees per delta unit.

        self.yaw += cursor_delta.x * SENSITIVITY;
        if self.yaw < 0.0 {
            self.yaw = 360.0 - self.yaw;
        } else if self.yaw > 360.0 {
            self.yaw = self.yaw - 360.0;
        }

        self.pitch += cursor_delta.y * SENSITIVITY;
        if self.pitch < -90.0 {
            self.pitch = -90.0;
        } else if self.pitch > 89.0 {
            self.pitch = 89.0;
        }

        let cos_pitch = self.pitch.cos_deg_fast();
        self.forward = Vector2::new(self.yaw.cos_deg_fast(), self.yaw.sin_deg_fast());
        let x = cos_pitch * self.forward.x;
        let y = self.pitch.sin_deg_fast();
        let z = cos_pitch * self.forward.y;
        self.dir = Vector3::new(x, y, z);
        self.transform.set().direction = self.dir;
        self.uniform.set(&mut self.transform);
    }

    /*
    pub fn look_at(& mut self, point: Vector3<f32>) {
        self.dir = point - self.pos;
        self.transform.set().direction = self.dir;
        self.uniform.set(&mut self.transform);
    }
    */

    pub fn update(&mut self, time_delta: f32) {
        let forward_speed = time_delta * self.forward_speed * self.multiplier;
        let strafe_speed = time_delta * self.strafe_speed * self.multiplier;
        let vertical_speed = time_delta * self.vertical_speed * self.multiplier;
        self.pos.x += (self.forward.x * forward_speed) + (-self.forward.y * strafe_speed);
        self.pos.z += (self.forward.y * forward_speed) + (self.forward.x * strafe_speed);
        self.pos.y += vertical_speed;
        self.transform.set().eye = self.pos;
        self.uniform.set(&mut self.transform);
    }

    pub fn model_view_projection_uniform(&mut self, model_transform: &Matrix4<f32>) -> &Uniform<ModelUniform> { 
        let _ = &self.uniform.set(ModelUniform::new(self.transform.matrix().mul(model_transform)));
        &self.uniform
    }
}