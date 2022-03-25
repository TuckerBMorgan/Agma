impl :: bincode :: Encode for Vec3
{
    fn encode < E : :: bincode :: enc :: Encoder > (& self, encoder : & mut E)
    -> core :: result :: Result < (), :: bincode :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(& self.x, encoder) ? ; :: bincode ::
        Encode :: encode(& self.y, encoder) ? ; :: bincode :: Encode ::
        encode(& self.z, encoder) ? ; Ok(())
    }
}