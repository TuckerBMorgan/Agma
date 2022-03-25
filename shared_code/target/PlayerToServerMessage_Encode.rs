impl :: bincode :: Encode for PlayerToServerMessage
{
    fn encode < E : :: bincode :: enc :: Encoder > (& self, encoder : & mut E)
    -> core :: result :: Result < (), :: bincode :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(& self.message_type, encoder) ? ; ::
        bincode :: Encode :: encode(& self.data, encoder) ? ; Ok(())
    }
}