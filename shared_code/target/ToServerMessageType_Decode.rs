impl :: bincode :: Decode for ToServerMessageType
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode > :: decode(decoder)
        ? ; match variant_index
        {
            1 => Ok(Self :: AwkFrameMessage { }), 2 =>
            Ok(Self :: InputMessage { }), variant =>
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
                {
                    found : variant, type_name : "ToServerMessageType",
                    allowed : :: bincode :: error :: AllowedEnumVariants ::
                    Allowed(& [1, 2])
                })
        }
    }
}