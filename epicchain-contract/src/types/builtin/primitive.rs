// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

/// Marker trait for primitive types that can be used as keys in a Map.
pub trait Primitive: Clone + PartialEq + Eq + PartialOrd + Ord + 'static {}

impl Primitive for bool {}
impl Primitive for u8 {}
impl Primitive for u16 {}
impl Primitive for u32 {}
impl Primitive for u64 {}
impl Primitive for u128 {}
impl Primitive for i8 {}
impl Primitive for i16 {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for i128 {}
impl Primitive for char {}
impl Primitive for String {}
impl<'a, T: Primitive + 'static> Primitive for &'a T where &'a T: 'static {}
