// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{
    env,
    types::{placeholder::*, *},
};

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Buffer(Vec<u8>);

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Buffer(Placeholder);

pub type Bytes = Buffer;

#[cfg(target_family = "wasm")]
impl Buffer {
    #[inline(always)]
    pub fn new(size: usize) -> Self {
        unsafe { env::asm::buffer_with_size(size) }
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        unsafe { env::asm::buffer_size(Self(self.0)) }
    }
}

#[cfg(not(target_family = "wasm"))]
impl Buffer {
    pub fn new(size: usize) -> Self {
        Self(vec![0; size])
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(Buffer);
