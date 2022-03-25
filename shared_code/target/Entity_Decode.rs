impl :: bincode :: Decode for Entity
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    { Ok(Self { pos : :: bincode :: Decode :: decode(decoder) ?, }) }
}