// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::types::{placeholder::*, *};

/// Interop is an opaque type that can be used to EpicChain interop interface.
#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Interop(Placeholder);

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Interop(Placeholder);

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(Interop);
