// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

//! Advanced unit tests for epicchain-contract types with comprehensive coverage.

#![cfg(test)]

use epicchain_contract::types::*;

#[test]
fn test_h256_operations() {
    // Test H256 zero
    let zero = H256::zero();
    let zero_bytes = zero.to_bytes();
    assert_eq!(zero_bytes.len(), 32);
    for byte in zero_bytes {
        assert_eq!(byte, 0);
    }

    // Test H256 from bytes
    let test_bytes = [0x42u8; 32];
    let h256 = H256::from_bytes(&test_bytes);
    let retrieved_bytes = h256.to_bytes();
    assert_eq!(retrieved_bytes, test_bytes);

    // Test H256 equality
    let h256_1 = H256::from_bytes(&test_bytes);
    let h256_2 = H256::from_bytes(&test_bytes);
    assert_eq!(h256_1.to_bytes(), h256_2.to_bytes());

    // Test different H256 values
    let different_bytes = [0x24u8; 32];
    let h256_different = H256::from_bytes(&different_bytes);
    assert_ne!(h256.to_bytes(), h256_different.to_bytes());
}

#[test]
fn test_int256_edge_cases() {
    // Test Int256 constants
    let zero = Int256::zero();
    let one = Int256::one();
    let minus_one = Int256::minus_one();

    // Test properties
    assert!(zero.is_zero());
    assert!(!zero.is_positive());
    assert!(!zero.is_negative());

    assert!(!one.is_zero());
    assert!(one.is_positive());
    assert!(!one.is_negative());

    assert!(!minus_one.is_zero());
    assert!(!minus_one.is_positive());
    assert!(minus_one.is_negative());

    // Test arithmetic operations
    let sum = one.checked_add(&one);
    assert!(!sum.is_zero());
    assert!(sum.is_positive());

    let diff = one.checked_sub(&one);
    assert!(diff.is_zero());

    let product = one.checked_mul(&one);
    assert_eq!(product.to_bytes(), one.to_bytes());

    let quotient = one.checked_div(&one);
    assert_eq!(quotient.to_bytes(), one.to_bytes());

    let negated = one.checked_neg();
    assert!(negated.is_negative());
}

#[test]
fn test_int256_serialization_round_trip() {
    let values = [
        Int256::zero(),
        Int256::one(),
        Int256::minus_one(),
    ];

    for value in values {
        let bytes = value.to_bytes();
        let deserialized = Int256::from_bytes(&bytes);

        // Check that properties are preserved
        assert_eq!(value.is_zero(), deserialized.is_zero());
        assert_eq!(value.is_positive(), deserialized.is_positive());
        assert_eq!(value.is_negative(), deserialized.is_negative());
    }
}

#[test]
fn test_byte_string_advanced_operations() {
    // Test empty ByteString
    let empty = ByteString::empty();
    assert!(empty.is_empty());

    // Test ByteString from literal
    let hello = ByteString::from_literal("Hello");
    let world = ByteString::from_literal("World");
    assert!(!hello.is_empty());
    assert!(!world.is_empty());

    // Test concatenation
    let hello_world = hello.concat(&world);
    assert!(!hello_world.is_empty());

    // Test concatenation with empty
    let hello_empty = hello.concat(&empty);
    assert_eq!(hello_empty, hello);

    let empty_hello = empty.concat(&hello);
    assert_eq!(empty_hello, hello);

    // Test equality
    let hello2 = ByteString::from_literal("Hello");
    assert_eq!(hello, hello2);
    assert_ne!(hello, world);

    // Test from bytes
    let bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello" in ASCII
    let hello_from_bytes = ByteString::from_bytes(&bytes);
    assert!(!hello_from_bytes.is_empty());
}

#[test]
fn test_array_comprehensive() {
    // Test with different types
    let mut int_array = Array::<i32>::new();
    let mut string_array = Array::<ByteString>::new();

    // Test pushing elements
    int_array.push(1);
    int_array.push(2);
    int_array.push(3);

    string_array.push(ByteString::from_literal("first"));
    string_array.push(ByteString::from_literal("second"));

    // Test getting elements
    let _first_int = int_array.get(0);
    let _second_int = int_array.get(1);
    let _third_int = int_array.get(2);

    // Test setting elements
    int_array.set(1, 42);
    let _updated_second = int_array.get(1);

    // Test string array operations separately to avoid borrowing issues
    let first_string = string_array.get(0);
    let second_string = string_array.get(1);

    // Verify strings are not empty before modifying
    assert!(!first_string.is_empty());
    assert!(!second_string.is_empty());

    // Now modify the array
    string_array.set(0, ByteString::from_literal("updated"));
    let updated_first = string_array.get(0);
    assert!(!updated_first.is_empty());
}

#[test]
fn test_map_operations() {
    let mut map = Map::<ByteString, i32>::new();

    // Test basic operations
    let key1 = ByteString::from_literal("key1");
    let key2 = ByteString::from_literal("key2");

    map.put(key1.clone(), 100);
    map.put(key2.clone(), 200);

    // Test retrieval
    let _value1 = map.get(&key1);
    let _value2 = map.get(&key2);

    // Test non-existent key
    let non_existent_key = ByteString::from_literal("non_existent");
    let _non_existent_value = map.get(&non_existent_key);

    // Test contains_key
    assert!(map.contains_key(&key1));
    assert!(map.contains_key(&key2));
    assert!(!map.contains_key(&non_existent_key));

    // Test removal
    let _removed_value = map.remove(&key1);
    assert!(!map.contains_key(&key1));
    assert!(map.contains_key(&key2));
}

#[test]
fn test_public_key_operations() {
    // Test PublicKey creation and operations
    let key_bytes = [0x03u8; 33]; // Compressed public key format
    let public_key = PublicKey::from_bytes(&key_bytes);

    let retrieved_bytes = public_key.to_bytes();
    assert_eq!(retrieved_bytes.len(), 33);
    assert_eq!(retrieved_bytes, key_bytes);

    // Test with different key
    let different_key_bytes = [0x02u8; 33];
    let different_public_key = PublicKey::from_bytes(&different_key_bytes);
    assert_ne!(public_key.to_bytes(), different_public_key.to_bytes());
}

#[test]
fn test_h160_comprehensive() {
    // Test H160 with various patterns
    let patterns = [
        [0x00u8; 20], // All zeros
        [0xFFu8; 20], // All ones
        [0xAAu8; 20], // Alternating pattern
    ];

    for pattern in patterns {
        let h160 = H160::from_bytes(&pattern);
        let retrieved = h160.to_bytes();
        assert_eq!(retrieved, pattern);
        assert_eq!(retrieved.len(), 20);
    }

    // Test zero specifically
    let zero = H160::zero();
    let zero_bytes = zero.to_bytes();
    assert_eq!(zero_bytes, [0u8; 20]);
}

#[test]
fn test_type_conversions() {
    // Test Int256 to ByteString conversion
    let int_val = Int256::one();
    let byte_string = int_val.into_byte_string();
    assert!(!byte_string.is_empty());

    // Test H160 to ByteString conversion
    let h160_val = H160::zero();
    let h160_byte_string = h160_val.into_byte_string();
    assert!(!h160_byte_string.is_empty());

    // Test H256 to ByteString conversion
    let h256_val = H256::zero();
    let h256_byte_string = h256_val.into_byte_string();
    assert!(!h256_byte_string.is_empty());
}

#[test]
fn test_serialization_consistency() {
    // Test that serialization is consistent across multiple calls
    let int_val = Int256::one();
    let bytes1 = int_val.to_bytes();
    let bytes2 = int_val.to_bytes();
    assert_eq!(bytes1, bytes2);

    let h160_val = H160::zero();
    let h160_bytes1 = h160_val.to_bytes();
    let h160_bytes2 = h160_val.to_bytes();
    assert_eq!(h160_bytes1, h160_bytes2);

    let h256_val = H256::zero();
    let h256_bytes1 = h256_val.to_bytes();
    let h256_bytes2 = h256_val.to_bytes();
    assert_eq!(h256_bytes1, h256_bytes2);
}
