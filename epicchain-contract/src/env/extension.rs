// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![allow(unused)]

#[cfg(target_family = "wasm")]
use crate::types::{placeholder::*, *};

#[link(wasm_import_module = "epicchain.extension")]
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
extern "C" {
    /// `h160_zero` returns a zero `H160`.
    pub(crate) fn h160_zero() -> H160;

    /// `h160_eq` checks if two `H160` are equal.
    pub(crate) fn h160_eq(a: H160, b: H160) -> bool;

    /// `h256_zero` returns a zero `H256`.
    pub(crate) fn h256_zero() -> H256;

    /// `h256_eq` checks if two `H256` are equal.
    pub(crate) fn h256_eq(a: H256, b: H256) -> bool;

    /// `h160_to_byte_string` converts a `H160` to a `ByteString`.
    pub(crate) fn h160_to_byte_string(src: H160) -> ByteString;

    /// `h256_to_byte_string` converts a `H256` to a `ByteString`.
    pub(crate) fn h256_to_byte_string(src: H256) -> ByteString;

    /// `h160_from_byte_string` converts a `ByteString` to a `H160`.
    pub(crate) fn h160_from_byte_string(src: ByteString) -> H160;

    /// `h256_from_byte_string` converts a `ByteString` to a `H256`.
    pub(crate) fn h256_from_byte_string(src: ByteString) -> H256;

    /// `concat_u8_byte_string` concatenates a `u8` prefix and a `ByteString`.
    pub(crate) fn concat_u8_byte_string(prefix: u8, src: ByteString) -> ByteString;

    /// `nullable_is_null` checks if a `Placeholder` is null.
    pub(crate) fn nullable_is_null(src: Placeholder) -> bool;

    /// `nullable_null` returns a null `Placeholder`.
    pub(crate) fn nullable_null() -> Placeholder;

    /// `int256_to_byte_string` converts a `Int256` to a `ByteString`.
    pub(crate) fn int256_to_byte_string(src: Int256) -> ByteString;

    /// `int256_from_byte_string` converts a `ByteString` to a `Int256`.
    pub(crate) fn int256_from_byte_string(src: ByteString) -> Int256;

    /// `int256_from_i64` converts an `i64` to a `Int256`.
    pub(crate) fn int256_from_i64(src: i64) -> Int256;
}
