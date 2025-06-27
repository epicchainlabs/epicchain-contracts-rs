// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
#[derive(Debug)]
pub struct PublicKey(ByteString);

impl PublicKey {
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        // A valid public key should be either 33 bytes (compressed) or 65 bytes (uncompressed)
        let bytes = self.0.to_bytes();
        match bytes.len() {
            33 => {
                // Compressed public key: first byte should be 0x02 or 0x03
                bytes[0] == 0x02 || bytes[0] == 0x03
            }
            65 => {
                // Uncompressed public key: first byte should be 0x04
                bytes[0] == 0x04
            }
            _ => false,
        }
    }

    /// Creates a PublicKey from a byte slice
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(ByteString::from_bytes(bytes))
    }

    /// Returns the underlying bytes as a vector
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl Default for PublicKey {
    fn default() -> Self {
        Self(ByteString::from_bytes(&[0u8; 33]))
    }
}

impl Clone for PublicKey {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Eq for PublicKey {}

impl PartialEq for PublicKey {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
