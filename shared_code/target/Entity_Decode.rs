impl :: bincode :: Decode for Entity
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
           {
               id : :: bincode :: Decode :: decode(decoder) ?, pos : ::
               bincode :: Decode :: decode(decoder) ?,
           })
    }
}