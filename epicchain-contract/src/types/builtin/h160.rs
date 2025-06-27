// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{
    env,
    types::{placeholder::*, *},
};

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
#[derive(Debug, Default)]
pub struct H160([u8; 20]);

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct H160(Placeholder);

impl H160 {
    pub const SIZE: usize = 20;

    #[inline(always)]
    #[rustfmt::skip]
    pub fn zero() -> Self {
        #[cfg(target_family = "wasm")]
        unsafe { env::extension::h160_zero() }

        #[cfg(not(target_family = "wasm"))]
        H160([0u8; 20])
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn hex_encode(&self) -> ByteString {
        let mut buf = self.0.clone();
        buf.reverse();
        ByteString::new("0x".to_string() + &hex::encode(buf.as_slice()))
    }

    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn hex_decode(hex: &str) -> Self {
        let hex = if hex.starts_with("0x") || hex.starts_with("0X") {
            &hex[2..]
        } else {
            hex
        };
        let bytes = hex::decode(hex).unwrap();

        let mut buf = [0u8; 20];
        buf.copy_from_slice(&bytes);
        buf.reverse();
        H160(buf)
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut buf = [0u8; 20];
        buf.copy_from_slice(bytes);
        H160(buf)
    }
}

impl PartialEq for H160 {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn eq(&self, other: &Self) -> bool {
        unsafe { env::extension::h160_eq(*self, *other) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Clone for H160 {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Eq for H160 {}
impl Copy for H160 {}

#[cfg(target_family = "wasm")]
impl core::fmt::Debug for H160 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "H160(placeholder)")
    }
}

#[cfg(target_family = "wasm")]
impl Default for H160 {
    fn default() -> Self {
        Self(Placeholder::new(0))
    }
}

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(H160);

impl IntoByteString for H160 {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn into_byte_string(self) -> ByteString {
        unsafe { env::extension::h160_to_byte_string(self) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn into_byte_string(self) -> ByteString {
        ByteString::with_bytes(self.0.as_slice())
    }
}

impl FromByteString for H160 {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn from_byte_string(src: ByteString) -> Self {
        unsafe { env::extension::h160_from_byte_string(src) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn from_byte_string(src: ByteString) -> Self {
        let bytes = src.as_bytes();
        let mut buf = [0u8; 20];
        let len = core::cmp::min(bytes.len(), 20);
        buf[..len].copy_from_slice(&bytes[..len]);
        H160(buf)
    }
}
