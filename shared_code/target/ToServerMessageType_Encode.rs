impl :: bincode :: Encode for ToServerMessageType
{
    fn encode < E : :: bincode :: enc :: Encoder > (& self, encoder : & mut E)
    -> core :: result :: Result < (), :: bincode :: error :: EncodeError >
    {
        match self
        {
            Self :: AwkFrameMessage =>
            {
                < u32 as :: bincode :: Encode > :: encode(& (0u32), encoder) ?
                ; Ok(())
            }, Self :: InputMessage =>
            {
                < u32 as :: bincode :: Encode > :: encode(& (1u32), encoder) ?
                ; Ok(())
            },
        }
    }
}