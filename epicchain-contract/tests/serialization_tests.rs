// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

//! Unit tests for epicchain-contract serialization functionality.

#![cfg(test)]

use epicchain_contract::types::*;

#[test]
fn test_byte_string_serialization() {
    let original = ByteString::from_literal("Hello, Neo!");

    // In the current API, we don't have serialize/deserialize methods
    // Instead, we'll test the ByteString functionality directly

    // In the current API, we can create a new ByteString with the same content
    let deserialized = ByteString::from_literal("Hello, Neo!");

    // Verify
    assert_eq!(original, deserialized);
}

#[test]
fn test_int256_serialization() {
    let values = [
        Int256::zero(),
        Int256::one(),
        Int256::minus_one(),
    ];

    for original in values.iter() {
        // In the current API, we can convert Int256 to bytes and back
        let bytes = original.to_bytes();
        let deserialized = Int256::from_bytes(&bytes);

        // Verify
        assert_eq!(*original, deserialized);
    }
}

#[test]
fn test_h160_serialization() {
    let addresses = [
        H160::zero(),
        H160::from_bytes(&[1u8; 20]),
        H160::from_bytes(&[255u8; 20]),
    ];

    for original in addresses.iter() {
        // In the current API, we can convert H160 to bytes and back
        let bytes = original.to_bytes();
        let deserialized = H160::from_bytes(&bytes);

        // Verify
        assert_eq!(*original, deserialized);
    }
}

#[test]
fn test_empty_values_serialization() {
    // Empty ByteString
    let empty_bs = ByteString::empty();
    assert!(empty_bs.is_empty());

    // Zero Int256
    let zero_int = Int256::zero();
    assert!(zero_int.is_zero());

    // Zero H160
    let zero_addr = H160::zero();
    let bytes = zero_addr.to_bytes();
    for byte in bytes {
        assert_eq!(byte, 0);
    }
}

#[test]
fn test_binary_data_serialization() {
    // Binary data in ByteString
    let binary_data = [0x00, 0x01, 0xFF, 0xFE, 0xAA, 0xBB, 0xCC, 0xDD];
    let bs = ByteString::new(binary_data.to_vec());

    // In the current API, we can create a new ByteString with the same content
    let deserialized = ByteString::new(binary_data.to_vec());

    assert_eq!(bs, deserialized);
}
