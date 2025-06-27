# Writing Efficient EpicChain Smart Contracts

This guide provides best practices for writing efficient smart contracts on the EpicChain blockchain using the **EpicChain Rust Smart Contract Framework**.

## Storage Efficiency

### Minimize Storage Operations

Storage operations are expensive on blockchains. Optimize your contract by:

1. **Batch Storage Updates**: Group multiple storage operations when possible
2. **Use Appropriate Storage Types**: Choose the smallest possible data type for your values
3. **Consider Cache Layers**: Use in-memory caching for frequently accessed values

Example of inefficient vs. efficient storage:

```rust
// Inefficient - Multiple storage operations
pub fn transfer(from: Address, to: Address, amount: u128) -> bool {
    let storage = Storage::new();

    let from_balance = storage.get::<_, u128>(&from_key).unwrap_or(0);
    storage.put(&from_key, &(from_balance - amount)); // First storage operation

    let to_balance = storage.get::<_, u128>(&to_key).unwrap_or(0);
    storage.put(&to_key, &(to_balance + amount)); // Second storage operation

    true
}

// Efficient - Batched operations
pub fn transfer(from: Address, to: Address, amount: u128) -> bool {
    let storage = Storage::new();

    // Read all data first
    let from_balance = storage.get::<_, u128>(&from_key).unwrap_or(0);
    let to_balance = storage.get::<_, u128>(&to_key).unwrap_or(0);

    // Perform computations
    let new_from_balance = from_balance - amount;
    let new_to_balance = to_balance + amount;

    // Write all data at once
    storage.put(&from_key, &new_from_balance);
    storage.put(&to_key, &new_to_balance);

    true
}
```

### Storage Key Design

Design efficient storage keys to minimize conflicts and improve organization:

```rust
// Good practice - Use prefixes for different data types
const BALANCE_PREFIX: &[u8] = b"balance:";
const ALLOWANCE_PREFIX: &[u8] = b"allowance:";
const METADATA_PREFIX: &[u8] = b"metadata:";

// Create keys with prefixes
let balance_key = [BALANCE_PREFIX, account.as_bytes()].concat();
```

## EpicPulse Optimization

### Method Safety

Mark read-only methods appropriately to save gas:

1. **Document Read-Only Methods**: Use naming conventions (`get_`, `query_`, etc.) or `@safe` annotations
2. **Follow Conventions**: Use standard method names from NEP specifications

```rust
/// Returns the token balance for the specified account
///
/// This method does not modify state and is marked as safe.
/// @safe
pub fn balanceOf(account: Address) -> u128 {
    // Read-only implementation
}
```

### Minimize Computation

Keep on-chain computations minimal:

1. **Precompute Values**: Calculate values off-chain when possible
2. **Simplify Algorithms**: Use efficient algorithms with minimal steps
3. **Use Fixed-Point Math**: Instead of floating point when possible

## Memory Management

### Stack vs. Heap

Be mindful of memory allocation:

1. **Prefer Stack Allocation**: Use fixed-size arrays when possible
2. **Minimize Vector Growth**: Pre-allocate vectors with known capacity
3. **Clean Up Unused Data**: Free memory when no longer needed

```rust
// Less efficient - Growable vector with potential reallocations
let mut data = Vec::new();
for i in 0..count {
    data.push(i);
}

// More efficient - Pre-allocated vector
let mut data = Vec::with_capacity(count);
for i in 0..count {
    data.push(i);
}
```

## Contract Interfacing

### Clear Method Signatures

Design clear method interfaces:

1. **Descriptive Parameter Names**: Use meaningful names that appear in the manifest
2. **Consistent Return Types**: Use consistent return types for similar operations
3. **Well-Documented Interfaces**: Add comprehensive documentation comments

```rust
/// Transfers tokens from one account to another
///
/// # Parameters
/// * `from` - The account to transfer tokens from
/// * `to` - The account to transfer tokens to
/// * `amount` - The amount of tokens to transfer
/// * `data` - Optional data to include with the transfer
///
/// # Returns
/// * `bool` - True if the transfer was successful, false otherwise
pub fn transfer(from: Address, to: Address, amount: u128, data: ByteString) -> bool {
    // Implementation
}
```

## Security Best Practices

### Input Validation

Always validate inputs:

```rust
pub fn transfer(from: Address, to: Address, amount: u128) -> bool {
    // Verify sender
    assert!(Runtime::check_witness(&from), "No authorization");

    // Check amount
    if amount == 0 {
        return true;
    }

    // Check recipient is not null address
    if to == Address::from([0u8; 20]) {
        return false;
    }

    // Further implementation
}
```

### Overflow Prevention

Guard against integer overflows:

```rust
// Unsafe - Potential overflow
let new_balance = balance + amount;

// Safe - Checks for overflow
let new_balance = balance.checked_add(amount).unwrap_or_else(|| {
    panic!("Transfer would overflow recipient balance");
});
```

## Testing

### Unit Tests

Write comprehensive tests for all contract functions:

```rust
#[test]
fn test_transfer() {
    // Set up test environment
    let owner = Address::from([1u8; 20]);
    let recipient = Address::from([2u8; 20]);

    // Mock Runtime.CheckWitness to return true for owner
    // Test the transfer functionality
    // Assert the expected state changes
}
```

## Manifest Optimization

### Standard Compliance

Implement standards fully to enable proper manifest generation:

1. **NEP-17 for Fungible Tokens**: Implement all required methods with correct signatures
2. **NEP-11 for Non-Fungible Tokens**: Follow the standard interface precisely

### Method Documentation

Document methods comprehensively for better manifest generation:

```rust
/// Returns the symbol of the token
///
/// This method returns the token's symbol as a ByteString.
/// The symbol is a short string like "XPR" that represents the token.
///
/// # Returns
/// A ByteString containing the token's symbol
pub fn symbol() -> ByteString {
    "TOKEN".into()
}
```

## Performance Monitoring

### EpicPulse Usage Analysis

Monitor and optimize gas usage:

1. **Test EpicPulse Consumption**: Measure gas usage for critical operations
2. **Profile Hot Paths**: Identify and optimize frequently executed code
3. **Benchmark Alternatives**: Compare different implementation approaches

## Conclusion

Efficient EpicChain smart contracts combine:

1. **Optimized Storage**: Minimize and batch storage operations
2. **EpicPulse Efficiency**: Reduce computation and mark read-only methods
3. **Clear Interfaces**: Design intuitive, well-documented APIs
4. **Comprehensive Testing**: Verify correctness and efficiency
5. **Good Documentation**: Enable accurate manifest generation

By following these practices, your contracts will be more efficient, secure, and easier to maintain.

## Working Examples

The framework includes examples that demonstrate efficient contract patterns:

### Efficient Token Implementation
```bash
cd examples/04-nep17-token
cat src/lib.rs           # See efficient NEP-17 implementation
make                     # Build optimized contract
```

### Optimized NFT Contract
```bash
cd examples/05-nep11-nft
cat src/lib.rs           # See efficient NFT implementation
make test               # Run performance tests
```

### EpicPulse-Optimized DEX
```bash
cd examples/09-simple-dex
cat src/lib.rs           # See efficient DEX implementation
make                     # Build with optimizations
```

## Build Optimizations

The framework includes build optimizations in all Makefiles:

```makefile
# Optimized build flags for production
RUST_FLAGS = "-Ctarget-feature=+multivalue \
    -Cllvm-args=--combiner-store-merging=false \
    -Clink-arg=--initial-memory=262144 \
    -Clink-arg=-zstack-size=131072"
```

These optimizations:
- **Reduce WASM size** - Smaller contracts use less gas
- **Optimize memory usage** - Efficient stack and heap management
- **Enable WASM features** - Use advanced WASM capabilities

## Related Documentation

- **[Getting Started Guide](getting-started.md)** - Framework setup and basics
- **[Contract Attributes](contract-attributes.md)** - Metadata and permissions
- **[Safe Methods](safe-methods.md)** - Read-only method patterns
- **[Testing Guide](testing-guide.md)** - Performance testing strategies