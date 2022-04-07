impl :: bincode :: Decode for PlayerToServerMessage
{
    fn decode < D : :: bincode :: de :: Decoder > (decoder : & mut D) -> core
    :: result :: Result < Self, :: bincode :: error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode > :: decode(decoder)
        ? ; match variant_index
        {
            0u32 => Ok(Self :: KeyboardAction { }), 1u32 =>
            Ok(Self :: MouseAction { }), 2u32 =>
            Ok(Self :: AwkFrameMessage { }), variant =>
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
                {
                    found : variant, type_name : "PlayerToServerMessage",
                    allowed : :: bincode :: error :: AllowedEnumVariants ::
                    Range { min : 0, max : 2 }
                })
        }
    }
}