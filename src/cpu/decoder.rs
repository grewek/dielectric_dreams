pub trait Decoder {
    type DecodedItem;
    fn decode(pattern: u32) -> Self::DecodedItem;
}
