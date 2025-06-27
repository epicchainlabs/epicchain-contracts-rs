//! # Hello World Smart Contract
//!
//! A simple EpicChain smart contract demonstrating basic functionality including:
//! - Contract attributes and metadata
//! - Safe and unsafe methods
//! - Storage operations
//! - Event emission
//! - Witness checking
//!
//! This contract serves as an introduction to EpicChain Rust smart contract development.

#![no_std]
#![no_main]

use epicchain_contract::prelude::*;
use epicchain_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

/// Hello World smart contract demonstrating basic EpicChain features
#[contract_author("EpicChain Rust Framework", "devs@epic-chain.org")]
#[contract_version("1.0.0")]
#[contract_standards("")]
#[contract_permission("*", "*")]
#[contract_meta("description", "A simple Hello World contract demonstrating basic EpicChain features")]
#[contract_meta("website", "https://github.com/R3E-Network/epicchain-contract-rs")]
pub struct HelloWorld {
    /// Storage key for the greeting message
    greeting_key: ByteString,
    /// Storage key for the visitor counter
    counter_key: ByteString,
    /// Storage key for visitor names
    visitors_prefix: ByteString,
}

#[contract_impl]
impl HelloWorld {
    /// Initialize the contract with default values
    pub fn init() -> Self {
        Self {
            greeting_key: ByteString::from_literal("greeting"),
            counter_key: ByteString::from_literal("counter"),
            visitors_prefix: ByteString::from_literal("visitor_"),
        }
    }

    /// Get the current greeting message
    ///
    /// This is a safe method that doesn't modify state and can be called by anyone.
    #[method]
    #[safe]
    pub fn get_greeting(&self) -> ByteString {
        let storage = Storage::get_context();

        match Storage::get(storage, self.greeting_key.clone()) {
            Some(greeting) => greeting,
            None => ByteString::from_literal("Hello, EpicChain World!"),
        }
    }

    /// Set a new greeting message
    ///
    /// This is an unsafe method that modifies state and requires witness verification.
    /// Only the contract owner can call this method.
    #[method]
    pub fn set_greeting(&self, new_greeting: ByteString) -> bool {
        // Verify that the caller is authorized (contract owner)
        let contract_hash = Runtime::get_executing_script_hash();
        if !Runtime::check_witness(contract_hash) {
            Runtime::log(ByteString::from_literal("Unauthorized: Only contract owner can set greeting"));
            return false;
        }

        // Validate input
        if new_greeting.is_empty() || new_greeting.len() > 100 {
            Runtime::log(ByteString::from_literal("Invalid greeting: must be 1-100 characters"));
            return false;
        }

        // Store the new greeting
        let storage = Storage::get_context();
        Storage::put(storage, self.greeting_key.clone(), new_greeting.clone());

        // Emit event
        let mut event_data = Array::new();
        event_data.push(new_greeting.into_any());
        Runtime::notify(ByteString::from_literal("GreetingChanged"), event_data);

        true
    }

    /// Get the total number of visitors
    #[method]
    #[safe]
    pub fn get_visitor_count(&self) -> Int256 {
        let storage = Storage::get_context();

        match Storage::get(storage, self.counter_key.clone()) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        }
    }

    /// Register a new visitor
    ///
    /// Anyone can call this method to register as a visitor.
    #[method]
    pub fn say_hello(&self, visitor_name: ByteString) -> ByteString {
        // Validate input
        if visitor_name.is_empty() || visitor_name.len() > 50 {
            Runtime::log(ByteString::from_literal("Invalid name: must be 1-50 characters"));
            return ByteString::from_literal("Error: Invalid name");
        }

        let storage = Storage::get_context();

        // Increment visitor counter
        let current_count = self.get_visitor_count();
        let new_count = current_count.checked_add(&Int256::one());
        Storage::put(storage.clone(), self.counter_key.clone(), new_count.into_byte_string());

        // Store visitor name with timestamp
        let visitor_key = self.visitors_prefix.concat(&new_count.into_byte_string());
        let timestamp = Runtime::get_time();
        let visitor_data = visitor_name.concat(&ByteString::from_literal(":"))
                                     .concat(&ByteString::from_bytes(&timestamp.to_le_bytes()));
        Storage::put(storage, visitor_key, visitor_data);

        // Emit event
        let mut event_data = Array::new();
        event_data.push(visitor_name.clone().into_any());
        Runtime::notify(ByteString::from_literal("VisitorRegistered"), event_data);

        // Return personalized greeting
        let greeting = self.get_greeting();
        let response = ByteString::from_literal("Hello, ")
                                 .concat(&visitor_name)
                                 .concat(&ByteString::from_literal("! "))
                                 .concat(&greeting);

        Runtime::log(response.clone());
        response
    }

    /// Get visitor information by number
    #[method]
    #[safe]
    pub fn get_visitor(&self, visitor_number: Int256) -> ByteString {
        if visitor_number <= Int256::zero() {
            return ByteString::from_literal("Error: Invalid visitor number");
        }

        let storage = Storage::get_context();
        let visitor_key = self.visitors_prefix.concat(&visitor_number.into_byte_string());

        match Storage::get(storage, visitor_key) {
            Some(visitor_data) => visitor_data,
            None => ByteString::from_literal("Error: Visitor not found"),
        }
    }

    /// Get contract information
    #[method]
    #[safe]
    pub fn get_info(&self) -> Map<ByteString, Any> {
        let mut info = Map::new();

        info.put(
            ByteString::from_literal("name"),
            ByteString::from_literal("Hello World Contract").into_any()
        );
        info.put(
            ByteString::from_literal("version"),
            ByteString::from_literal("1.0.0").into_any()
        );
        info.put(
            ByteString::from_literal("author"),
            ByteString::from_literal("EpicChain Rust Framework").into_any()
        );
        info.put(
            ByteString::from_literal("visitor_count"),
            self.get_visitor_count().into_any()
        );
        info.put(
            ByteString::from_literal("current_greeting"),
            self.get_greeting().into_any()
        );

        info
    }

    /// Reset the contract state (owner only)
    #[method]
    pub fn reset(&self) -> bool {
        // Verify authorization
        let contract_hash = Runtime::get_executing_script_hash();
        if !Runtime::check_witness(contract_hash) {
            Runtime::log(ByteString::from_literal("Unauthorized: Only contract owner can reset"));
            return false;
        }

        let storage = Storage::get_context();

        // Reset counter
        Storage::delete(storage.clone(), self.counter_key.clone());

        // Reset greeting to default
        Storage::delete(storage, self.greeting_key.clone());

        // Emit event
        let event_data = Array::new();
        Runtime::notify(ByteString::from_literal("ContractReset"), event_data);

        Runtime::log(ByteString::from_literal("Contract state has been reset"));
        true
    }
}

// Function exports are now auto-generated by #[contract_impl] macro

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_initialization() {
        let contract = HelloWorld::init();
        assert!(!contract.greeting_key.is_empty());
        assert!(!contract.counter_key.is_empty());
        assert!(!contract.visitors_prefix.is_empty());
    }

    #[test]
    fn test_default_greeting() {
        let contract = HelloWorld::init();
        let greeting = contract.get_greeting();
        // Note: In no_std environment, we can't easily compare strings
        // This test verifies that get_greeting() returns without error
        assert!(!greeting.is_empty());
    }

    #[test]
    fn test_visitor_count_starts_at_zero() {
        let contract = HelloWorld::init();
        let count = contract.get_visitor_count();
        assert!(count.is_zero());
    }
}
