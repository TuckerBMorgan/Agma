impl :: bincode :: Decode for Matrix3x3
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
           {
               m11 : :: bincode :: Decode :: decode(decoder) ?, m12 : ::
               bincode :: Decode :: decode(decoder) ?, m13 : :: bincode ::
               Decode :: decode(decoder) ?, m21 : :: bincode :: Decode ::
               decode(decoder) ?, m22 : :: bincode :: Decode ::
               decode(decoder) ?, m23 : :: bincode :: Decode ::
               decode(decoder) ?, m31 : :: bincode :: Decode ::
               decode(decoder) ?, m32 : :: bincode :: Decode ::
               decode(decoder) ?, m33 : :: bincode :: Decode ::
               decode(decoder) ?,
           })
    }
}