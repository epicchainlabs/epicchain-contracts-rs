// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![allow(unused)]

#[cfg(target_family = "wasm")]
use crate::types::*;

#[link(wasm_import_module = "epicchain.stdlib")]
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
extern "C" {
    /// `base58_encode` encodes a string to base58.
    pub(crate) fn base58_encode(str: ByteString) -> ByteString;

    /// `base58_decode` decodes a base58 string to a string.
    pub(crate) fn base58_decode(str: ByteString) -> ByteString;

    /// `base64_encode` encodes a string to base64.
    pub(crate) fn base64_encode(str: ByteString) -> ByteString;

    /// `base64_decode` decodes a base64 string to a string.
    pub(crate) fn base64_decode(str: ByteString) -> ByteString;

    /// `json_serialize` serializes a string to JSON.
    pub(crate) fn json_serialize(str: ByteString) -> ByteString;

    /// `json_deserialize` deserializes a JSON string to a string.
    pub(crate) fn json_deserialize(str: ByteString) -> ByteString;

    /// `hex_encode` encodes a string to hex.
    pub(crate) fn hex_encode(str: ByteString) -> ByteString;

    /// `hex_decode` decodes a hex string to a string.
    pub(crate) fn hex_decode(str: ByteString) -> ByteString;

    /// `string_split` splits a string into an array of strings.
    pub(crate) fn string_split(str: ByteString, separator_str: ByteString) -> ByteString;
}
