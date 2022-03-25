impl :: bincode :: Decode for Vec3
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
           {
               x : :: bincode :: Decode :: decode(decoder) ?, y : :: bincode
               :: Decode :: decode(decoder) ?, z : :: bincode :: Decode ::
               decode(decoder) ?,
           })
    }
}