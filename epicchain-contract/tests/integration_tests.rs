// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

//! Integration tests for the EpicChain Rust smart contract framework.

#![cfg(test)]

use epicchain_contract::prelude::*;
use epicchain_contract::types::{builtin::IntoAny, H256};

/// Integration test for basic contract functionality
#[test]
fn test_basic_contract_integration() {
    // Test basic contract structure
    #[contract_author("Basic Test")]
    #[contract_version("1.0.0")]
    pub struct BasicContract {
        name: ByteString,
    }

    #[contract_impl]
    impl BasicContract {
        pub fn init() -> Self {
            Self {
                name: ByteString::from_literal("BasicContract"),
            }
        }

        #[method]
        #[safe]
        pub fn get_name(&self) -> ByteString {
            self.name.clone()
        }

        #[method]
        #[safe]
        pub fn get_version(&self) -> ByteString {
            ByteString::from_literal("1.0.0")
        }

        #[method]
        #[safe]
        pub fn calculate_sum(&self, a: Int256, b: Int256) -> Int256 {
            a.checked_add(&b)
        }

        #[method]
        pub fn emit_event(&self, message: ByteString) {
            let event_name = ByteString::from_literal("TestEvent");
            let mut state = Array::<Any>::new();
            state.push(message.into_any());
            Runtime::notify(event_name, state);
        }
    }

    // Test contract instantiation
    let contract = BasicContract::init();
    assert!(!contract.name.is_empty());

    // Test safe methods
    let name = contract.get_name();
    assert!(!name.is_empty());

    let version = contract.get_version();
    assert!(!version.is_empty());

    // Test arithmetic
    let sum = contract.calculate_sum(Int256::one(), Int256::one());
    assert!(!sum.is_zero());

    // Test event emission
    let test_message = ByteString::from_literal("Hello World");
    contract.emit_event(test_message);
}

/// Integration test for type system functionality
#[test]
fn test_type_system_integration() {
    // Test Int256 operations
    let zero = Int256::zero();
    let one = Int256::one();
    let minus_one = Int256::minus_one();

    assert!(zero.is_zero());
    assert!(one.is_positive());
    assert!(minus_one.is_negative());

    let sum = one.checked_add(&one);
    assert!(!sum.is_zero());

    // Test H160 operations
    let h160_zero = H160::zero();
    let h160_bytes = h160_zero.to_bytes();
    assert_eq!(h160_bytes.len(), 20);

    // Test H256 operations
    let h256_zero = H256::zero();
    let h256_bytes = h256_zero.to_bytes();
    assert_eq!(h256_bytes.len(), 32);

    // Test ByteString operations
    let hello = ByteString::from_literal("Hello");
    let world = ByteString::from_literal("World");
    let hello_world = hello.concat(&world);
    assert!(!hello_world.is_empty());

    // Test PublicKey operations
    let key_bytes = [0x03u8; 33];
    let public_key = PublicKey::from_bytes(&key_bytes);
    let retrieved_bytes = public_key.to_bytes();
    assert_eq!(retrieved_bytes, key_bytes);
}

/// Integration test for runtime services
#[test]
fn test_runtime_services_integration() {
    // Test runtime service calls that work in test environment
    let trigger = Runtime::get_trigger();
    assert!(matches!(trigger, TriggerType::Application));

    let platform = Runtime::get_platform();
    assert_eq!(platform.to_bytes(), b"NEO");

    let network = Runtime::get_network();
    assert_eq!(network, 860833102); // Private net magic

    // Test witness checking (would fail in test environment)
    let test_account = H160::zero();
    let _has_witness = Runtime::check_witness(test_account);

    // Test logging
    let log_message = ByteString::from_literal("Test log message");
    Runtime::log(log_message);
}

/// Integration test for crypto operations
#[test]
fn test_crypto_integration() {
    // Test public key operations
    let key_bytes = [0x03u8; 33];
    let public_key = PublicKey::from_bytes(&key_bytes);
    let retrieved_bytes = public_key.to_bytes();
    assert_eq!(retrieved_bytes, key_bytes);

    // Test signature verification (with dummy data)
    let signature = ByteString::from_bytes(&[0x42u8; 64]);
    let result = Crypto::check_signature(public_key.clone(), signature);
    assert_eq!(result, false); // Expected to fail with dummy data

    // Test standard account creation
    let address = Contract::create_standard_account(public_key);
    assert_ne!(address, H160::zero());
}
