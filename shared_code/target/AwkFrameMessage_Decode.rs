impl :: bincode :: Decode for AwkFrameMessage
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
           {
               message_type : :: bincode :: Decode :: decode(decoder) ?,
               frame_number : :: bincode :: Decode :: decode(decoder) ?,
           })
    }
}