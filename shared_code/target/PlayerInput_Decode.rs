impl :: bincode :: Decode for PlayerInput
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    { Ok(Self { input_values : :: bincode :: Decode :: decode(decoder) ?, }) }
}