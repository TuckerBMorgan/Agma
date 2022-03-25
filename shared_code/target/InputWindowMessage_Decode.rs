impl :: bincode :: Decode for InputWindowMessage
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
           {
               message_type : :: bincode :: Decode :: decode(decoder) ?,
               input_messages : :: bincode :: Decode :: decode(decoder) ?,
           })
    }
}