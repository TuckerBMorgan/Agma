use std::ops::Mul;
use cgmath::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct LazyTranslation {
    pub current_translation: Vector3<f32>,
    pub desired_translation: Vector3<f32>
}

impl LazyTranslation {
    pub fn new(translation: Vector3<f32>) -> LazyTranslation {
        LazyTranslation {
            current_translation: translation,
            desired_translation: translation
        }
    }

    pub fn update(&mut self) {
        let direction = self.desired_translation - self.current_translation;//.normalize();
        if direction.magnitude() < 0.1  {
            self.current_translation = self.desired_translation;
        }
        else {
            self.current_translation += direction.magnitude() * 0.05f32 * direction.normalize();
        }
    }
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct LazyRotation {
    pub current_rotation: f32,
    pub desired_rotation: f32
}

impl LazyRotation {
    pub fn new(rotation: f32) -> LazyRotation {
        LazyRotation {
            current_rotation: rotation,
            desired_rotation: rotation
        }
    }

    pub fn update(&mut self) {
        if self.current_rotation == self.desired_rotation {
            return;
        }

        let direction = (self.current_rotation - self.desired_rotation).signum();
        self.current_rotation += direction * -1.0 * 10.0;

        if self.current_rotation < 0.0 {
            self.current_rotation = 360.0 + self.current_rotation;
        }

        self.current_rotation = self.current_rotation % 360.0;
        
        let distance = 180.0 - (((self.current_rotation - self.desired_rotation).abs() % 360.0) - 180.0).abs();
        if distance <= 10.0 {
            self.current_rotation = self.desired_rotation;
        }

        if self.current_rotation == std::f32::NAN {
            self.current_rotation = 0.0f32;
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct LazyScale {
    pub current_scale: Vector3<f32>,
    pub desired_scale: Vector3<f32>
}

impl LazyScale {
    pub fn new(scale: Vector3<f32>) -> LazyScale {
        LazyScale {
            current_scale: scale,
            desired_scale: scale
        }
    }

    pub fn update(&mut self) {
        let direction = self.desired_scale - self.current_scale;//.normalize();
        if direction.magnitude() < 0.1  {
            self.current_scale = self.desired_scale;
        }
        else {
            self.current_scale += direction.normalize() * 0.02f32;
        }
    }
}

/// The Transform of an entity
/// used to represent to where an entity is
/// what scale they are 
/// and where they are pointing
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct TransformComponent {
    pub translation: LazyTranslation,
    rotation: LazyRotation,
    scale: LazyScale,
    matrix: Matrix4<f32>,
    dirty: bool
}

impl TransformComponent {
    pub fn new(translation: Vector3<f32>, rotation: f32, scale: Vector3<f32>) -> TransformComponent {
        let rotation_transform : Matrix4<f32> = Matrix4::from_angle_y(cgmath::Rad(rotation * (3.14 / 180.0) ));
        let matrix = Matrix4::from_translation(translation).mul(rotation_transform).mul(Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z));
        TransformComponent {
            translation: LazyTranslation::new(translation),
            rotation: LazyRotation::new(rotation),
            scale: LazyScale::new(scale),
            matrix,
            dirty: false
        }
    }
    
    pub fn position(&self) -> Vector3<f32> {
        return self.translation.current_translation;
    }

    pub fn set_desired_translation(&mut self, desired_translation: Vector3<f32>) {
        self.translation.desired_translation = desired_translation;
        self.dirty = true;
    }

    pub fn set_desired_scale(&mut self, desired_scale: Vector3<f32>) {
        self.scale.desired_scale = desired_scale;
        self.dirty = true;
    }

    pub fn set_desired_rotation(&mut self, set_desired_rotation: f32) {
        self.rotation.desired_rotation = set_desired_rotation;
        self.dirty = true;
    }

    pub fn matrix(&mut self) -> Matrix4<f32> {
        if self.dirty {
            let rotation_transform : Matrix4<f32> = Matrix4::from_angle_y(cgmath::Rad(self.rotation.current_rotation * (3.14 / 180.0)));
            let scale = self.scale.current_scale;
            self.matrix = Matrix4::from_translation(self.translation.current_translation).mul(rotation_transform).mul(Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z));
            self.dirty = false;
        }
        return self.matrix;
    }
    
    pub fn update_transform(&mut self) {
        self.dirty = true;
        self.translation.update();
        self.scale.update();
        self.rotation.update();
    }
    
}

impl bincode::Encode for TransformComponent {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        _encoder: &mut E,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        Ok(())
    }
}

impl bincode::Decode for TransformComponent {
    fn decode<D: bincode::de::Decoder>(
        _decoder: &mut D,
    ) -> core::result::Result<Self, bincode::error::DecodeError> {
        return Ok(
            TransformComponent::new(Vector3::new(0.0, 0.0, 0.0), 0.0, Vector3::new(0.0, 0.0, 0.0))
        );
    }
}