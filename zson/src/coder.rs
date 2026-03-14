use crate::{Decodable, Encodable, Value};

pub trait Coder {
    type Type: ToOwned + ?Sized;

    fn encode<T: Encodable>(value: &T) -> <Self::Type as ToOwned>::Owned;
    fn decode<T: Decodable>(value: &Self::Type) -> Option<T>;

    fn convert<Dst: Coder>(input: &Self::Type) -> Option<<Dst::Type as ToOwned>::Owned> {
        let value = Self::decode::<Value>(input)?;
        Some(Dst::encode::<Value>(&value))
    }
}