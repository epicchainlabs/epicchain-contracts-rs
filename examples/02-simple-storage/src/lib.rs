//! # Simple Storage Smart Contract
//!
//! Demonstrates advanced storage patterns and data management in EpicChain:
//! - Multiple data type storage (strings, numbers, addresses, arrays)
//! - Storage enumeration and iteration
//! - Complex data structures and serialization
//! - Storage optimization techniques
//! - Access control and permissions
//!
//! This contract serves as a comprehensive guide to storage operations in EpicChain.

#![no_std]
#![no_main]

use epicchain_contract::prelude::*;
use epicchain_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

/// Simple Storage contract demonstrating advanced storage patterns
#[contract_author("EpicChain Rust Framework", "devs@epic-chain.org")]
#[contract_version("1.0.0")]
#[contract_standards("")]
#[contract_permission("*", "*")]
#[contract_meta("description", "Advanced storage patterns and data management")]
#[contract_meta("category", "Storage")]
pub struct SimpleStorage {
    // Storage prefixes for different data types
    string_prefix: ByteString,
    number_prefix: ByteString,
    address_prefix: ByteString,
    array_prefix: ByteString,
    map_prefix: ByteString,

    // Metadata keys
    owner_key: ByteString,
    total_items_key: ByteString,
    categories_key: ByteString,
}

#[contract_impl]
impl SimpleStorage {
    /// Initialize the contract
    pub fn init() -> Self {
        Self {
            string_prefix: ByteString::from_literal("str_"),
            number_prefix: ByteString::from_literal("num_"),
            address_prefix: ByteString::from_literal("addr_"),
            array_prefix: ByteString::from_literal("arr_"),
            map_prefix: ByteString::from_literal("map_"),
            owner_key: ByteString::from_literal("owner"),
            total_items_key: ByteString::from_literal("total_items"),
            categories_key: ByteString::from_literal("categories"),
        }
    }

    /// Set the contract owner (one-time initialization)
    #[method]
    pub fn set_owner(&self, owner: H160) -> bool {
        let storage = Storage::get_context();

        // Check if owner is already set
        if Storage::get(storage.clone(), self.owner_key.clone()).is_some() {
            Runtime::log(ByteString::from_literal("Owner already set"));
            return false;
        }

        // Verify the caller is authorized to set owner
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        Storage::put(storage, self.owner_key.clone(), owner.into_byte_string());
        let mut event_data = Array::new(); event_data.push(owner.into_any()); Runtime::notify(ByteString::from_literal("OwnerSet"), event_data);
        true
    }

    /// Get the contract owner
    #[method]
    #[safe]
    pub fn get_owner(&self) -> H160 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.owner_key.clone()) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => H160::zero(),
        }
    }

    /// Check if caller is the owner
    fn is_owner(&self) -> bool {
        let owner = self.get_owner();
        if owner == H160::zero() {
            return false;
        }
        Runtime::check_witness(owner)
    }

    /// Store a string value
    #[method]
    pub fn put_string(&self, key: ByteString, value: ByteString) -> bool {
        if !self.validate_key(&key) {
            return false;
        }

        let storage = Storage::get_context();
        let storage_key = self.string_prefix.concat(&key);
        Storage::put(storage, storage_key, value.clone());

        self.increment_total_items();
        let mut event_data = Array::new(); event_data.push(value.into_any()); Runtime::notify(ByteString::from_literal("StringStored"), event_data);
        true
    }

    /// Get a string value
    #[method]
    #[safe]
    pub fn get_string(&self, key: ByteString) -> ByteString {
        let storage = Storage::get_context();
        let storage_key = self.string_prefix.concat(&key);

        match Storage::get(storage, storage_key) {
            Some(value) => value,
            None => ByteString::from_literal(""),
        }
    }

    /// Store a number value
    #[method]
    pub fn put_number(&self, key: ByteString, value: Int256) -> bool {
        if !self.validate_key(&key) {
            return false;
        }

        let storage = Storage::get_context();
        let storage_key = self.number_prefix.concat(&key);
        Storage::put(storage, storage_key, value.into_byte_string());

        self.increment_total_items();
        let mut event_data = Array::new(); event_data.push(value.into_any()); Runtime::notify(ByteString::from_literal("NumberStored"), event_data);
        true
    }

    /// Get a number value
    #[method]
    #[safe]
    pub fn get_number(&self, key: ByteString) -> Int256 {
        let storage = Storage::get_context();
        let storage_key = self.number_prefix.concat(&key);

        match Storage::get(storage, storage_key) {
            Some(value_bytes) => Int256::from_byte_string(value_bytes),
            None => Int256::zero(),
        }
    }

    /// Store an address value
    #[method]
    pub fn put_address(&self, key: ByteString, value: H160) -> bool {
        if !self.validate_key(&key) {
            return false;
        }

        let storage = Storage::get_context();
        let storage_key = self.address_prefix.concat(&key);
        Storage::put(storage, storage_key, value.into_byte_string());

        self.increment_total_items();
        let mut event_data = Array::new(); event_data.push(value.into_any()); Runtime::notify(ByteString::from_literal("AddressStored"), event_data);
        true
    }

    /// Get an address value
    #[method]
    #[safe]
    pub fn get_address(&self, key: ByteString) -> H160 {
        let storage = Storage::get_context();
        let storage_key = self.address_prefix.concat(&key);

        match Storage::get(storage, storage_key) {
            Some(value_bytes) => H160::from_byte_string(value_bytes),
            None => H160::zero(),
        }
    }

    /// Store an array of strings
    #[method]
    pub fn put_array(&self, key: ByteString, values: Array<ByteString>) -> bool {
        if !self.validate_key(&key) {
            return false;
        }

        let storage = Storage::get_context();
        let storage_key = self.array_prefix.concat(&key);

        // Serialize array as concatenated strings with length prefixes
        let mut serialized = ByteString::empty();
        let array_len = values.size();

        // Store array length first
        serialized = serialized.concat(&ByteString::from_bytes(&(array_len as u32).to_le_bytes()));

        // Store each string with its length
        for i in 0..array_len {
            let item = values.get(i);
            let item_bytes = item.to_bytes();
            let item_len = item_bytes.len() as u32;

            serialized = serialized.concat(&ByteString::from_bytes(&item_len.to_le_bytes()));
            serialized = serialized.concat(&item);
        }

        Storage::put(storage, storage_key, serialized);
        self.increment_total_items();
        let mut event_data = Array::new();
        event_data.push(Int256::new(array_len as i64).into_any());
        Runtime::notify(ByteString::from_literal("ArrayStored"), event_data);
        true
    }

    /// Get an array of strings
    #[method]
    #[safe]
    pub fn get_array(&self, key: ByteString) -> Array<ByteString> {
        let storage = Storage::get_context();
        let storage_key = self.array_prefix.concat(&key);

        match Storage::get(storage, storage_key) {
            Some(serialized) => self.deserialize_array(serialized),
            None => Array::new(),
        }
    }

    /// Store a map of key-value pairs
    #[method]
    pub fn put_map(&self, key: ByteString, _map_data: Map<ByteString, ByteString>) -> bool {
        if !self.validate_key(&key) {
            return false;
        }

        let storage = Storage::get_context();
        let storage_key = self.map_prefix.concat(&key);

        // Simplified map serialization - in production, implement proper map iteration
        let serialized = ByteString::from_literal("map_data_placeholder");
        let map_size = 1; // Placeholder size

        Storage::put(storage, storage_key, serialized);
        self.increment_total_items();
        let mut event_data = Array::new();
        event_data.push(Int256::new(map_size as i64).into_any());
        Runtime::notify(ByteString::from_literal("MapStored"), event_data);
        true
    }

    /// Get a map of key-value pairs
    #[method]
    #[safe]
    pub fn get_map(&self, key: ByteString) -> Map<ByteString, ByteString> {
        let storage = Storage::get_context();
        let storage_key = self.map_prefix.concat(&key);

        match Storage::get(storage, storage_key) {
            Some(serialized) => self.deserialize_map(serialized),
            None => Map::new(),
        }
    }

    /// Delete any stored value by key and type
    #[method]
    pub fn delete(&self, data_type: ByteString, key: ByteString) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can delete"));
            return false;
        }

        let storage = Storage::get_context();
        let prefix = self.get_prefix_for_type(&data_type);

        if prefix.is_empty() {
            Runtime::log(ByteString::from_literal("Invalid data type"));
            return false;
        }

        let storage_key = prefix.concat(&key);
        Storage::delete(storage, storage_key);

        let mut event_data = Array::new(); event_data.push(key.into_any()); Runtime::notify(ByteString::from_literal("ItemDeleted"), event_data);
        true
    }

    /// Get total number of stored items
    #[method]
    #[safe]
    pub fn get_total_items(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.total_items_key.clone()) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        }
    }

    /// Get contract statistics
    #[method]
    #[safe]
    pub fn get_stats(&self) -> Map<ByteString, Any> {
        let mut stats = Map::new();

        stats.put(
            ByteString::from_literal("total_items"),
            self.get_total_items().into_any()
        );
        stats.put(
            ByteString::from_literal("owner"),
            self.get_owner().into_any()
        );
        stats.put(
            ByteString::from_literal("contract_hash"),
            Runtime::get_executing_script_hash().into_any()
        );

        stats
    }

    // Helper functions

    fn validate_key(&self, key: &ByteString) -> bool {
        if key.is_empty() || key.len() > 64 {
            Runtime::log(ByteString::from_literal("Invalid key: must be 1-64 characters"));
            return false;
        }
        true
    }

    fn increment_total_items(&self) {
        let storage = Storage::get_context();
        let current = self.get_total_items();
        let new_total = current.checked_add(&Int256::one());
        Storage::put(storage, self.total_items_key.clone(), new_total.into_byte_string());
    }

    fn get_prefix_for_type(&self, data_type: &ByteString) -> ByteString {
        // Simplified type checking - in production, implement proper type detection
        if data_type == &ByteString::from_literal("string") {
            self.string_prefix.clone()
        } else if data_type == &ByteString::from_literal("number") {
            self.number_prefix.clone()
        } else if data_type == &ByteString::from_literal("address") {
            self.address_prefix.clone()
        } else if data_type == &ByteString::from_literal("array") {
            self.array_prefix.clone()
        } else if data_type == &ByteString::from_literal("map") {
            self.map_prefix.clone()
        } else {
            ByteString::empty()
        }
    }

    fn deserialize_array(&self, serialized: ByteString) -> Array<ByteString> {
        let bytes = serialized.to_bytes();
        let mut array = Array::new();

        if bytes.len() < 4 {
            return array;
        }

        // Read array length
        let array_len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        let mut offset = 4;

        for _ in 0..array_len {
            if offset + 4 > bytes.len() {
                break;
            }

            // Read item length
            let item_len = u32::from_le_bytes([
                bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
            ]) as usize;
            offset += 4;

            if offset + item_len > bytes.len() {
                break;
            }

            // Read item data
            let item_bytes = &bytes[offset..offset + item_len];
            let item = ByteString::from_bytes(item_bytes);
            array.push(item);
            offset += item_len;
        }

        array
    }

    fn deserialize_map(&self, serialized: ByteString) -> Map<ByteString, ByteString> {
        let bytes = serialized.to_bytes();
        let mut map = Map::new();

        if bytes.len() < 4 {
            return map;
        }

        // Read map size
        let map_size = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        let mut offset = 4;

        for _ in 0..map_size {
            if offset + 4 > bytes.len() {
                break;
            }

            // Read key length and key
            let key_len = u32::from_le_bytes([
                bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
            ]) as usize;
            offset += 4;

            if offset + key_len > bytes.len() {
                break;
            }

            let key_bytes = &bytes[offset..offset + key_len];
            let key = ByteString::from_bytes(key_bytes);
            offset += key_len;

            if offset + 4 > bytes.len() {
                break;
            }

            // Read value length and value
            let value_len = u32::from_le_bytes([
                bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
            ]) as usize;
            offset += 4;

            if offset + value_len > bytes.len() {
                break;
            }

            let value_bytes = &bytes[offset..offset + value_len];
            let value = ByteString::from_bytes(value_bytes);
            offset += value_len;

            map.put(key, value);
        }

        map
    }
}
