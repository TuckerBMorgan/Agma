impl :: bincode :: Encode for PlayerToServerMessage
{
    fn encode < E : :: bincode :: enc :: Encoder > (& self, encoder : & mut E)
    -> core :: result :: Result < (), :: bincode :: error :: EncodeError >
    {
        match self
        {
            Self :: KeyboardAction =>
            {
                < u32 as :: bincode :: Encode > :: encode(& (0u32), encoder) ?
                ; Ok(())
            }, Self :: MouseAction =>
            {
                < u32 as :: bincode :: Encode > :: encode(& (1u32), encoder) ?
                ; Ok(())
            }, Self :: AwkFrameMessage =>
            {
                < u32 as :: bincode :: Encode > :: encode(& (2u32), encoder) ?
                ; Ok(())
            },
        }
    }
}