impl :: bincode :: Decode for World
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
           {
               frame_number : :: bincode :: Decode :: decode(decoder) ?,
               entities : :: bincode :: Decode :: decode(decoder) ?, inputs :
               :: bincode :: Decode :: decode(decoder) ?, click_inputs : ::
               bincode :: Decode :: decode(decoder) ?,
           })
    }
}