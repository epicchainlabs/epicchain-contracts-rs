// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::builtin::{bytes::Bytes, string::ByteString};
use super::neo_serializable::{EpicChainSerializable, SerializationError};

/// Trait for types that can be serialized
pub trait Serialize {
    /// Serialize this value to bytes
    fn serialize(&self) -> Result<Bytes, SerializationError>;
}

/// Serialize a value to bytes using Neo's serialization format
pub fn serialize<T: EpicChainSerializable>(value: &T) -> Result<Bytes, SerializationError> {
    Ok(value.to_bytes())
}

/// Serialize a value to a ByteString
pub fn serialize_to_bytestring<T: EpicChainSerializable>(value: &T) -> Result<ByteString, SerializationError> {
    let bytes = value.to_bytes();
    Ok(ByteString::from_slice(bytes.as_slice()))
}

/// Serialize multiple values into a single byte array
pub fn serialize_multiple<T: EpicChainSerializable>(values: &[T]) -> Result<Bytes, SerializationError> {
    let mut result = Vec::new();

    // Write count as varint
    write_varint(&mut result, values.len() as u64);

    // Write each value
    for value in values {
        let serialized = value.to_bytes();
        result.extend_from_slice(serialized.as_slice());
    }

    Ok(Bytes::from_slice(&result))
}

/// Serialize a vector of values
pub fn serialize_vec<T: EpicChainSerializable>(values: &Vec<T>) -> Result<Bytes, SerializationError> {
    serialize_multiple(values.as_slice())
}

/// Serialize an optional value
pub fn serialize_option<T: EpicChainSerializable>(value: &Option<T>) -> Result<Bytes, SerializationError> {
    let mut result = Vec::new();

    match value {
        Some(v) => {
            result.push(1u8); // Present marker
            let serialized = v.to_bytes();
            result.extend_from_slice(serialized.as_slice());
        }
        None => {
            result.push(0u8); // Absent marker
        }
    }

    Ok(Bytes::from_slice(&result))
}

/// Serialize a tuple of two values
pub fn serialize_tuple2<T1: EpicChainSerializable, T2: EpicChainSerializable>(
    value1: &T1,
    value2: &T2
) -> Result<Bytes, SerializationError> {
    let mut result = Vec::new();

    let serialized1 = value1.to_bytes();
    let serialized2 = value2.to_bytes();

    result.extend_from_slice(serialized1.as_slice());
    result.extend_from_slice(serialized2.as_slice());

    Ok(Bytes::from_slice(&result))
}

/// Serialize a tuple of three values
pub fn serialize_tuple3<T1: EpicChainSerializable, T2: EpicChainSerializable, T3: EpicChainSerializable>(
    value1: &T1,
    value2: &T2,
    value3: &T3
) -> Result<Bytes, SerializationError> {
    let mut result = Vec::new();

    let serialized1 = value1.to_bytes();
    let serialized2 = value2.to_bytes();
    let serialized3 = value3.to_bytes();

    result.extend_from_slice(serialized1.as_slice());
    result.extend_from_slice(serialized2.as_slice());
    result.extend_from_slice(serialized3.as_slice());

    Ok(Bytes::from_slice(&result))
}

// Implement Serialize for basic types
impl Serialize for u8 {
    fn serialize(&self) -> Result<Bytes, SerializationError> {
        Ok(self.to_bytes())
    }
}

impl Serialize for u16 {
    fn serialize(&self) -> Result<Bytes, SerializationError> {
        Ok(self.to_bytes())
    }
}

impl Serialize for u32 {
    fn serialize(&self) -> Result<Bytes, SerializationError> {
        Ok(self.to_bytes())
    }
}

impl Serialize for u64 {
    fn serialize(&self) -> Result<Bytes, SerializationError> {
        Ok(self.to_bytes())
    }
}

impl Serialize for i32 {
    fn serialize(&self) -> Result<Bytes, SerializationError> {
        Ok(self.to_bytes())
    }
}

impl Serialize for i64 {
    fn serialize(&self) -> Result<Bytes, SerializationError> {
        Ok(self.to_bytes())
    }
}

impl Serialize for bool {
    fn serialize(&self) -> Result<Bytes, SerializationError> {
        Ok(self.to_bytes())
    }
}

impl Serialize for ByteString {
    fn serialize(&self) -> Result<Bytes, SerializationError> {
        Ok(Bytes::from_slice(&self.to_bytes()))
    }
}

impl<T: EpicChainSerializable> Serialize for Vec<T> {
    fn serialize(&self) -> Result<Bytes, SerializationError> {
        serialize_vec(self)
    }
}

impl<T: EpicChainSerializable> Serialize for Option<T> {
    fn serialize(&self) -> Result<Bytes, SerializationError> {
        serialize_option(self)
    }
}

// Helper function for varint encoding
fn write_varint(buffer: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        buffer.push((value & 0x7F) as u8 | 0x80);
        value >>= 7;
    }
    buffer.push(value as u8);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::builtin::h160::H160;

    #[test]
    fn test_serialize_u32() {
        let value = 42u32;
        let serialized = serialize(&value).unwrap();
        assert_eq!(serialized.as_slice(), &[42, 0, 0, 0]);
    }

    #[test]
    fn test_serialize_bool() {
        let value_true = true;
        let value_false = false;

        let serialized_true = serialize(&value_true).unwrap();
        let serialized_false = serialize(&value_false).unwrap();

        assert_eq!(serialized_true.as_slice(), &[1]);
        assert_eq!(serialized_false.as_slice(), &[0]);
    }

    #[test]
    fn test_serialize_bytestring() {
        let value = ByteString::from_literal("hello");
        let serialized = serialize(&value).unwrap();

        // Should contain length (5) as varint + "hello"
        assert_eq!(serialized.as_slice()[0], 5); // length
        assert_eq!(&serialized.as_slice()[1..], b"hello");
    }

    #[test]
    fn test_serialize_option() {
        let some_value = Some(42u32);
        let none_value: Option<u32> = None;

        let serialized_some = serialize_option(&some_value).unwrap();
        let serialized_none = serialize_option(&none_value).unwrap();

        assert_eq!(serialized_some.as_slice()[0], 1); // Present marker
        assert_eq!(serialized_none.as_slice(), &[0]); // Absent marker
    }

    #[test]
    fn test_serialize_vec() {
        let values = vec![1u32, 2u32, 3u32];
        let serialized = serialize_vec(&values).unwrap();

        // Should start with count (3) as varint
        assert_eq!(serialized.as_slice()[0], 3);
    }
}
