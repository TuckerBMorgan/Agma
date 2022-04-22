use cgmath::*;

/// Used to tag an entity as a minion, and hold control
/// variables used by them
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct MinionComponent {
    pub champion_index: u32,
    pub destinations : [Vector3<f32>; 2],
    pub current_index: usize
}

impl MinionComponent {
    pub fn new() -> MinionComponent {
        MinionComponent {
            champion_index: 0,
            destinations: [Vector3::new(0.0, 0.0, 0.0), Vector3::new(10.0, 0.0, 10.0)],
            current_index: 0
        }
    }
}


impl bincode::Encode for MinionComponent {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.champion_index, encoder)?;

        bincode::Encode::encode(&self.destinations[0].x, encoder)?;
        bincode::Encode::encode(&self.destinations[0].y, encoder)?;
        bincode::Encode::encode(&self.destinations[0].z, encoder)?;

        bincode::Encode::encode(&self.destinations[1].x, encoder)?;
        bincode::Encode::encode(&self.destinations[1].y, encoder)?;
        bincode::Encode::encode(&self.destinations[1].z, encoder)?;

        bincode::Encode::encode(&self.current_index, encoder)?;
        Ok(())
    }
}

impl bincode::Decode for MinionComponent {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> core::result::Result<Self, bincode::error::DecodeError> {
        return Ok(MinionComponent {
                champion_index: bincode::Decode::decode(decoder).unwrap(),
                destinations:  [Vector3::new(bincode::Decode::decode(decoder).unwrap(), bincode::Decode::decode(decoder).unwrap(), bincode::Decode::decode(decoder).unwrap()), 
                                Vector3::new(bincode::Decode::decode(decoder).unwrap(), bincode::Decode::decode(decoder).unwrap(), bincode::Decode::decode(decoder).unwrap())
                            ],
                current_index: bincode::Decode::decode(decoder).unwrap()
        });
    }
}