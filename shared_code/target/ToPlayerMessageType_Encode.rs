impl :: bincode :: Encode for ToPlayerMessageType
{
    fn encode < E : :: bincode :: enc :: Encoder > (& self, encoder : & mut E)
    -> core :: result :: Result < (), :: bincode :: error :: EncodeError >
    {
        match self
        {
            Self :: UpdateWorld =>
            {
                < u32 as :: bincode :: Encode > :: encode(& (0u32), encoder) ?
                ; Ok(())
            }, Self :: StateWorld =>
            {
                < u32 as :: bincode :: Encode > :: encode(& (1u32), encoder) ?
                ; Ok(())
            },
        }
    }
}