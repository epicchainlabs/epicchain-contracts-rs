//! # Counter Smart Contract
//!
//! Demonstrates advanced state management and access control patterns:
//! - Multiple counter management with named counters
//! - Atomic increment/decrement operations
//! - Access control with multiple permission levels
//! - Event-driven architecture with detailed logging
//! - Statistical tracking and analytics
//! - Gas-optimized operations
//!
//! This contract showcases production-ready state management patterns.

#![no_std]
#![no_main]

use epicchain_contract::prelude::*;
use epicchain_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

/// Counter contract demonstrating advanced state management
#[contract_author("EpicChain Rust Framework", "devs@epic-chain.org")]
#[contract_version("1.0.0")]
#[contract_standards("")]
#[contract_permission("*", "*")]
#[contract_meta("description", "Advanced counter with multiple counters and access control")]
#[contract_meta("category", "State Management")]
pub struct Counter {
    // Storage keys
    default_counter_key: ByteString,
    counter_prefix: ByteString,
    owner_key: ByteString,
    operators_prefix: ByteString,

    // Statistics keys
    total_operations_key: ByteString,
    total_counters_key: ByteString,
    last_operation_key: ByteString,
}

/// Counter operation types for events and logging
#[derive(Clone)]
pub enum Operation {
    Increment,
    Decrement,
    Reset,
    Create,
    Delete,
}

impl Operation {
    fn to_string(&self) -> ByteString {
        match self {
            Operation::Increment => ByteString::from_literal("increment"),
            Operation::Decrement => ByteString::from_literal("decrement"),
            Operation::Reset => ByteString::from_literal("reset"),
            Operation::Create => ByteString::from_literal("create"),
            Operation::Delete => ByteString::from_literal("delete"),
        }
    }
}

#[contract_impl]
impl Counter {
    /// Initialize the contract
    pub fn init() -> Self {
        Self {
            default_counter_key: ByteString::from_literal("default_counter"),
            counter_prefix: ByteString::from_literal("counter_"),
            owner_key: ByteString::from_literal("owner"),
            operators_prefix: ByteString::from_literal("op_"),
            total_operations_key: ByteString::from_literal("total_ops"),
            total_counters_key: ByteString::from_literal("total_counters"),
            last_operation_key: ByteString::from_literal("last_op"),
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

        // Verify the caller is authorized
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

    /// Add an operator (owner only)
    #[method]
    pub fn add_operator(&self, operator: H160) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can add operators"));
            return false;
        }

        let storage = Storage::get_context();
        let operator_key = self.operators_prefix.concat(&operator.into_byte_string());
        Storage::put(storage, operator_key, ByteString::from_literal("true"));

        let mut event_data = Array::new(); event_data.push(operator.into_any()); Runtime::notify(ByteString::from_literal("OperatorAdded"), event_data);
        true
    }

    /// Remove an operator (owner only)
    #[method]
    pub fn remove_operator(&self, operator: H160) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can remove operators"));
            return false;
        }

        let storage = Storage::get_context();
        let operator_key = self.operators_prefix.concat(&operator.into_byte_string());
        Storage::delete(storage, operator_key);

        let mut event_data = Array::new(); event_data.push(operator.into_any()); Runtime::notify(ByteString::from_literal("OperatorRemoved"), event_data);
        true
    }

    /// Check if an address is an operator
    #[method]
    #[safe]
    pub fn is_operator(&self, address: H160) -> bool {
        let storage = Storage::get_context();
        let operator_key = self.operators_prefix.concat(&address.into_byte_string());
        Storage::get(storage, operator_key).is_some()
    }

    /// Get the default counter value
    #[method]
    #[safe]
    pub fn get(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.default_counter_key.clone()) {
            Some(value_bytes) => Int256::from_byte_string(value_bytes),
            None => Int256::zero(),
        }
    }

    /// Get a named counter value
    #[method]
    #[safe]
    pub fn get_counter(&self, name: ByteString) -> Int256 {
        if !self.validate_counter_name(&name) {
            return Int256::zero();
        }

        let storage = Storage::get_context();
        let counter_key = self.counter_prefix.concat(&name);

        match Storage::get(storage, counter_key) {
            Some(value_bytes) => Int256::from_byte_string(value_bytes),
            None => Int256::zero(),
        }
    }

    /// Increment the default counter
    #[method]
    pub fn increment(&self) -> Int256 {
        if !self.is_authorized() {
            Runtime::log(ByteString::from_literal("Unauthorized: Not owner or operator"));
            return Int256::zero();
        }

        let storage = Storage::get_context();
        let current = self.get();
        let new_value = current.checked_add(&Int256::one());

        Storage::put(storage, self.default_counter_key.clone(), new_value.into_byte_string());

        self.record_operation(Operation::Increment, ByteString::from_literal("default"));
        let mut event_data = Array::new(); event_data.push(new_value.into_any()); Runtime::notify(ByteString::from_literal("CounterIncremented"), event_data);

        new_value
    }

    /// Increment a named counter
    #[method]
    pub fn increment_counter(&self, name: ByteString) -> Int256 {
        if !self.is_authorized() {
            Runtime::log(ByteString::from_literal("Unauthorized: Not owner or operator"));
            return Int256::zero();
        }

        if !self.validate_counter_name(&name) {
            return Int256::zero();
        }

        let storage = Storage::get_context();
        let counter_key = self.counter_prefix.concat(&name);
        let current = self.get_counter(name.clone());
        let new_value = current.checked_add(&Int256::one());

        Storage::put(storage, counter_key, new_value.into_byte_string());

        self.record_operation(Operation::Increment, name.clone());
        let mut event_data = Array::new(); event_data.push(name.into_any()); Runtime::notify(ByteString::from_literal("NamedCounterIncremented"), event_data);

        new_value
    }

    /// Decrement the default counter
    #[method]
    pub fn decrement(&self) -> Int256 {
        if !self.is_authorized() {
            Runtime::log(ByteString::from_literal("Unauthorized: Not owner or operator"));
            return Int256::zero();
        }

        let storage = Storage::get_context();
        let current = self.get();
        let new_value = current.checked_sub(&Int256::one());

        Storage::put(storage, self.default_counter_key.clone(), new_value.into_byte_string());

        self.record_operation(Operation::Decrement, ByteString::from_literal("default"));
        let mut event_data = Array::new(); event_data.push(new_value.into_any()); Runtime::notify(ByteString::from_literal("CounterDecremented"), event_data);

        new_value
    }

    /// Decrement a named counter
    #[method]
    pub fn decrement_counter(&self, name: ByteString) -> Int256 {
        if !self.is_authorized() {
            Runtime::log(ByteString::from_literal("Unauthorized: Not owner or operator"));
            return Int256::zero();
        }

        if !self.validate_counter_name(&name) {
            return Int256::zero();
        }

        let storage = Storage::get_context();
        let counter_key = self.counter_prefix.concat(&name);
        let current = self.get_counter(name.clone());
        let new_value = current.checked_sub(&Int256::one());

        Storage::put(storage, counter_key, new_value.into_byte_string());

        self.record_operation(Operation::Decrement, name.clone());
        let mut event_data = Array::new(); event_data.push(name.into_any()); Runtime::notify(ByteString::from_literal("NamedCounterDecremented"), event_data);

        new_value
    }

    /// Add a specific amount to the default counter
    #[method]
    pub fn add(&self, amount: Int256) -> Int256 {
        if !self.is_authorized() {
            Runtime::log(ByteString::from_literal("Unauthorized: Not owner or operator"));
            return Int256::zero();
        }

        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid amount: must be positive"));
            return self.get();
        }

        let storage = Storage::get_context();
        let current = self.get();
        let new_value = current.checked_add(&amount);

        Storage::put(storage, self.default_counter_key.clone(), new_value.into_byte_string());

        self.record_operation(Operation::Increment, ByteString::from_literal("default"));
        let mut event_data = Array::new(); event_data.push(amount.into_any()); Runtime::notify(ByteString::from_literal("CounterAdded"), event_data);

        new_value
    }

    /// Reset the default counter to zero (owner only)
    #[method]
    pub fn reset(&self) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can reset"));
            return false;
        }

        let storage = Storage::get_context();
        Storage::put(storage, self.default_counter_key.clone(), Int256::zero().into_byte_string());

        self.record_operation(Operation::Reset, ByteString::from_literal("default"));
        Runtime::notify(ByteString::from_literal("CounterReset"), Array::new());

        true
    }

    /// Reset a named counter to zero (owner only)
    #[method]
    pub fn reset_counter(&self, name: ByteString) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can reset"));
            return false;
        }

        if !self.validate_counter_name(&name) {
            return false;
        }

        let storage = Storage::get_context();
        let counter_key = self.counter_prefix.concat(&name);
        Storage::put(storage, counter_key, Int256::zero().into_byte_string());

        self.record_operation(Operation::Reset, name.clone());
        let mut event_data = Array::new(); event_data.push(name.into_any()); Runtime::notify(ByteString::from_literal("NamedCounterReset"), event_data);

        true
    }

    /// Create a new named counter
    #[method]
    pub fn create_counter(&self, name: ByteString, initial_value: Int256) -> bool {
        if !self.is_authorized() {
            Runtime::log(ByteString::from_literal("Unauthorized: Not owner or operator"));
            return false;
        }

        if !self.validate_counter_name(&name) {
            return false;
        }

        let storage = Storage::get_context();
        let counter_key = self.counter_prefix.concat(&name);

        // Check if counter already exists
        if Storage::get(storage.clone(), counter_key.clone()).is_some() {
            Runtime::log(ByteString::from_literal("Counter already exists"));
            return false;
        }

        Storage::put(storage.clone(), counter_key, initial_value.into_byte_string());

        // Update total counters count
        let total_counters = self.get_total_counters();
        let new_total = total_counters.checked_add(&Int256::one());
        Storage::put(storage, self.total_counters_key.clone(), new_total.into_byte_string());

        self.record_operation(Operation::Create, name.clone());
        let mut event_data = Array::new(); event_data.push(name.into_any()); Runtime::notify(ByteString::from_literal("CounterCreated"), event_data);

        true
    }

    /// Delete a named counter (owner only)
    #[method]
    pub fn delete_counter(&self, name: ByteString) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can delete"));
            return false;
        }

        if !self.validate_counter_name(&name) {
            return false;
        }

        let storage = Storage::get_context();
        let counter_key = self.counter_prefix.concat(&name);

        // Check if counter exists
        if Storage::get(storage.clone(), counter_key.clone()).is_none() {
            Runtime::log(ByteString::from_literal("Counter does not exist"));
            return false;
        }

        Storage::delete(storage.clone(), counter_key);

        // Update total counters count
        let total_counters = self.get_total_counters();
        let new_total = total_counters.checked_sub(&Int256::one());
        Storage::put(storage, self.total_counters_key.clone(), new_total.into_byte_string());

        self.record_operation(Operation::Delete, name.clone());
        let mut event_data = Array::new(); event_data.push(name.into_any()); Runtime::notify(ByteString::from_literal("CounterDeleted"), event_data);

        true
    }

    /// Get contract statistics
    #[method]
    #[safe]
    pub fn get_stats(&self) -> Map<ByteString, Any> {
        let mut stats = Map::new();

        stats.put(
            ByteString::from_literal("default_counter"),
            self.get().into_any()
        );
        stats.put(
            ByteString::from_literal("total_operations"),
            self.get_total_operations().into_any()
        );
        stats.put(
            ByteString::from_literal("total_counters"),
            self.get_total_counters().into_any()
        );
        stats.put(
            ByteString::from_literal("owner"),
            self.get_owner().into_any()
        );
        stats.put(
            ByteString::from_literal("last_operation_time"),
            Int256::new(self.get_last_operation_time() as i64).into_any()
        );

        stats
    }

    // Helper functions

    fn is_owner(&self) -> bool {
        let owner = self.get_owner();
        if owner == H160::zero() {
            return false;
        }
        Runtime::check_witness(owner)
    }

    fn is_authorized(&self) -> bool {
        if self.is_owner() {
            return true;
        }

        let caller = Runtime::get_calling_script_hash();
        self.is_operator(caller)
    }

    fn validate_counter_name(&self, name: &ByteString) -> bool {
        if name.is_empty() || name.len() > 32 {
            Runtime::log(ByteString::from_literal("Invalid counter name: must be 1-32 characters"));
            return false;
        }
        true
    }

    fn record_operation(&self, operation: Operation, counter_name: ByteString) {
        let storage = Storage::get_context();

        // Increment total operations
        let total_ops = self.get_total_operations();
        let new_total = total_ops.checked_add(&Int256::one());
        Storage::put(storage.clone(), self.total_operations_key.clone(), new_total.into_byte_string());

        // Record last operation time
        let timestamp = Runtime::get_time();
        Storage::put(storage, self.last_operation_key.clone(), ByteString::from_bytes(&timestamp.to_le_bytes()));

        // Log operation
        let log_message = ByteString::from_literal("Operation: ")
            .concat(&operation.to_string())
            .concat(&ByteString::from_literal(" on counter: "))
            .concat(&counter_name);
        Runtime::log(log_message);
    }

    fn get_total_operations(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.total_operations_key.clone()) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        }
    }

    fn get_total_counters(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.total_counters_key.clone()) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        }
    }

    fn get_last_operation_time(&self) -> u64 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.last_operation_key.clone()) {
            Some(time_bytes) => {
                let bytes = time_bytes.to_bytes();
                if bytes.len() >= 8 {
                    u64::from_le_bytes([
                        bytes[0], bytes[1], bytes[2], bytes[3],
                        bytes[4], bytes[5], bytes[6], bytes[7]
                    ])
                } else {
                    0
                }
            },
            None => 0,
        }
    }
}
