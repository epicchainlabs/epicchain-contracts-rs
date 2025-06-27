// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

//! Mock EpicChain environment for testing purposes.
//! This module provides a simulated EpicChain blockchain environment
//! that can be used for unit testing EpicChain smart contracts.

#![cfg(test)]

use std::collections::HashMap;
use epicchain_contract::types::*;

/// Notification event emitted by a contract
pub struct Notification {
    /// Name of the notification event
    pub name: ByteString,
    /// Data associated with the notification
    pub data: Any,
}

/// Mock implementation of the EpicChain blockchain environment
pub struct MockEpicChainEnvironment {
    /// Storage map for contract state
    storage: HashMap<Vec<u8>, Vec<u8>>,
    /// List of witness accounts
    witnesses: Vec<H160>,
    /// Current blockchain timestamp
    timestamp: u64,
    /// Gas remaining for execution
    epicpulse_left: u64,
    /// List of emitted notifications
    notifications: Vec<Notification>,
    /// Current contract hash
    executing_script_hash: H160,
    /// Calling contract hash
    calling_script_hash: H160,
    /// Entry script hash
    entry_script_hash: H160,
}

impl MockEpicChainEnvironment {
    /// Create a new mock environment with default values
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            witnesses: Vec::new(),
            timestamp: 0,
            epicpulse_left: 1_000_000_000,
            notifications: Vec::new(),
            executing_script_hash: H160::zero(),
            calling_script_hash: H160::zero(),
            entry_script_hash: H160::zero(),
        }
    }

    /// Create a new mock environment with a specific executing script hash
    pub fn with_executing_script_hash(mut self, script_hash: H160) -> Self {
        self.executing_script_hash = script_hash;
        self
    }

    /// Create a new mock environment with a specific calling script hash
    pub fn with_calling_script_hash(mut self, script_hash: H160) -> Self {
        self.calling_script_hash = script_hash;
        self
    }

    /// Create a new mock environment with a specific entry script hash
    pub fn with_entry_script_hash(mut self, script_hash: H160) -> Self {
        self.entry_script_hash = script_hash;
        self
    }

    /// Get the executing script hash
    pub fn executing_script_hash(&self) -> H160 {
        self.executing_script_hash.clone()
    }

    /// Get the calling script hash
    pub fn calling_script_hash(&self) -> H160 {
        self.calling_script_hash.clone()
    }

    /// Get the entry script hash
    pub fn entry_script_hash(&self) -> H160 {
        self.entry_script_hash.clone()
    }

    // Storage operations

    /// Put a value in storage
    pub fn storage_put(&mut self, key: &[u8], value: &[u8]) {
        self.storage.insert(key.to_vec(), value.to_vec());
    }

    /// Get a value from storage
    pub fn storage_get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.storage.get(key).cloned()
    }

    /// Delete a value from storage
    pub fn storage_delete(&mut self, key: &[u8]) {
        self.storage.remove(key);
    }

    /// Check if a key exists in storage
    pub fn storage_contains(&self, key: &[u8]) -> bool {
        self.storage.contains_key(key)
    }

    // Witness operations

    /// Add a witness account
    pub fn add_witness(&mut self, account: H160) {
        if !self.witnesses.contains(&account) {
            self.witnesses.push(account);
        }
    }

    /// Check if an account is a witness
    pub fn has_witness(&self, account: H160) -> bool {
        self.witnesses.contains(&account)
    }

    // Time operations

    /// Set the current timestamp
    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }

    /// Get the current timestamp
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    // Gas operations

    /// Set the remaining gas
    pub fn set_epicpulse_left(&mut self, gas: u64) {
        self.epicpulse_left = gas;
    }

    /// Get the remaining gas
    pub fn get_epicpulse_left(&self) -> u64 {
        self.epicpulse_left
    }

    // Notification operations

    /// Add a notification
    pub fn add_notification(&mut self, name: ByteString, data: Any) {
        self.notifications.push(Notification { name, data });
    }

    /// Get all notifications
    pub fn get_notifications(&self) -> &[Notification] {
        &self.notifications
    }

    /// Clear all notifications
    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }
}

// Extension traits for framework types to work with mock environment

/// Trait for types that can be initialized with a mock environment
pub trait WithMockEnv {
    /// Create a new instance with the given mock environment
    fn new_with_env(env: &mut MockEpicChainEnvironment) -> Self;
}

// Mock implementation of runtime functions that use the mock environment
pub mod runtime {
    use super::*;

    /// Check if an account is a witness (authorized)
    pub fn check_witness_with_env(env: &MockEpicChainEnvironment, account: H160) -> bool {
        env.has_witness(account)
    }

    /// Emit a notification event
    pub fn notify_with_env(env: &mut MockEpicChainEnvironment, name: ByteString, data: Any) {
        env.add_notification(name, data);
    }

    /// Get the current timestamp
    pub fn get_timestamp_with_env(env: &MockEpicChainEnvironment) -> u64 {
        env.get_timestamp()
    }

    /// Get the remaining gas
    pub fn epicpulse_left_with_env(env: &MockEpicChainEnvironment) -> u64 {
        env.get_epicpulse_left()
    }

    /// Get the executing script hash
    pub fn executing_script_hash_with_env(env: &MockEpicChainEnvironment) -> H160 {
        env.executing_script_hash()
    }

    /// Get the calling script hash
    pub fn calling_script_hash_with_env(env: &MockEpicChainEnvironment) -> H160 {
        env.calling_script_hash()
    }

    /// Get the entry script hash
    pub fn entry_script_hash_with_env(env: &MockEpicChainEnvironment) -> H160 {
        env.entry_script_hash()
    }
}

// Mock implementation of storage operations
pub mod storage {
    use super::*;

    /// A storage map that uses the mock environment
    pub struct MockStorageMap<'a> {
        env: &'a mut MockEpicChainEnvironment,
    }

    impl<'a> MockStorageMap<'a> {
        /// Create a new storage map with the given environment
        pub fn new(env: &'a mut MockEpicChainEnvironment) -> Self {
            Self { env }
        }

        /// Put a value in storage
        pub fn put(&mut self, key: ByteString, value: ByteString) {
            // Convert ByteString to Vec<u8> for storage
            let key_bytes = key.to_bytes().to_vec();
            let value_bytes = value.to_bytes().to_vec();
            self.env.storage_put(&key_bytes, &value_bytes);
        }

        /// Get a value from storage
        pub fn get(&self, key: ByteString) -> Option<ByteString> {
            let key_bytes = key.to_bytes().to_vec();
            self.env.storage_get(&key_bytes)
                .map(|bytes| ByteString::from_bytes(&bytes))
        }

        /// Delete a value from storage
        pub fn delete(&mut self, key: ByteString) {
            let key_bytes = key.to_bytes().to_vec();
            self.env.storage_delete(&key_bytes);
        }

        /// Check if a key exists in storage
        pub fn contains_key(&self, key: ByteString) -> bool {
            let key_bytes = key.to_bytes().to_vec();
            self.env.storage_contains(&key_bytes)
        }
    }
}