// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![allow(unused)]

#[cfg(target_family = "wasm")]
use crate::types::*;

#[link(wasm_import_module = "epicchain.numeric")]
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
unsafe extern "C" {
    /// `int256_zero` returns the zero value of type Int256
    pub(crate) fn int256_zero() -> Int256;

    /// `int256_one` returns the one value of type Int256
    pub(crate) fn int256_one() -> Int256;

    /// `int256_minus_one` returns the negative one value of type Int256
    pub(crate) unsafe fn int256_minus_one() -> Int256;

    /// `int256_max` returns the maximum value of type Int256
    pub(crate) fn int256_max() -> Int256;

    /// `int256_min` returns the minimum value of type Int256
    pub(crate) fn int256_min() -> Int256;

    /// `int256_lt` returns true if a < b, false otherwise
    pub(crate) fn int256_lt(a: Int256, b: Int256) -> bool;

    /// `int256_eq` returns true if a == b, false otherwise
    pub(crate) fn int256_eq(a: Int256, b: Int256) -> bool;

    /// `int256_gt` returns true if a > b, false otherwise
    pub(crate) fn int256_gt(a: Int256, b: Int256) -> bool;

    /// `int256_ge` returns true if a >= b, false otherwise
    pub(crate) fn int256_ge(a: Int256, b: Int256) -> bool;

    /// `int256_le` returns true if a <= b, false otherwise
    pub(crate) fn int256_le(a: Int256, b: Int256) -> bool;

    /// `int256_ne` returns true if a != b, false otherwise
    pub(crate) fn int256_ne(a: Int256, b: Int256) -> bool;

    /// `int256_add` returns a + b
    pub(crate) fn int256_add(a: Int256, b: Int256) -> Int256;

    /// `int256_inc` returns a + 1
    pub(crate) fn int256_inc(a: Int256) -> Int256;

    /// `int256_sub` returns a - b
    pub(crate) fn int256_sub(a: Int256, b: Int256) -> Int256;

    /// `int256_dec` returns a - 1
    pub(crate) fn int256_dec(a: Int256) -> Int256;

    /// `int256_mul` returns a * b
    pub(crate) fn int256_mul(a: Int256, b: Int256) -> Int256;

    /// `int256_div` returns a / b
    pub(crate) fn int256_div(a: Int256, b: Int256) -> Int256;

    /// `int256_mod` returns a % b
    pub(crate) fn int256_mod(a: Int256, b: Int256) -> Int256;

    /// `int256_sign` returns the sign of a, 1 if positive, -1 if negative, 0 if zero
    pub(crate) fn int256_sign(a: Int256) -> Int256;

    /// `int256_is_zero` returns true if a is zero, false otherwise
    pub(crate) fn int256_is_zero(a: Int256) -> bool;

    /// `int256_is_one` returns true if a is one, false otherwise
    pub(crate) fn int256_is_one(a: Int256) -> bool;

    /// `int256_is_positive` returns true if a is positive, false otherwise
    pub(crate) fn int256_is_positive(a: Int256) -> bool;

    /// `int256_is_negative` returns true if a is negative, false otherwise
    pub(crate) fn int256_is_negative(a: Int256) -> bool;

    /// `int256_neg` returns -a
    pub(crate) fn int256_neg(a: Int256) -> Int256;

    /// `int256_abs` returns the absolute value of a
    pub(crate) fn int256_abs(a: Int256) -> Int256;

    /// `int256_pow` returns a ^ exponent
    pub(crate) fn int256_pow(a: Int256, exponent: u32) -> Int256;

    /// `int256_sqrt` returns the square root of a
    pub(crate) fn int256_sqrt(a: Int256) -> Int256;

    /// `int256_mulmod` returns (a * b) % modulus
    pub(crate) fn int256_mulmod(a: Int256, b: Int256, modulus: Int256) -> Int256;

    /// `int256_modpow` returns (a ^ exponent) % modulus
    pub(crate) fn int256_modpow(a: Int256, exponent: i32, modulus: Int256) -> Int256;

    /// `int256_shr` returns a >> shift
    pub(crate) fn int256_shr(a: Int256, shift: u32) -> Int256;

    /// `int256_shl` returns a << shift
    pub(crate) fn int256_shl(a: Int256, shift: u32) -> Int256;
}
