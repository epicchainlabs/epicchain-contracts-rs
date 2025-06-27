// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod any;
pub mod array;
pub mod buffer;
pub mod h160;
pub mod h256;
pub mod int256;
pub mod interop;
pub mod map;
pub mod nullable;
pub mod primitive;
pub mod string;
pub mod structs;
pub mod bytes;

pub use {any::*, array::*, buffer::*, interop::*, map::*};
pub use {h160::*, h256::*, int256::*, nullable::*, string::*};

pub trait Builtin: inner::Sealed {}

pub trait Primitive: Builtin + Eq + PartialEq {}

// impl Primitive for a list of types
macro_rules! impl_primitive {
    ($($type:ty),*) => {
        $(impl Primitive for $type {})*

        $(impl Builtin for $type {})*

        $(impl inner::Sealed for $type {})*
    };
}

// impl_primitive!(i8, i16, u8, u16);
impl_primitive!(i32, i64, u32, u64, isize, usize);
impl_primitive!(bool, ByteString, Int256, H256, H160);

impl<T> Builtin for Array<T> {}
impl<T> inner::Sealed for Array<T> {}

impl Builtin for Buffer {}
impl inner::Sealed for Buffer {}

impl<K: Primitive + primitive::Primitive + std::hash::Hash + Eq + Clone, V: Clone> Builtin for Map<K, V> {}
impl<K: Primitive + primitive::Primitive + std::hash::Hash + Eq + Clone, V: Clone> inner::Sealed for Map<K, V> {}

pub(crate) mod inner {
    pub trait Sealed {}
}
