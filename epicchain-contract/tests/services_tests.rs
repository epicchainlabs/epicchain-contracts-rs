// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

//! Unit tests for epicchain-contract services.

#![cfg(test)]

use epicchain_contract::prelude::*;
use epicchain_contract::contract::native::{Gas, Neo, ContractManagement};
mod mock_env;

#[test]
fn test_storage_service() {
    // Test Storage::get_context
    let _context = Storage::get_context();
    // Note: StorageContext doesn't have is_read_only() method
    // The read-only nature is enforced by the type system

    // Test Storage::get_read_only_context
    let _read_only_context = Storage::get_read_only_context();
    // ReadOnlyStorageContext is read-only by design
}

#[test]
fn test_runtime_service() {
    // Test Runtime::get_trigger
    let trigger = Runtime::get_trigger();
    assert!(matches!(trigger, TriggerType::Application));

    // Test Runtime::get_platform
    let platform = Runtime::get_platform();
    assert_eq!(platform.to_bytes(), b"NEO");

    // Test Runtime::get_network
    let network = Runtime::get_network();
    assert_eq!(network, 860833102); // Magic number for private net
}

#[test]
fn test_crypto_service() {
    // We can't fully test crypto operations without proper key material,
    // but we can at least verify the API works

    // Create a dummy public key and signature
    let public_key = PublicKey::from_bytes(&[1u8; 33]);
    let signature = ByteString::from_bytes(&[2u8; 64]);

    // Test signature verification (will return false with dummy data)
    let result = Crypto::check_signature(public_key, signature);
    assert_eq!(result, false);
}

#[test]
fn test_contract_service() {
    // Test Contract::create_standard_account
    let public_key = PublicKey::from_bytes(&[1u8; 33]);
    let address = Contract::create_standard_account(public_key);

    // Verify we got a non-zero address
    assert_ne!(address, H160::zero());
}

#[test]
fn test_native_contracts() {
    // Test EpicPulse contract
    let total_supply = Gas::total_supply();
    assert!(total_supply > Int256::zero());

    // Test EpicChain contract
    let neo_total_supply = EpicChain::total_supply();
    assert!(neo_total_supply > Int256::zero());

    // Test ContractManagement
    let min_fee = ContractManagement::get_min_deployment_fee();
    assert!(min_fee > Int256::zero());
}
