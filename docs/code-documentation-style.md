# Code Documentation Style Guide

This document outlines the recommended style for documenting code in the epicchain-contract-rs framework. Following these guidelines will ensure consistency across the codebase and enable optimal manifest generation.

## Documentation Comment Format

### Module Documentation

Each module should start with a documentation comment that describes its purpose:

```rust
//! # Contract Module
//! 
//! This module provides functionality for interacting with
//! other contracts on the EpicChain blockchain.
```

### Struct/Enum Documentation

Document structs and enums with a general description and details about their purpose:

```rust
/// A storage key used to access data in the contract's storage
/// 
/// Storage keys provide type-safe access to contract storage
/// with automatic serialization and deserialization.
pub struct StorageKey<T> {
    key: Vec<u8>,
    phantom: PhantomData<T>,
}
```

### Implementation Documentation

Document implementations, especially for `#[contract]` impl blocks:

```rust
/// TokenContract implements a standard NEP-17 token with
/// additional features for metadata management.
/// 
/// This contract provides the following functionality:
/// - Token transfers between accounts
/// - Balance queries
/// - Metadata storage and retrieval
#[contract]
impl TokenContract {
    // Implementation
}
```

### Method Documentation

Document each public method with:
1. A summary description
2. Parameter details
3. Return value explanation
4. Any side effects
5. Safety information (if applicable)

```rust
/// Transfers tokens from one account to another
/// 
/// This method verifies the sender's authorization using
/// Runtime.CheckWitness before performing the transfer.
/// 
/// # Parameters
/// * `from` - The account sending the tokens
/// * `to` - The account receiving the tokens
/// * `amount` - The number of tokens to transfer
/// * `data` - Optional data to include with the transfer
/// 
/// # Returns
/// `true` if the transfer was successful, `false` otherwise
/// 
/// # Side Effects
/// - Updates the balance of both accounts in storage
/// - Emits a "Transfer" event
/// - May call "onNEP17Payment" on the receiving contract
/// 
/// @safe - This indicates it's a read-only method (if applicable)
pub fn transfer(from: Address, to: Address, amount: u128, data: ByteString) -> bool {
    // Implementation
}
```

## Special Documentation Annotations

### @safe Annotation

Use the `@safe` annotation to mark methods that don't modify contract state:

```rust
/// Returns the balance of the specified account
/// 
/// # Parameters
/// * `account` - The account to query
/// 
/// # Returns
/// The account's token balance
/// 
/// @safe
pub fn balanceOf(account: Address) -> u128 {
    // Implementation
}
```

### @event Annotation

Document events with the `@event` annotation:

```rust
/// @event
/// Emitted when tokens are transferred between accounts
/// 
/// # Parameters
/// * `from` - The sender address (Hash160)
/// * `to` - The receiver address (Hash160)
/// * `amount` - The amount transferred (Integer)
pub fn Transfer(from: Address, to: Address, amount: u128) {
    // This is typically called by Runtime.notify
}
```

### @deprecated Annotation

Mark deprecated methods with the `@deprecated` annotation:

```rust
/// Returns the contract owner
/// 
/// @deprecated Use getOwner() instead
/// @safe
pub fn owner() -> Address {
    // Implementation
}
```

## Documentation Organization

### Constants Section

Document constants with a description of their purpose:

```rust
/// The token name as defined in the NEP-17 standard
const TOKEN_NAME: &'static str = "ExampleToken";

/// The token symbol as defined in the NEP-17 standard
const TOKEN_SYMBOL: &'static str = "EXT";

/// The number of decimal places for the token
const DECIMALS: u8 = 8;
```

### Method Grouping

Group related methods together with section comments:

```rust
//
// NEP-17 Standard Methods
//

/// Returns the token symbol
pub fn symbol() -> ByteString { /* ... */ }

/// Returns the number of decimal places
pub fn decimals() -> u8 { /* ... */ }

//
// Storage Methods
//

/// Stores a value in contract storage
pub fn store(key: ByteString, value: ByteString) { /* ... */ }

/// Retrieves a value from contract storage
pub fn retrieve(key: ByteString) -> ByteString { /* ... */ }
```

## Examples

Include examples in documentation when helpful:

```rust
/// Splits a string by the given delimiter
/// 
/// # Example
/// ```
/// let parts = split("a,b,c", ",");
/// assert_eq!(parts, ["a", "b", "c"]);
/// ```
pub fn split(s: ByteString, delimiter: ByteString) -> Vec<ByteString> {
    // Implementation
}
```

## Type Documentation

Document types in parameter and return documentation:

```rust
/// Retrieves the asset balance for the specified account
/// 
/// # Parameters
/// * `account` - The account address (Hash160)
/// * `assetId` - The asset ID (UInt256)
/// 
/// # Returns
/// The balance as a fixed8 value (Integer)
pub fn getBalance(account: Address, assetId: H256) -> u64 {
    // Implementation
}
```

## Best Practices

1. **Be Clear and Concise**: Documentation should be clear but not verbose
2. **Focus on Intent**: Explain why, not just what the code does
3. **Document Preconditions**: If a method requires certain conditions to be true
4. **Document Edge Cases**: Explain behavior for edge cases and error conditions
5. **Keep Documentation Updated**: Update docs when code changes
6. **Use Consistent Style**: Follow the same style throughout the codebase
7. **Avoid Redundancy**: Don't repeat information that's obvious from the code
8. **Document Public API**: Thoroughly document public methods and types
9. **Describe Effects**: Document side effects like storage modifications or events
10. **Use Markdown**: Take advantage of Markdown formatting for readability

## Examples of Well-Documented Code

### Example 1: Token Contract Method

```rust
/// Transfers tokens from one account to another
/// 
/// This method is required by the NEP-17 standard. It verifies
/// that the sender has authorized the transfer using Runtime.CheckWitness
/// and that the sender has sufficient balance.
/// 
/// # Parameters
/// * `from` - The account to transfer tokens from (Hash160)
/// * `to` - The account to transfer tokens to (Hash160)
/// * `amount` - The amount of tokens to transfer (Integer)
/// * `data` - Optional data to include with the transfer (Any)
/// 
/// # Returns
/// `true` if the transfer was successful, `false` otherwise
/// 
/// # Emits
/// * `Transfer` event with parameters (from, to, amount)
pub fn transfer(from: Address, to: Address, amount: u128, data: ByteString) -> bool {
    // Implementation
}
```

### Example 2: Storage Helper

```rust
/// Stores a value with an expiration time
/// 
/// This method stores a value in contract storage with an associated
/// expiration timestamp. Expired values will be treated as not existing
/// when retrieved with `get_with_expiry`.
/// 
/// # Parameters
/// * `key` - The storage key (ByteArray)
/// * `value` - The value to store (ByteArray)
/// * `expiration` - The expiration timestamp in milliseconds since epoch (Integer)
/// 
/// @safe
pub fn store_with_expiry(key: ByteString, value: ByteString, expiration: u64) {
    // Implementation
}
```

## Conclusion

Following a consistent documentation style:
- Improves code readability and maintainability
- Enables better automatic manifest generation
- Makes it easier for other developers to use your code
- Creates a more professional and polished codebase

Always document with the end user in mind, whether that's another developer, contract integrator, or future maintainer of your code.