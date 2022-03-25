impl :: bincode :: Decode for ToPlayerMessageType
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode > :: decode(decoder)
        ? ; match variant_index
        {
            0u32 => Ok(Self :: UpdateWorld { }), 1u32 =>
            Ok(Self :: StateWorld { }), variant =>
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
                {
                    found : variant, type_name : "ToPlayerMessageType",
                    allowed : :: bincode :: error :: AllowedEnumVariants ::
                    Range { min : 0, max : 1 }
                })
        }
    }
}