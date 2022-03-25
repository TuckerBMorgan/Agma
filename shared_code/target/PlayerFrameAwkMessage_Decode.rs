impl :: bincode :: Decode for PlayerFrameAwkMessage
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    { Ok(Self { frame_number : :: bincode :: Decode :: decode(decoder) ?, }) }
}