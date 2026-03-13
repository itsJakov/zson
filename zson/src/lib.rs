pub use zson_macros::{Decodable, Encodable};

mod value;
pub use value::*;

mod encode;
pub use encode::*;

mod decode;
pub use decode::*;

mod json;
pub use json::*;

mod cbor;
pub use cbor::*;

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
