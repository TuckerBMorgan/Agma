impl :: bincode :: Encode for World
{
    fn encode < E : :: bincode :: enc :: Encoder > (& self, encoder : & mut E)
    -> core :: result :: Result < (), :: bincode :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(& self.frame_number, encoder) ? ; ::
        bincode :: Encode :: encode(& self.entities, encoder) ? ; :: bincode
        :: Encode :: encode(& self.transforms, encoder) ? ; :: bincode ::
        Encode :: encode(& self.input, encoder) ? ; Ok(())
    }
}