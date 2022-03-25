impl :: bincode :: Encode for Entity
{
    fn encode < E : :: bincode :: enc :: Encoder > (& self, encoder : & mut E)
    -> core :: result :: Result < (), :: bincode :: error :: EncodeError >
    { :: bincode :: Encode :: encode(& self.pos, encoder) ? ; Ok(()) }
}