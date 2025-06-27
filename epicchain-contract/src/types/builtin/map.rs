// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::{placeholder::*, *}};

#[cfg(not(target_family = "wasm"))]
use std::collections::HashMap;

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
#[derive(Clone)]
pub struct Map<K: Primitive + std::hash::Hash + Eq + Clone, V: Clone> {
    value: HashMap<K, V>,
}

#[cfg(target_family = "wasm")]
#[repr(C)]
#[derive(Clone)]
pub struct Map<K: Primitive + Clone, V: Clone> {
    value: Placeholder,
    _marker: core::marker::PhantomData<(K, V)>,
}

#[cfg(target_family = "wasm")]
impl<K: Primitive + IntoPlaceholder + Clone, V: IntoPlaceholder + FromPlaceholder + Clone + 'static> Map<K, V> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            value: unsafe { env::asm::map_new() },
            _marker: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        unsafe { env::asm::map_size(self.value) }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    #[inline(always)]
    pub fn put(&mut self, key: K, value: V) {
        unsafe { env::asm::map_set(self.value, key.into_placeholder(), value.into_placeholder()) }
    }

    #[inline(always)]
    pub fn get(&self, key: &K) -> Option<V>
    where
        K: Clone,
    {
        let placeholder = unsafe { env::asm::map_get(self.value, key.clone().into_placeholder()) };
        if placeholder.is_null() {
            None
        } else {
            Some(V::from_placeholder(placeholder))
        }
    }

    /// Remove a key-value pair from the map
    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: Clone + IntoPlaceholder,
        V: FromPlaceholder,
    {
        let key_placeholder = key.clone().into_placeholder();
        let result = unsafe { env::asm::map_remove(self.value, key_placeholder) };
        if result.is_null() {
            None
        } else {
            Some(V::from_placeholder(result))
        }
    }

    /// Check if the map contains a specific key
    pub fn contains_key(&self, key: &K) -> bool
    where
        K: Clone + IntoPlaceholder,
    {
        let key_placeholder = key.clone().into_placeholder();
        unsafe { env::asm::map_has_key(self.value, key_placeholder) }
    }

    /// Get all keys in the map
    pub fn keys(&self) -> Array<K>
    where
        K: FromPlaceholder,
    {
        let keys_placeholder = unsafe { env::asm::map_keys(self.value) };
        Array::from_placeholder(keys_placeholder)
    }

    /// Get all values in the map
    pub fn values(&self) -> Array<V>
    where
        V: FromPlaceholder,
    {
        let values_placeholder = unsafe { env::asm::map_values(self.value) };
        Array::from_placeholder(values_placeholder)
    }

    /// Clear all entries from the map
    pub fn clear(&mut self) {
        unsafe { env::asm::map_clear(self.value) }
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K: Primitive + std::hash::Hash + Eq + Clone, V: Clone> Map<K, V> {
    pub fn new() -> Self {
        Self { value: HashMap::new() }
    }

    pub fn size(&self) -> usize {
        self.value.len()
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn put(&mut self, key: K, value: V) {
        self.value.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.value.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.value.remove(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.value.contains_key(key)
    }

    pub fn keys(&self) -> Array<K> {
        let mut result = Array::new();
        for key in self.value.keys() {
            result.push(key.clone());
        }
        result
    }

    pub fn values(&self) -> Array<V> {
        let mut result = Array::new();
        for value in self.value.values() {
            result.push(value.clone());
        }
        result
    }

    pub fn clear(&mut self) {
        self.value.clear();
    }
}

#[cfg(target_family = "wasm")]
impl<K: Primitive + Clone + 'static, V: Clone + 'static> FromPlaceholder for Map<K, V> {
    #[inline(always)]
    fn from_placeholder(placeholder: Placeholder) -> Self {
        Self { value: placeholder, _marker: core::marker::PhantomData }
    }
}

#[cfg(target_family = "wasm")]
impl<K: Primitive + Clone + 'static, V: Clone + 'static> IntoPlaceholder for Map<K, V> {
    #[inline(always)]
    fn into_placeholder(self) -> Placeholder {
        self.value
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K: Primitive + Clone + std::hash::Hash + Eq, V: Clone> Default for Map<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_family = "wasm")]
impl<K: Primitive + IntoPlaceholder + Clone, V: IntoPlaceholder + FromPlaceholder + Clone + 'static> Default for Map<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
