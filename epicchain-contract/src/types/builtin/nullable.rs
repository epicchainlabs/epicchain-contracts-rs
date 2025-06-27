// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::{placeholder::*, *}};

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Nullable<T> {
    value: Placeholder,
    _marker: core::marker::PhantomData<T>,
}

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Nullable<T> {
    value: Option<T>,
}

#[cfg(target_family = "wasm")]
#[allow(private_bounds)]
impl<T: FromPlaceholder + IntoPlaceholder> Nullable<T> {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn new(value: T) -> Self {
        Self { 
            value: value.into_placeholder(),
            _marker: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn null() -> Self {
        Self { 
            value: unsafe { env::extension::nullable_null() },
            _marker: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn is_null(&self) -> bool {
        unsafe { env::extension::nullable_is_null(self.value) }
    }

    #[inline(always)]
    pub fn unwrap_or(self, default: T) -> T {
        if self.is_null() {
            default
        } else {
            T::from_placeholder(self.value)
        }
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn unwrap(self) -> T {
        if self.is_null() {
            unsafe { env::asm::abort() }
        }
        T::from_placeholder(self.value)
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> Nullable<T> {
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }

    #[inline(always)]
    pub fn null() -> Self {
        Self { value: None }
    }

    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.value.is_none()
    }

    #[inline(always)]
    pub fn unwrap_or(self, default: T) -> T {
        self.value.unwrap_or(default)
    }

    #[inline(always)]
    pub fn unwrap(self) -> T {
        self.value.unwrap()
    }
}

#[cfg(target_family = "wasm")]
impl<T: FromPlaceholder + IntoPlaceholder> Default for Nullable<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::null()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> Default for Nullable<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::null()
    }
}
