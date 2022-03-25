impl :: bincode :: Encode for Matrix3x3
{
    fn encode < E : :: bincode :: enc :: Encoder > (& self, encoder : & mut E)
    -> core :: result :: Result < (), :: bincode :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(& self.m11, encoder) ? ; :: bincode ::
        Encode :: encode(& self.m12, encoder) ? ; :: bincode :: Encode ::
        encode(& self.m13, encoder) ? ; :: bincode :: Encode ::
        encode(& self.m21, encoder) ? ; :: bincode :: Encode ::
        encode(& self.m22, encoder) ? ; :: bincode :: Encode ::
        encode(& self.m23, encoder) ? ; :: bincode :: Encode ::
        encode(& self.m31, encoder) ? ; :: bincode :: Encode ::
        encode(& self.m32, encoder) ? ; :: bincode :: Encode ::
        encode(& self.m33, encoder) ? ; Ok(())
    }
}