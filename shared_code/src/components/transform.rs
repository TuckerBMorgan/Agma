use std::ops::Mul;
use cgmath::*;

/// The Transform of an entity
/// used to represent to where an entity is
/// what scale they are 
/// and where they are pointing
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct TransformComponent {
    pub transform: Matrix4<f32>
}

impl TransformComponent {
    pub fn new(matrix: Matrix4<f32>) -> TransformComponent {
        TransformComponent {
            transform: matrix
        }
    }
    
    pub fn position(&self) -> Vector3<f32> {
        return Vector3::new(self.transform.w.x, self.transform.w.y, self.transform.w.z);
    }

    pub fn move_character(&mut self, offset: Vector3<f32>) {
        self.transform = self.transform.mul(Matrix4::from_translation(offset));
    }
}

impl bincode::Encode for TransformComponent {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.transform.x.x, encoder)?;
        bincode::Encode::encode(&self.transform.x.y, encoder)?;
        bincode::Encode::encode(&self.transform.x.z, encoder)?;
        bincode::Encode::encode(&self.transform.x.w, encoder)?;

        bincode::Encode::encode(&self.transform.y.x, encoder)?;
        bincode::Encode::encode(&self.transform.y.y, encoder)?;
        bincode::Encode::encode(&self.transform.y.z, encoder)?;
        bincode::Encode::encode(&self.transform.y.w, encoder)?;

        bincode::Encode::encode(&self.transform.z.x, encoder)?;
        bincode::Encode::encode(&self.transform.z.y, encoder)?;
        bincode::Encode::encode(&self.transform.z.z, encoder)?;
        bincode::Encode::encode(&self.transform.z.w, encoder)?;

        bincode::Encode::encode(&self.transform.w.x, encoder)?;
        bincode::Encode::encode(&self.transform.w.y, encoder)?;
        bincode::Encode::encode(&self.transform.w.z, encoder)?;
        bincode::Encode::encode(&self.transform.w.w, encoder)?;

        Ok(())
    }
}

impl bincode::Decode for TransformComponent {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> core::result::Result<Self, bincode::error::DecodeError> {
        let transform = Matrix4::new(
            bincode::Decode::decode(decoder).unwrap(), 
            bincode::Decode::decode(decoder).unwrap(), 
            bincode::Decode::decode(decoder).unwrap(),
            bincode::Decode::decode(decoder).unwrap(),

            bincode::Decode::decode(decoder).unwrap(), 
            bincode::Decode::decode(decoder).unwrap(), 
            bincode::Decode::decode(decoder).unwrap(),
            bincode::Decode::decode(decoder).unwrap(),

            bincode::Decode::decode(decoder).unwrap(), 
            bincode::Decode::decode(decoder).unwrap(), 
            bincode::Decode::decode(decoder).unwrap(),
            bincode::Decode::decode(decoder).unwrap(),

            bincode::Decode::decode(decoder).unwrap(), 
            bincode::Decode::decode(decoder).unwrap(), 
            bincode::Decode::decode(decoder).unwrap(),
            bincode::Decode::decode(decoder).unwrap(),
        );
        return Ok(TransformComponent {
            transform
        });
    }
}