// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::{
    builtin::{
        string::ByteString,
    },
};
use super::neo_serializable::{EpicChainSerializable, SerializationError};

/// Trait for types that can be deserialized
pub trait Deserialize: Sized {
    /// Deserialize from bytes
    fn deserialize(bytes: &[u8]) -> Result<Self, SerializationError>;
}

/// Deserialize a value from bytes using Neo's serialization format
pub fn deserialize<T: EpicChainSerializable>(bytes: &[u8]) -> Result<T, SerializationError> {
    T::from_bytes(bytes)
}

/// Deserialize a value from a ByteString
pub fn deserialize_from_bytestring<T: EpicChainSerializable>(bytestring: &ByteString) -> Result<T, SerializationError> {
    T::from_bytes(bytestring.as_bytes())
}

/// Deserialize multiple values from a single byte array
pub fn deserialize_multiple<T: EpicChainSerializable>(bytes: &[u8]) -> Result<Vec<T>, SerializationError> {
    let mut offset = 0;

    // Read count
    let (count, varint_size) = read_varint(&bytes[offset..])?;
    offset += varint_size;

    let mut result = Vec::with_capacity(count as usize);

    // Read each value
    for _ in 0..count {
        let value = T::from_bytes(&bytes[offset..])?;
        let value_size = value.serialized_size();
        offset += value_size;
        result.push(value);
    }

    Ok(result)
}

/// Deserialize a vector of values
pub fn deserialize_vec<T: EpicChainSerializable>(bytes: &[u8]) -> Result<Vec<T>, SerializationError> {
    deserialize_multiple(bytes)
}

/// Deserialize an optional value
pub fn deserialize_option<T: EpicChainSerializable>(bytes: &[u8]) -> Result<Option<T>, SerializationError> {
    if bytes.is_empty() {
        return Err(SerializationError::InsufficientData);
    }

    let marker = bytes[0];
    match marker {
        0 => Ok(None),
        1 => {
            if bytes.len() < 2 {
                return Err(SerializationError::InsufficientData);
            }
            let value = T::from_bytes(&bytes[1..])?;
            Ok(Some(value))
        }
        _ => Err(SerializationError::InvalidFormat),
    }
}

/// Deserialize a tuple of two values
pub fn deserialize_tuple2<T1: EpicChainSerializable, T2: EpicChainSerializable>(
    bytes: &[u8]
) -> Result<(T1, T2), SerializationError> {
    let value1 = T1::from_bytes(bytes)?;
    let offset = value1.serialized_size();

    if bytes.len() < offset {
        return Err(SerializationError::InsufficientData);
    }

    let value2 = T2::from_bytes(&bytes[offset..])?;

    Ok((value1, value2))
}

/// Deserialize a tuple of three values
pub fn deserialize_tuple3<T1: EpicChainSerializable, T2: EpicChainSerializable, T3: EpicChainSerializable>(
    bytes: &[u8]
) -> Result<(T1, T2, T3), SerializationError> {
    let value1 = T1::from_bytes(bytes)?;
    let offset1 = value1.serialized_size();

    if bytes.len() < offset1 {
        return Err(SerializationError::InsufficientData);
    }

    let value2 = T2::from_bytes(&bytes[offset1..])?;
    let offset2 = offset1 + value2.serialized_size();

    if bytes.len() < offset2 {
        return Err(SerializationError::InsufficientData);
    }

    let value3 = T3::from_bytes(&bytes[offset2..])?;

    Ok((value1, value2, value3))
}

// Implement Deserialize for basic types
impl Deserialize for u8 {
    fn deserialize(bytes: &[u8]) -> Result<Self, SerializationError> {
        Self::from_bytes(bytes)
    }
}

impl Deserialize for u16 {
    fn deserialize(bytes: &[u8]) -> Result<Self, SerializationError> {
        Self::from_bytes(bytes)
    }
}

impl Deserialize for u32 {
    fn deserialize(bytes: &[u8]) -> Result<Self, SerializationError> {
        Self::from_bytes(bytes)
    }
}

impl Deserialize for u64 {
    fn deserialize(bytes: &[u8]) -> Result<Self, SerializationError> {
        Self::from_bytes(bytes)
    }
}

impl Deserialize for i32 {
    fn deserialize(bytes: &[u8]) -> Result<Self, SerializationError> {
        Self::from_bytes(bytes)
    }
}

impl Deserialize for i64 {
    fn deserialize(bytes: &[u8]) -> Result<Self, SerializationError> {
        Self::from_bytes(bytes)
    }
}

impl Deserialize for bool {
    fn deserialize(bytes: &[u8]) -> Result<Self, SerializationError> {
        Self::from_bytes(bytes)
    }
}

impl Deserialize for ByteString {
    fn deserialize(bytes: &[u8]) -> Result<Self, SerializationError> {
        Ok(Self::from_bytes(bytes))
    }
}

impl<T: EpicChainSerializable> Deserialize for Vec<T> {
    fn deserialize(bytes: &[u8]) -> Result<Self, SerializationError> {
        deserialize_vec(bytes)
    }
}

impl<T: EpicChainSerializable> Deserialize for Option<T> {
    fn deserialize(bytes: &[u8]) -> Result<Self, SerializationError> {
        deserialize_option(bytes)
    }
}

// Helper function for varint decoding
fn read_varint(bytes: &[u8]) -> Result<(u64, usize), SerializationError> {
    let mut result = 0u64;
    let mut shift = 0;
    let mut offset = 0;

    loop {
        if offset >= bytes.len() {
            return Err(SerializationError::InsufficientData);
        }

        let byte = bytes[offset];
        offset += 1;

        result |= ((byte & 0x7F) as u64) << shift;

        if byte & 0x80 == 0 {
            break;
        }

        shift += 7;
        if shift >= 64 {
            return Err(SerializationError::InvalidFormat);
        }
    }

    Ok((result, offset))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize::serialize::serialize;

    #[test]
    fn test_deserialize_u32() {
        let value = 42u32;
        let serialized = serialize(&value).unwrap();
        let deserialized: u32 = deserialize(serialized.as_slice()).unwrap();
        assert_eq!(value, deserialized);
    }

    #[test]
    fn test_deserialize_bool() {
        let value_true = true;
        let value_false = false;

        let serialized_true = serialize(&value_true).unwrap();
        let serialized_false = serialize(&value_false).unwrap();

        let deserialized_true: bool = deserialize(serialized_true.as_slice()).unwrap();
        let deserialized_false: bool = deserialize(serialized_false.as_slice()).unwrap();

        assert_eq!(value_true, deserialized_true);
        assert_eq!(value_false, deserialized_false);
    }

    #[test]
    fn test_deserialize_bytestring() {
        let value = ByteString::from_literal("hello");
        let serialized = serialize(&value).unwrap();
        let deserialized: ByteString = deserialize(serialized.as_slice()).unwrap();

        assert_eq!(value.as_bytes(), deserialized.as_bytes());
    }

    #[test]
    fn test_round_trip_serialization() {
        let original = 12345u64;
        let serialized = serialize(&original).unwrap();
        let deserialized: u64 = deserialize(serialized.as_slice()).unwrap();
        assert_eq!(original, deserialized);
    }
}
