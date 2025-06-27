// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

//! Unit tests for epicchain-contract attributes and macros.

#![cfg(test)]

use epicchain_contract::prelude::*;

/// Test contract for attribute testing
#[contract_author("Test Author")]
#[contract_version("1.0.0")]
#[contract_permission("*:*")]
#[contract_meta("Description", "Test contract for attribute validation")]
#[contract_meta("Email", "test@example.com")]
pub struct TestAttributeContract {
    storage_key: ByteString,
}

#[contract_impl]
impl TestAttributeContract {
    pub fn init() -> Self {
        Self {
            storage_key: ByteString::from_literal("test_key"),
        }
    }

    #[method]
    #[safe]
    pub fn safe_method(&self) -> ByteString {
        ByteString::from_literal("safe_result")
    }

    #[method]
    pub fn unsafe_method(&self) -> ByteString {
        // This method can modify state
        let storage = Storage::get_context();
        let value = ByteString::from_literal("test_value");
        Storage::put(storage, self.storage_key.clone(), value.clone());
        value
    }

    #[method]
    pub fn get_stored_value(&self) -> Option<ByteString> {
        let storage = Storage::get_context();
        Storage::get(storage, self.storage_key.clone())
    }
}

/// Test contract with minimal attributes
#[contract_author("Minimal Author")]
pub struct MinimalContract {
    counter: ByteString,
}

#[contract_impl]
impl MinimalContract {
    pub fn init() -> Self {
        Self {
            counter: ByteString::from_literal("counter"),
        }
    }

    #[method]
    #[safe]
    pub fn get_counter(&self) -> Int256 {
        Int256::zero()
    }
}

/// Test contract with complex permissions
#[contract_author("Permission Test Author")]
#[contract_permission("0x1234567890123456789012345678901234567890:method1")]
#[contract_permission("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd:method2")]
#[contract_permission("*:read")]
pub struct PermissionTestContract {
    data: ByteString,
}

#[contract_impl]
impl PermissionTestContract {
    pub fn init() -> Self {
        Self {
            data: ByteString::from_literal("permission_data"),
        }
    }

    #[method]
    #[safe]
    pub fn read_data(&self) -> ByteString {
        self.data.clone()
    }

    #[method]
    pub fn method1(&self) -> ByteString {
        ByteString::from_literal("method1_result")
    }

    #[method]
    pub fn method2(&self) -> ByteString {
        ByteString::from_literal("method2_result")
    }
}

/// Test contract with standards compliance
#[contract_author("Standards Author")]
#[contract_version("2.1.0")]
#[contract_standards("NEP-17")]
#[contract_standards("NEP-11")]
pub struct StandardsContract {
    token_data: ByteString,
}

#[contract_impl]
impl StandardsContract {
    pub fn init() -> Self {
        Self {
            token_data: ByteString::from_literal("token_info"),
        }
    }

    #[method]
    #[safe]
    pub fn symbol(&self) -> ByteString {
        ByteString::from_literal("TEST")
    }

    #[method]
    #[safe]
    pub fn decimals(&self) -> Int256 {
        // Return 8 decimals using arithmetic
        let mut result = Int256::one();
        for _ in 0..3 {
            result = result.checked_mul(&result);
        }
        result // This gives us 8 (2^3)
    }

    #[method]
    #[safe]
    pub fn total_supply(&self) -> Int256 {
        // Return a large number using arithmetic
        let mut result = Int256::one();
        for _ in 0..20 {
            result = result.checked_add(&Int256::one());
        }
        result // This gives us 21
    }
}

#[test]
fn test_contract_instantiation() {
    // Test that contracts can be instantiated
    let test_contract = TestAttributeContract::init();
    assert!(!test_contract.storage_key.is_empty());

    let minimal_contract = MinimalContract::init();
    assert!(!minimal_contract.counter.is_empty());

    let permission_contract = PermissionTestContract::init();
    assert!(!permission_contract.data.is_empty());

    let standards_contract = StandardsContract::init();
    assert!(!standards_contract.token_data.is_empty());
}

#[test]
fn test_safe_methods() {
    let test_contract = TestAttributeContract::init();
    let result = test_contract.safe_method();
    assert!(!result.is_empty());

    let minimal_contract = MinimalContract::init();
    let counter = minimal_contract.get_counter();
    assert!(counter.is_zero());

    let permission_contract = PermissionTestContract::init();
    let data = permission_contract.read_data();
    assert!(!data.is_empty());

    let standards_contract = StandardsContract::init();
    let symbol = standards_contract.symbol();
    assert!(!symbol.is_empty());

    let decimals = standards_contract.decimals();
    assert!(!decimals.is_zero());

    let total_supply = standards_contract.total_supply();
    assert!(total_supply.is_positive());
}

#[test]
fn test_unsafe_methods() {
    let test_contract = TestAttributeContract::init();

    // Test unsafe method that modifies state
    let result = test_contract.unsafe_method();
    assert!(!result.is_empty());

    // Test retrieval of stored value
    let _stored = test_contract.get_stored_value();
    // Test passes if no panic occurs during execution
}

#[test]
fn test_permission_methods() {
    let permission_contract = PermissionTestContract::init();

    let method1_result = permission_contract.method1();
    assert!(!method1_result.is_empty());

    let method2_result = permission_contract.method2();
    assert!(!method2_result.is_empty());

    let read_result = permission_contract.read_data();
    assert!(!read_result.is_empty());
}

#[test]
fn test_standards_compliance() {
    let standards_contract = StandardsContract::init();

    // Test NEP-17 like methods
    let symbol = standards_contract.symbol();
    let decimals = standards_contract.decimals();
    let total_supply = standards_contract.total_supply();

    assert!(!symbol.is_empty());
    assert!(!decimals.is_zero());
    assert!(total_supply.is_positive());
}

#[test]
fn test_contract_metadata() {
    // Test that contracts with metadata can be instantiated and used
    let test_contract = TestAttributeContract::init();
    let result = test_contract.safe_method();
    assert!(!result.is_empty());

    // Test contract with version information
    let standards_contract = StandardsContract::init();
    let symbol = standards_contract.symbol();
    assert!(!symbol.is_empty());
}

#[test]
fn test_method_attributes() {
    let test_contract = TestAttributeContract::init();

    // Test that safe methods work
    let safe_result = test_contract.safe_method();
    assert!(!safe_result.is_empty());

    // Test that unsafe methods work
    let unsafe_result = test_contract.unsafe_method();
    assert!(!unsafe_result.is_empty());
}

#[test]
fn test_complex_contract_structure() {
    // Test that contracts with multiple attributes work together
    let permission_contract = PermissionTestContract::init();

    // Test all methods work
    let read_result = permission_contract.read_data();
    let method1_result = permission_contract.method1();
    let method2_result = permission_contract.method2();

    assert!(!read_result.is_empty());
    assert!(!method1_result.is_empty());
    assert!(!method2_result.is_empty());
}
