// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

/// Attribute to mark a contract method as read-only (safe)
///
/// Methods marked with this attribute will be included in the manifest with `"safe": true`
/// meaning they do not modify contract state and can be safely called without incurring
/// additional fees.
///
/// # Example
///
/// ```rust
/// use epicchain_contract::prelude::*;
///
/// struct ExampleContract;
///
/// impl ExampleContract {
///     /// Returns the total supply without modifying state
///     #[safe]
///     pub fn total_supply() -> Int256 {
///         // Read-only operation
///         let storage = StorageMap::new();
///         let value = storage.get(ByteString::from_literal("total_supply"));
///         // Convert ByteString to Int256 or return zero
///         Int256::new(1000000) // Simplified for example
///     }
///
///     /// Modifies contract state - not marked as safe
///     pub fn mint(to: H160, amount: Int256) {
///         // This changes contract state
///         let mut storage = StorageMap::new();
///         // ... state modification code ...
///     }
/// }
/// ```
#[macro_export]
macro_rules! safe {
    () => {};
}

/// Attribute to mark a contract method as read-only (safe)
///
/// Use this attribute on methods that only read contract state and don't modify it.
/// These methods will be marked with `"safe": true` in the manifest.
pub use safe as safe;
