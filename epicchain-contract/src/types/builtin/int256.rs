// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{
    env,
    types::{placeholder::*, *},
};

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
#[derive(Debug, PartialOrd, Ord)]
pub struct Int256(num256::Int256);

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Int256(Placeholder);

impl Int256 {
    pub const SIZE: usize = 32;
}

#[cfg(target_family = "wasm")]
impl Int256 {
    #[inline(always)]
    pub fn new(n: i64) -> Self {
        unsafe { env::extension::int256_from_i64(n) }
    }

    #[inline(always)]
    pub fn from_u64(n: u64) -> Self {
        // For WASM target, avoid u64 operations that generate I32WrapI64
        // Use the existing new() method with i32 conversion
        Self::new(n as i32 as i64)
    }

    #[inline(always)]
    pub fn zero() -> Self {
        unsafe { env::numeric::int256_zero() }
    }

    #[inline(always)]
    pub fn one() -> Self {
        unsafe { env::numeric::int256_one() }
    }

    #[inline(always)]
    pub fn minus_one() -> Self {
        unsafe { env::numeric::int256_minus_one() }
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        unsafe { env::numeric::int256_is_zero(Self(self.0)) }
    }

    #[inline(always)]
    pub fn is_one(&self) -> bool {
        unsafe { env::numeric::int256_is_one(Self(self.0)) }
    }

    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        unsafe { env::numeric::int256_is_positive(Self(self.0)) }
    }

    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        unsafe { env::numeric::int256_is_negative(Self(self.0)) }
    }

    /// Returns the result of adding two `Int256` values.
    /// It will abort if the addition overflows.
    #[inline(always)]
    pub fn checked_add(&self, other: &Self) -> Self {
        unsafe { env::numeric::int256_add(Self(self.0), Self(other.0)) }
    }

    /// Returns the result of incrementing a `Int256` value.
    /// It will abort if the increment overflows.
    #[inline(always)]
    pub fn checked_inc(&self) -> Self {
        unsafe { env::numeric::int256_inc(Self(self.0)) }
    }

    /// Returns the result of subtracting two `Int256` values.
    /// It will abort if the subtraction underflows.
    #[inline(always)]
    pub fn checked_sub(&self, other: &Self) -> Self {
        unsafe { env::numeric::int256_sub(Self(self.0), Self(other.0)) }
    }

    /// Returns the result of decrementing a `Int256` value.
    /// It will abort if the decrement underflows.
    #[inline(always)]
    pub fn checked_dec(&self) -> Self {
        unsafe { env::numeric::int256_dec(Self(self.0)) }
    }

    /// Returns the result of multiplying two `Int256` values.
    /// It will abort if the multiplication overflows.
    #[inline(always)]
    pub fn checked_mul(&self, other: &Self) -> Self {
        unsafe { env::numeric::int256_mul(Self(self.0), Self(other.0)) }
    }

    /// Returns the result of dividing two `Int256` values.
    /// It will abort if the division underflows.
    #[inline(always)]
    pub fn checked_div(&self, other: &Self) -> Self {
        unsafe { env::numeric::int256_div(Self(self.0), Self(other.0)) }
    }

    /// Returns the result of moduloing two `Int256` values.
    /// It will abort if the modulo is zero.
    #[inline(always)]
    pub fn checked_mod(&self, other: &Self) -> Self {
        unsafe { env::numeric::int256_mod(Self(self.0), Self(other.0)) }
    }

    /// Returns the result of negating a `Int256` value.
    /// It will abort if the negation underflows.
    #[inline(always)]
    pub fn checked_neg(&self) -> Self {
        unsafe { env::numeric::int256_neg(Self(self.0)) }
    }

    /// Returns the result of taking the absolute value of a `Int256` value.
    /// It will abort if the absolute value underflows.
    #[inline(always)]
    pub fn checked_abs(&self) -> Self {
        unsafe { env::numeric::int256_abs(Self(self.0)) }
    }

    /// Returns the result of raising a `Int256` value to the power of an exponent.
    /// It will abort if the exponent is negative.
    #[inline(always)]
    pub fn checked_pow(&self, exponent: u32) -> Self {
        unsafe { env::numeric::int256_pow(Self(self.0), exponent) }
    }

    /// Returns the result of taking the square root of a `Int256` value.
    /// It will abort if the square root is not an integer.
    #[inline(always)]
    pub fn checked_sqrt(&self) -> Self {
        unsafe { env::numeric::int256_sqrt(Self(self.0)) }
    }

    /// Returns the result of (a * b) % modulus.
    /// It will abort if the modulus is zero.
    #[inline(always)]
    pub fn checked_mulmod(&self, other: &Self, modulus: &Self) -> Self {
        unsafe { env::numeric::int256_mulmod(Self(self.0), Self(other.0), Self(modulus.0)) }
    }

    /// Returns the result of taking the (a ^ exponent) % modulus of a `Int256` value.
    /// It will abort if the exponent is negative and is not -1.
    #[inline(always)]
    pub fn checked_modpow(&self, exponent: i32, modulus: &Self) -> Self {
        unsafe { env::numeric::int256_modpow(Self(self.0), exponent, Self(modulus.0)) }
    }

    /// Returns the result of shifting a `Int256` value to the right by a specified number of bits.
    #[inline(always)]
    pub fn checked_shr(&self, shift: u32) -> Self {
        unsafe { env::numeric::int256_shr(Self(self.0), shift) }
    }

    /// Returns the result of shifting a `Int256` value to the left by a specified number of bits.
    #[inline(always)]
    pub fn checked_shl(&self, shift: u32) -> Self {
        unsafe { env::numeric::int256_shl(Self(self.0), shift) }
    }
}

#[cfg(not(target_family = "wasm"))]
impl Int256 {
    pub fn new(n: i64) -> Self {
        Int256(num256::Int256::from(n))
    }

    pub fn from_u64(n: u64) -> Self {
        Int256(num256::Int256::from(n))
    }

    pub fn zero() -> Self {
        Int256(num256::Int256::from(0))
    }

    pub fn one() -> Self {
        Int256(num256::Int256::from(1))
    }

    pub fn minus_one() -> Self {
        Int256(num256::Int256::from(-1))
    }

    pub fn is_zero(&self) -> bool {
        self.0 == num256::Int256::from(0)
    }

    pub fn is_positive(&self) -> bool {
        self.0 > num256::Int256::from(0)
    }

    pub fn is_negative(&self) -> bool {
        self.0 < num256::Int256::from(0)
    }

    pub fn checked_add(&self, other: &Self) -> Self {
        use num_traits::CheckedAdd;
        Self(self.0.checked_add(&other.0).unwrap())
    }

    pub fn checked_inc(&self) -> Self {
        use num_traits::CheckedAdd;
        Self(self.0.checked_add(&num256::Int256::from(1)).unwrap())
    }

    pub fn checked_sub(&self, other: &Self) -> Self {
        use num_traits::CheckedSub;
        Self(self.0.checked_sub(&other.0).unwrap())
    }

    pub fn checked_dec(&self) -> Self {
        use num_traits::CheckedSub;
        Self(self.0.checked_sub(&num256::Int256::from(1)).unwrap())
    }

    pub fn checked_mul(&self, other: &Self) -> Self {
        use num_traits::CheckedMul;
        Self(self.0.checked_mul(&other.0).unwrap())
    }

    pub fn checked_div(&self, other: &Self) -> Self {
        use num_traits::CheckedDiv;
        Self(self.0.checked_div(&other.0).unwrap())
    }

    pub fn checked_mod(&self, other: &Self) -> Self {
        if other.is_zero() {
            panic!("Int256::checked_mod: modulus is zero");
        }

        use num_traits::Bounded;
        if self.0 == num256::Int256::min_value() && other.eq(&Self::minus_one()) {
            panic!("Int256::checked_mod: overflow");
        }

        Self(self.0 % other.0)
    }

    pub fn checked_neg(&self) -> Self {
        use num_traits::Bounded;
        if self.0 == num256::Int256::min_value() {
            panic!("Int256::checked_neg: overflow");
        }

        Self(-self.0)
    }

    pub fn checked_abs(&self) -> Self {
        use num_traits::{Bounded, Signed};
        if self.0 == num256::Int256::min_value() {
            panic!("Int256::checked_abs: overflow");
        }
        Self(self.0.abs())
    }

    pub fn checked_pow(&self, exponent: u32) -> Self {
        if exponent == 0 {
            return Self::one();
        }

        let mut pow = self.clone();
        for _ in 1..exponent {
            pow = pow.checked_mul(self);
        }
        pow
    }

    pub fn checked_sqrt(&self) -> Self {
        if self.0 < num256::Int256::from(0) {
            panic!("Int256::checked_sqrt: negative value");
        }

        Self(self.0.sqrt().to_int256().unwrap())
    }

    // pub fn checked_mulmod(&self, other: &Self, modulus: &Self) -> Self {}
    //
    // pub fn checked_modpow(&self, exponent: i32, modulus: &Self) -> Self {}

    pub fn checked_shr(&self, shift: u32) -> Self {
        if shift > 256 {
            panic!("Int256::checked_shr: shift is too large");
        }

        let shifted = num256::Uint256::from_le_bytes(&self.0.to_le_bytes()) >> shift.into();
        let mut shifted = shifted.to_le_bytes();
        if !self.is_negative() {
            return Self(num256::Int256::from_le_bytes(&shifted));
        }

        use num_traits::Bounded;
        let mask = (num256::Uint256::max_value() << (256 - shift).into()).to_le_bytes();
        for i in 0..shifted.len() {
            shifted[i] &= mask[i];
        }
        Self(num256::Int256::from_le_bytes(&shifted))
    }

    pub fn checked_shl(&self, shift: u32) -> Self {
        if shift > 256 {
            panic!("Int256::checked_shl: shift is too large");
        }

        let unsigned = num256::Uint256::from_le_bytes(&self.0.to_le_bytes());
        Self(num256::Int256::from_le_bytes(&(unsigned << shift.into()).to_le_bytes()))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(num256::Int256::from_le_bytes(bytes))
    }
}


impl Default for Int256 {
    #[inline(always)]
    fn default() -> Self {
        Self::zero()
    }
}

impl PartialEq for Int256 {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn eq(&self, other: &Self) -> bool {
        unsafe { env::numeric::int256_eq(Self(self.0), Self(other.0)) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Clone for Int256 {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Eq for Int256 {}
impl Copy for Int256 {}

#[cfg(target_family = "wasm")]
impl core::fmt::Debug for Int256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Int256(placeholder)")
    }
}

#[cfg(target_family = "wasm")]
impl PartialOrd for Int256 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(target_family = "wasm")]
impl Ord for Int256 {
    fn cmp(&self, _other: &Self) -> core::cmp::Ordering {
        // For WASM target, we can't actually compare placeholders
        // This is a placeholder implementation
        core::cmp::Ordering::Equal
    }
}

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(Int256);

impl IntoByteString for Int256 {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn into_byte_string(self) -> ByteString {
        unsafe { env::extension::int256_to_byte_string(self) }
    }

    #[inline(always)]
    #[cfg(not(target_family = "wasm"))]
    fn into_byte_string(self) -> ByteString {
        let data = self.0.to_le_bytes();
        let index = data.iter().take_while(|&&x| x == 0).count();
        ByteString::with_bytes(if index == data.len() { &[0] } else { &data[index..] })
    }
}

impl FromByteString for Int256 {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn from_byte_string(src: ByteString) -> Self {
        unsafe { env::extension::int256_from_byte_string(src) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn from_byte_string(src: ByteString) -> Self {
        if src.len() > Int256::SIZE {
            panic!("Int256::from_byte_string: source string is too long");
        }
        Int256(num256::Int256::from_le_bytes(src.as_bytes()))
    }
}
