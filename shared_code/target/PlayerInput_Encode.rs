impl :: bincode :: Encode for PlayerInput
{
    fn encode < E : :: bincode :: enc :: Encoder > (& self, encoder : & mut E)
    -> core :: result :: Result < (), :: bincode :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(& self.input_values, encoder) ? ;
        Ok(())
    }
}