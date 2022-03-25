impl :: bincode :: Decode for UpdateWorldMessage
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
           {
               message_type : :: bincode :: Decode :: decode(decoder) ?,
               current_frame_number : :: bincode :: Decode :: decode(decoder)
               ?, delta_frame_number : :: bincode :: Decode :: decode(decoder)
               ?, data : :: bincode :: Decode :: decode(decoder) ?,
           })
    }
}