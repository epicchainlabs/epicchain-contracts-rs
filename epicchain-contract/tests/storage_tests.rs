// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

//! Unit tests for epicchain-contract storage functionality.

#![cfg(test)]

use epicchain_contract::types::*;
mod mock_env;
use mock_env::{MockEpicChainEnvironment, storage::MockStorageMap};

#[test]
fn test_storage_map_put_get() {
    // Create mock environment and storage
    let mut env = MockEpicChainEnvironment::new();
    let mut storage = MockStorageMap::new(&mut env);

    // Test data
    let key = ByteString::from_literal("test_key");
    let value = ByteString::from_literal("test_value");

    // Put key-value pair
    storage.put(key.clone(), value.clone());

    // Retrieve value
    let retrieved = storage.get(key.clone());

    // Verify value was stored correctly
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), value);
}

#[test]
fn test_storage_map_delete() {
    // Create mock environment and storage
    let mut env = MockEpicChainEnvironment::new();
    let mut storage = MockStorageMap::new(&mut env);

    // Test data
    let key = ByteString::from_literal("test_key");
    let value = ByteString::from_literal("test_value");

    // Put key-value pair
    storage.put(key.clone(), value);

    // Delete key-value pair
    storage.delete(key.clone());

    // Verify key was deleted
    let retrieved = storage.get(key.clone());
    assert!(retrieved.is_none());
}

#[test]
fn test_storage_map_contains_key() {
    // Create mock environment and storage
    let mut env = MockEpicChainEnvironment::new();
    let mut storage = MockStorageMap::new(&mut env);

    // Test data
    let key = ByteString::from_literal("test_key");
    let value = ByteString::from_literal("test_value");
    let non_existent_key = ByteString::from_literal("non_existent_key");

    // Initially key doesn't exist
    assert!(!storage.contains_key(key.clone()));

    // Put key-value pair
    storage.put(key.clone(), value);

    // Verify key exists
    assert!(storage.contains_key(key.clone()));

    // Verify non-existent key doesn't exist
    assert!(!storage.contains_key(non_existent_key));
}

#[test]
fn test_storage_map_multiple_operations() {
    // Create mock environment and storage
    let mut env = MockEpicChainEnvironment::new();
    let mut storage = MockStorageMap::new(&mut env);

    // Test data
    let keys = [
        ByteString::from_literal("key1"),
        ByteString::from_literal("key2"),
        ByteString::from_literal("key3"),
    ];

    let values = [
        ByteString::from_literal("value1"),
        ByteString::from_literal("value2"),
        ByteString::from_literal("value3"),
    ];

    // Put multiple key-value pairs
    for i in 0..keys.len() {
        storage.put(keys[i].clone(), values[i].clone());
    }

    // Verify all keys exist with correct values
    for i in 0..keys.len() {
        assert!(storage.contains_key(keys[i].clone()));
        let retrieved = storage.get(keys[i].clone());
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), values[i]);
    }

    // Delete second key
    storage.delete(keys[1].clone());

    // Verify second key is deleted
    assert!(!storage.contains_key(keys[1].clone()));
    assert!(storage.get(keys[1].clone()).is_none());

    // Verify other keys still exist
    assert!(storage.contains_key(keys[0].clone()));
    assert!(storage.contains_key(keys[2].clone()));

    // Update first key's value
    let new_value = ByteString::from_literal("new_value1");
    storage.put(keys[0].clone(), new_value.clone());

    // Verify first key has updated value
    let retrieved = storage.get(keys[0].clone());
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), new_value);
}

#[test]
fn test_storage_map_empty_key_value() {
    // Create mock environment and storage
    let mut env = MockEpicChainEnvironment::new();
    let mut storage = MockStorageMap::new(&mut env);

    // Test with empty key
    let empty_key = ByteString::empty();
    let value = ByteString::from_literal("value_for_empty_key");

    // Store value with empty key
    storage.put(empty_key.clone(), value.clone());

    // Verify empty key exists with correct value
    assert!(storage.contains_key(empty_key.clone()));
    let retrieved = storage.get(empty_key.clone());
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), value);

    // Test with empty value
    let key = ByteString::from_literal("key_for_empty_value");
    let empty_value = ByteString::empty();

    // Store empty value
    storage.put(key.clone(), empty_value.clone());

    // Verify key exists with empty value
    assert!(storage.contains_key(key.clone()));
    let retrieved = storage.get(key.clone());
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), empty_value);
}

#[test]
fn test_storage_map_special_characters() {
    // Create mock environment and storage
    let mut env = MockEpicChainEnvironment::new();
    let mut storage = MockStorageMap::new(&mut env);

    // Test with special character key
    let special_key = ByteString::from_literal("!@#$%^&*()_+");
    let value = ByteString::from_literal("value_for_special_key");

    // Store value with special character key
    storage.put(special_key.clone(), value.clone());

    // Verify special character key exists with correct value
    assert!(storage.contains_key(special_key.clone()));
    let retrieved = storage.get(special_key.clone());
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), value);

    // Test with binary data
    let binary_key = ByteString::new(vec![0x00, 0x01, 0xFF, 0xFE]);
    let binary_value = ByteString::new(vec![0xAA, 0xBB, 0xCC, 0xDD]);

    // Store binary value with binary key
    storage.put(binary_key.clone(), binary_value.clone());

    // Verify binary key exists with correct binary value
    assert!(storage.contains_key(binary_key.clone()));
    let retrieved = storage.get(binary_key.clone());
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), binary_value);
}

#[test]
fn test_storage_map_overwrite() {
    // Create mock environment and storage
    let mut env = MockEpicChainEnvironment::new();
    let mut storage = MockStorageMap::new(&mut env);

    // Test data
    let key = ByteString::from_literal("overwrite_key");
    let value1 = ByteString::from_literal("initial_value");
    let value2 = ByteString::from_literal("updated_value");

    // Put initial key-value pair
    storage.put(key.clone(), value1.clone());

    // Verify initial value
    let retrieved = storage.get(key.clone());
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), value1);

    // Overwrite with new value
    storage.put(key.clone(), value2.clone());

    // Verify updated value
    let retrieved = storage.get(key.clone());
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), value2);
}