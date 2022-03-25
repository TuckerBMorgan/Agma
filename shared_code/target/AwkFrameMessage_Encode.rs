impl :: bincode :: Encode for AwkFrameMessage
{
    fn encode < E : :: bincode :: enc :: Encoder > (& self, encoder : & mut E)
    -> core :: result :: Result < (), :: bincode :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(& self.message_type, encoder) ? ; ::
        bincode :: Encode :: encode(& self.frame_number, encoder) ? ; Ok(())
    }
}