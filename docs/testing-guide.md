# Testing Guide for EpicChain Rust Smart Contracts

This guide provides comprehensive strategies and patterns for testing EpicChain smart contracts written with the **EpicChain Rust Smart Contract Framework**.

## Overview

The framework provides **built-in testing support** with mock environments and testing utilities. All 13 examples include comprehensive tests that demonstrate best practices.

## Testing with the Framework

### Built-in Test Support

Every example includes a `make test` target:

```bash
# Run tests for any example
cd examples/04-nep17-token
make test               # Run all unit tests
make check              # Check code without building
```

### Framework Testing Features

The framework provides:
- **Mock Environment** - Simulated EpicChain runtime for testing
- **Storage Mocking** - Test storage operations without blockchain
- **Witness Simulation** - Test authorization and permissions
- **Event Testing** - Verify event emission and parameters

## Testing Approaches

### 1. Unit Testing with Mock Environment

For contracts that don't heavily rely on blockchain-specific features, you can create mock implementations of Neo's environment.

Create a testing module in your contract:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Mock storage implementation
    struct MockStorage {
        data: std::collections::HashMap<Vec<u8>, Vec<u8>>
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                data: std::collections::HashMap::new()
            }
        }

        fn put(&mut self, key: &[u8], value: &[u8]) {
            self.data.insert(key.to_vec(), value.to_vec());
        }

        fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
            self.data.get(key).cloned()
        }
    }

    #[test]
    fn test_contract_logic() {
        // Setup mock environment
        let mut storage = MockStorage::new();

        // Test contract logic
        // ...
    }
}
```

### 2. Integration Testing with EpicChain-Express

For full integration testing, you can use [EpicChain-Express](https://github.com/epicchainlabs/epicchain-express) to create a private EpicChain blockchain for testing.

1. Deploy your contract to EpicChain-Express
2. Interact with it using EpicChain SDK
3. Verify the results

### 3. Conditional Compilation for Testability

Restructure your contract to allow for testing:

```rust
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use epicchain_contract::prelude::*;

pub struct MyContract;

// Core logic separated from contract interface
impl MyContract {
    pub fn internal_logic(input: u32) -> u32 {
        // Pure logic that can be tested directly
        input * 2
    }
}

#[contract]
impl MyContract {
    pub fn my_method(input: u32) -> u32 {
        // Call internal logic
        MyContract::internal_logic(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_logic() {
        assert_eq!(MyContract::internal_logic(21), 42);
    }
}
```

### 4. Custom Test Harness

For more complex contracts, create a dedicated test harness:

```rust
#[cfg(test)]
mod test_utils {
    use epicchain_contract::prelude::*;

    pub struct TestEnvironment {
        pub storage: std::collections::HashMap<Vec<u8>, Vec<u8>>,
        pub witnesses: Vec<H160>,
        pub timestamp: u64,
        // Other environment variables
    }

    impl TestEnvironment {
        pub fn new() -> Self {
            Self {
                storage: std::collections::HashMap::new(),
                witnesses: Vec::new(),
                timestamp: 0,
                // Initialize other fields
            }
        }

        pub fn with_storage(mut self, key: &[u8], value: &[u8]) -> Self {
            self.storage.insert(key.to_vec(), value.to_vec());
            self
        }

        pub fn with_witness(mut self, account: H160) -> Self {
            self.witnesses.push(account);
            self
        }

        // Other builder methods
    }
}
```

## Testing NEP-17 Token Contracts

Here's a specific example for testing a NEP-17 token:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_decimals() {
        assert_eq!(MyToken::decimals(), 8);
    }

    #[test]
    fn test_token_symbol() {
        assert_eq!(MyToken::symbol().as_bytes(), b"MTK");
    }

    // More complex tests would require mocking the EpicChain environment
}
```

## Test Coverage Recommendations

Aim to test:

1. **Pure Functions**: Any function that doesn't interact with blockchain state
2. **State Transitions**: Contract state changes (using mock storage)
3. **Error Conditions**: Ensure proper handling of invalid inputs
4. **Access Control**: Verify authorization checks work correctly
5. **Edge Cases**: Test boundary conditions like zero values, empty strings

## Advanced Testing with EpicChain Blockchain Emulation

For advanced testing, consider:

1. Creating a mock EpicChain VM implementation
2. Using [epicchain-go](https://github.com/epicchainlabs/epicchain-go) for testing in a simulated environment
3. Creating specialized testing utilities for common patterns

## Continuous Integration

Set up CI for automatic testing:

```yaml
# .github/workflows/test.yml
name: Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    - name: Run Tests
      run: cargo test
```

## Debugging Tips

1. Use `println!` debugging in test mode (not available in contracts)
2. Implement detailed error types for easier debugging
3. Test state transitions step by step
4. Use EpicChain's debug tools with compiled contracts

## Working Examples

The framework includes comprehensive tests in all examples:

### NEP-17 Token Tests
```bash
cd examples/04-nep17-token
make test
# Tests: symbol(), decimals(), transfer(), balanceOf(), etc.
```

### NFT Contract Tests
```bash
cd examples/05-nep11-nft
make test
# Tests: mint(), transfer(), ownerOf(), tokenURI(), etc.
```

### Governance Tests
```bash
cd examples/11-governance
make test
# Tests: voting, proposals, execution, permissions
```

## Test Structure

Each example follows this testing pattern:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use epicchain_contract::testing::*;

    #[test]
    fn test_contract_deployment() {
        let contract = MyContract::init();
        // Test initialization
    }

    #[test]
    fn test_method_functionality() {
        let contract = MyContract::init();
        // Test specific methods
    }

    #[test]
    fn test_error_conditions() {
        // Test error handling
    }
}
```

## Best Practices

1. **Test All Public Methods** - Ensure complete coverage
2. **Test Error Conditions** - Verify proper error handling
3. **Test State Changes** - Verify storage operations
4. **Test Events** - Verify event emission
5. **Test Permissions** - Verify authorization checks

## Integration Testing

For full integration testing:

1. **Build Contract** - `make` to generate NEF and manifest
2. **Deploy to Testnet** - Use EpicChain-CLI or EpicChain-Express
3. **Test with SDK** - Use EpicChain SDK for integration tests
4. **Verify Results** - Check blockchain state and events

## Related Documentation

- **[Getting Started Guide](getting-started.md)** - Setup and first contract
- **[Contract Attributes](contract-attributes.md)** - Metadata and permissions
- **[Safe Methods](safe-methods.md)** - Read-only method patterns