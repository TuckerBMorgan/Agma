impl :: bincode :: Decode for PlayerToServerMessage
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
           {
               message_type : :: bincode :: Decode :: decode(decoder) ?, data
               : :: bincode :: Decode :: decode(decoder) ?,
           })
    }
}