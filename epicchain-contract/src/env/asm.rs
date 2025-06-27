// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![allow(unused)]

#[cfg(target_family = "wasm")]
use crate::types::{placeholder::*, *};

#[link(wasm_import_module = "epicchain.asm")]
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
extern "C" {
    /// `new_array` creates a new empty `Array`.
    pub(crate) fn array_new() -> Placeholder;

    /// `new_array_size` creates a new `Array` with the given size.
    pub(crate) fn array_with_size(size: usize) -> Placeholder;

    /// `array_push` pushes a value onto the array.
    pub(crate) fn array_push(array: Placeholder, value: Placeholder);

    /// `array_pop` pops a value from the array.
    pub(crate) fn array_pop(array: Placeholder) -> Placeholder;

    /// `array_get` gets a value from the array.
    pub(crate) fn array_get(array: Placeholder, index: usize) -> Placeholder;

    /// `array_set` sets a value in the array.
    pub(crate) fn array_set(array: Placeholder, index: usize, value: Placeholder);

    /// `array_remove` removes a value from the array.
    pub(crate) fn array_remove(array: Placeholder, index: usize);

    /// `array_clear` clears the array, the array will be empty after this call.
    pub(crate) fn array_clear(array: Placeholder);

    /// `array_size` gets the size of the array.
    pub(crate) fn array_size(array: Placeholder) -> usize;

    /// `array_reverse` reverses the array.
    pub(crate) fn array_reverse(array: Placeholder);

    /// `buffer_with_size` creates a new `Buffer` with the given size.
    pub(crate) fn buffer_with_size(size: usize) -> Buffer;

    /// `buffer_copy` copies a buffer to another buffer.
    pub(crate) fn buffer_copy(src: Buffer, dst: Buffer, start_index: usize, count: usize);

    /// `buffer_sub` creates a new `Buffer` from a subarray of the given buffer.
    pub(crate) fn buffer_sub(src: Buffer, start_index: usize, count: usize) -> Buffer;

    /// `buffer_concat` concatenates two buffers.
    pub(crate) fn buffer_concat(src: Buffer, dst: Buffer) -> Buffer;

    /// `buffer_search` searches for a value in the buffer.
    pub(crate) fn buffer_search(buffer: Buffer, sub_buffer: ByteString, start_index: usize) -> usize;

    /// `buffer_search_backward` searches for a value in the buffer backward.
    pub(crate) fn buffer_search_backward(buffer: Buffer, sub_buffer: ByteString, start_index: usize) -> usize;

    /// `buffer_size` gets the size of the buffer.
    pub(crate) fn buffer_size(buffer: Buffer) -> usize;

    /// `buffer_reverse` reverses the buffer.
    pub(crate) fn buffer_reverse(buffer: Buffer);

    /// `new_map` creates a new empty `Map`.
    pub(crate) fn map_new() -> Placeholder;

    /// `map_set` sets a value in the map.
    pub(crate) fn map_set(map: Placeholder, key: Placeholder, value: Placeholder);

    /// `map_get` gets a value from the map.
    pub(crate) fn map_get(map: Placeholder, key: Placeholder) -> Placeholder;

    /// `map_remove` removes a key from the map.
    pub(crate) fn map_remove(map: Placeholder, key: Placeholder) -> Placeholder;

    /// `map_has_key` checks if the map contains the given key.
    pub(crate) fn map_has_key(map: Placeholder, key: Placeholder) -> bool;

    /// `map_clear` clears the map, the map will be empty after this call.
    pub(crate) fn map_clear(map: Placeholder);

    /// `map_size` gets the size of the map.
    pub(crate) fn map_size(map: Placeholder) -> usize;

    /// `map_keys` returns the keys of the map.
    pub(crate) fn map_keys(map: Placeholder) -> Placeholder;

    /// `map_values` returns the values of the map.
    pub(crate) fn map_values(map: Placeholder) -> Placeholder;

    /// `string_sub` returns a substring of the given string.
    pub(crate) fn string_sub(str: ByteString, start_index: usize, end_index: usize) -> ByteString;

    /// `string_concat` concatenates two strings.
    pub(crate) fn string_concat(a: ByteString, b: ByteString) -> ByteString;

    /// `string_len` returns the length of the given string.
    pub(crate) fn string_len(str: ByteString) -> usize;

    /// `string_from_literal` creates a new string from a literal.
    pub(crate) fn string_from_literal(literal: &str) -> ByteString;

    /// `string_eq` checks if two strings are equal.
    pub(crate) fn string_eq(a: ByteString, b: ByteString) -> bool;

    /// `string_empty` returns an empty string.
    pub(crate) fn string_empty() -> ByteString;

    /// `assert` asserts the given condition.
    /// If the condition is false, the execution will be aborted.
    pub(crate) fn assert(condition: bool);

    /// `assert_with_message` asserts the given condition with the given message.
    /// If the condition is false, the execution will be aborted with the given message.
    pub(crate) fn assert_with_message(condition: bool, message: ByteString);

    /// `abort` aborts the execution.
    pub(crate) fn abort() -> !;

    /// `abort_with_message` aborts the execution with the given message.
    pub(crate) fn abort_with_message(message: ByteString) -> !;
}
