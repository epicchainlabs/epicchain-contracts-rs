// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{
    env,
    types::{placeholder::*, *},
};

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Array<T> {
    value: Vec<T>,
    // _marker: core::marker::PhantomData<T>,
}

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Array<T> {
    value: Placeholder,
    _marker: core::marker::PhantomData<T>,
}

#[cfg(target_family = "wasm")]
impl<T: Default> Array<T> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            value: unsafe { env::asm::array_new() },
            _marker: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub fn from_items(items: &[T]) -> Self
    where
        T: IntoPlaceholder + Clone,
    {
        let array = Self {
            value: unsafe { env::asm::array_new() },
            _marker: core::marker::PhantomData,
        };

        // Populate the array with items
        for item in items {
            unsafe {
                env::asm::array_push(array.value, item.clone().into_placeholder())
            }
        }

        array
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        unsafe { env::asm::array_size(self.value) }
    }

    #[inline(always)]
    pub fn push(&mut self, value: T)
    where
        T: IntoPlaceholder,
    {
        unsafe {
            env::asm::array_push(self.value, value.into_placeholder())
        }
    }

    #[inline(always)]
    pub fn pop(&mut self) -> T
    where
        T: FromPlaceholder,
    {
        let placeholder = unsafe { env::asm::array_pop(self.value) };
        T::from_placeholder(placeholder)
    }

    /// Get an element at the specified index
    ///
    /// # Arguments
    /// * `index` - The index of the element to retrieve
    ///
    /// # Returns
    /// The element at the specified index
    pub fn get(&self, index: usize) -> T
    where
        T: FromPlaceholder,
    {
        let placeholder = unsafe { env::asm::array_get(self.value, index) };
        T::from_placeholder(placeholder)
    }

    #[inline(always)]
    pub fn set(&mut self, index: usize, value: T)
    where
        T: IntoPlaceholder,
    {
        unsafe {
            env::asm::array_set(self.value, index, value.into_placeholder())
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> Array<T> {
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }

    pub fn from_items(items: &[T]) -> Self
    where
        T: Clone,
    {
        Self { value: items.to_vec() }
    }

    pub fn size(&self) -> usize {
        self.value.len()
    }

    pub fn push(&mut self, value: T) {
        self.value.push(value);
    }

    pub fn pop(&mut self) -> T {
        self.value.pop().unwrap()
    }

    pub fn get(&self, index: usize) -> T
    where
        T: Clone,
    {
        self.value[index].clone() // Return a clone for consistency with WASM target
    }

    pub fn set(&mut self, index: usize, value: T) {
        self.value[index] = value;
    }
}

#[cfg(target_family = "wasm")]
impl<T: 'static> FromPlaceholder for Array<T> {
    #[inline(always)]
    fn from_placeholder(placeholder: Placeholder) -> Self {
        Self { value: placeholder, _marker: core::marker::PhantomData }
    }
}

#[cfg(target_family = "wasm")]
impl<T: 'static> IntoPlaceholder for Array<T> {
    #[inline(always)]
    fn into_placeholder(self) -> Placeholder {
        self.value
    }
}

impl<T: Default> Default for Array<T> {
    fn default() -> Self {
        Self::new()
    }
}
