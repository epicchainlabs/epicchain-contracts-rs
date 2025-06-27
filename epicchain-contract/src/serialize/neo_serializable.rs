// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::builtin::{bytes::Bytes, string::ByteString, h160::H160, h256::H256, int256::Int256};

/// Trait for types that can be serialized to and from EpicChain's binary format
pub trait EpicChainSerializable: Sized {
    /// Serialize this value to bytes
    fn to_bytes(&self) -> Bytes;

    /// Deserialize from bytes
    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError>;

    /// Get the serialized size in bytes
    fn serialized_size(&self) -> usize {
        self.to_bytes().len()
    }
}

/// Errors that can occur during serialization/deserialization
#[derive(Debug, Clone)]
pub enum SerializationError {
    /// Not enough bytes to deserialize
    InsufficientData,
    /// Invalid data format
    InvalidFormat,
    /// Unsupported type
    UnsupportedType,
    /// Custom error message
    Custom(String),
}

impl core::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SerializationError::InsufficientData => write!(f, "Insufficient data for deserialization"),
            SerializationError::InvalidFormat => write!(f, "Invalid data format"),
            SerializationError::UnsupportedType => write!(f, "Unsupported type for serialization"),
            SerializationError::Custom(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

// Implement EpicChainSerializable for basic types

impl EpicChainSerializable for u8 {
    fn to_bytes(&self) -> Bytes {
        Bytes::from_slice(&[*self])
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        if bytes.len() < 1 {
            return Err(SerializationError::InsufficientData);
        }
        Ok(bytes[0])
    }
}

impl EpicChainSerializable for u16 {
    fn to_bytes(&self) -> Bytes {
        Bytes::from_slice(&self.to_le_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        if bytes.len() < 2 {
            return Err(SerializationError::InsufficientData);
        }
        let mut array = [0u8; 2];
        array.copy_from_slice(&bytes[0..2]);
        Ok(u16::from_le_bytes(array))
    }
}

impl EpicChainSerializable for u32 {
    fn to_bytes(&self) -> Bytes {
        Bytes::from_slice(&self.to_le_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        if bytes.len() < 4 {
            return Err(SerializationError::InsufficientData);
        }
        let mut array = [0u8; 4];
        array.copy_from_slice(&bytes[0..4]);
        Ok(u32::from_le_bytes(array))
    }
}

impl EpicChainSerializable for u64 {
    fn to_bytes(&self) -> Bytes {
        Bytes::from_slice(&self.to_le_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        if bytes.len() < 8 {
            return Err(SerializationError::InsufficientData);
        }
        let mut array = [0u8; 8];
        array.copy_from_slice(&bytes[0..8]);
        Ok(u64::from_le_bytes(array))
    }
}

impl EpicChainSerializable for i32 {
    fn to_bytes(&self) -> Bytes {
        Bytes::from_slice(&self.to_le_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        if bytes.len() < 4 {
            return Err(SerializationError::InsufficientData);
        }
        let mut array = [0u8; 4];
        array.copy_from_slice(&bytes[0..4]);
        Ok(i32::from_le_bytes(array))
    }
}

impl EpicChainSerializable for i64 {
    fn to_bytes(&self) -> Bytes {
        Bytes::from_slice(&self.to_le_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        if bytes.len() < 8 {
            return Err(SerializationError::InsufficientData);
        }
        let mut array = [0u8; 8];
        array.copy_from_slice(&bytes[0..8]);
        Ok(i64::from_le_bytes(array))
    }
}

impl EpicChainSerializable for bool {
    fn to_bytes(&self) -> Bytes {
        Bytes::from_slice(&[if *self { 1u8 } else { 0u8 }])
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        if bytes.len() < 1 {
            return Err(SerializationError::InsufficientData);
        }
        Ok(bytes[0] != 0)
    }
}

impl EpicChainSerializable for ByteString {
    fn to_bytes(&self) -> Bytes {
        // EpicChain format: length (varint) + data
        let data = self.as_bytes();
        let mut result = Vec::new();

        // Write length as varint
        write_varint(&mut result, data.len() as u64);

        // Write data
        result.extend_from_slice(data);

        Bytes::from_slice(&result)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        let mut offset = 0;

        // Read length
        let (length, varint_size) = read_varint(bytes)?;
        offset += varint_size;

        // Check if we have enough data
        if bytes.len() < offset + length as usize {
            return Err(SerializationError::InsufficientData);
        }

        // Read data
        let data = &bytes[offset..offset + length as usize];
        Ok(ByteString::from_slice(data))
    }
}

impl EpicChainSerializable for H160 {
    fn to_bytes(&self) -> Bytes {
        #[cfg(not(target_family = "wasm"))]
        {
            let bytes = H160::to_bytes(self);
            Bytes::from_slice(bytes.as_slice())
        }
        #[cfg(target_family = "wasm")]
        {
            // For WASM target, return placeholder
            Bytes::from_slice(&[])
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        if bytes.len() < 20 {
            return Err(SerializationError::InsufficientData);
        }
        #[cfg(not(target_family = "wasm"))]
        {
            Ok(H160::from_bytes(&bytes[0..20]))
        }
        #[cfg(target_family = "wasm")]
        {
            // For WASM target, return placeholder
            Ok(H160::default())
        }
    }
}

impl EpicChainSerializable for H256 {
    fn to_bytes(&self) -> Bytes {
        #[cfg(not(target_family = "wasm"))]
        {
            let bytes = H256::to_bytes(self);
            Bytes::from_slice(&bytes)
        }
        #[cfg(target_family = "wasm")]
        {
            // For WASM target, return placeholder
            Bytes::from_slice(&[])
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        if bytes.len() < 32 {
            return Err(SerializationError::InsufficientData);
        }
        #[cfg(not(target_family = "wasm"))]
        {
            Ok(H256::from_bytes(&bytes[0..32]))
        }
        #[cfg(target_family = "wasm")]
        {
            // For WASM target, return placeholder
            Ok(H256::default())
        }
    }
}

impl EpicChainSerializable for Int256 {
    fn to_bytes(&self) -> Bytes {
        #[cfg(not(target_family = "wasm"))]
        {
            let bytes = Int256::to_bytes(self);
            Bytes::from_slice(bytes.as_slice())
        }
        #[cfg(target_family = "wasm")]
        {
            // For WASM target, return placeholder
            Bytes::from_slice(&[])
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        if bytes.len() < 32 {
            return Err(SerializationError::InsufficientData);
        }
        #[cfg(not(target_family = "wasm"))]
        {
            Ok(Int256::from_bytes(&bytes[0..32]))
        }
        #[cfg(target_family = "wasm")]
        {
            // For WASM target, return placeholder
            Ok(Int256::default())
        }
    }
}

// Helper functions for varint encoding/decoding
fn write_varint(buffer: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        buffer.push((value & 0x7F) as u8 | 0x80);
        value >>= 7;
    }
    buffer.push(value as u8);
}

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
