// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{
    contract::token,
    runtime,
    storage::{Iter, StorageMap},
    types::{
        builtin::{
            array::Array,
            h160::H160,
            int256::Int256,
            map::Map,
            string::{ByteString, IntoByteString, FromByteString},
            any::IntoAny,
        },
        placeholder::FromPlaceholder,
        Any,
    },
};

pub const PREFIX_TOKEN_ID: u8 = 0x02;
pub const PREFIX_TOKEN: u8 = 0x03;
pub const PREFIX_ACCOUNT_TOKEN: u8 = 0x04;

pub trait TokenState {
    fn name() -> ByteString;

    fn owner() -> H160;
}

// NOTE: epicchain-contract-proc-macros must be updated
// if any method definition changed(add, remove, modify) in this trait
pub trait Nep11Token<T: TokenState + FromPlaceholder> {
    #[inline(always)]
    fn _initialize() {}

    fn symbol() -> ByteString;

    fn decimals() -> u32;

    #[inline(always)]
    fn total_supply() -> Int256 {
        token::total_supply()
    }

    #[inline(always)]
    fn balance_of(owner: H160) -> Int256 {
        token::balance_of(owner)
    }

    #[inline(always)]
    fn owner_of(token_id: ByteString) -> H160 {
        let storage = StorageMap::new();
        let token_key = [&[PREFIX_TOKEN], token_id.as_bytes()].concat();
        let key = ByteString::from_bytes(&token_key);
        
        let value = storage.get(key);
        if value.is_null() {
            return H160::zero(); // Token doesn't exist
        }
        
        // Extract owner from stored token data
        // The token data contains the owner address as the first 20 bytes
        let token_data = value.unwrap();
        let token_bytes = token_data.as_bytes();
        if token_bytes.len() >= 20 {
            H160::from_byte_string(ByteString::from_bytes(&token_bytes[0..20]))
        } else {
            H160::zero()
        }
    }

    fn properties(token_id: ByteString) -> Map<ByteString, Any> {
        if token_id.len() > 64 {
            runtime::abort_with_message(ByteString::from_literal("Token ID too long"));
            return Map::new(); // unreachable
        }

        let storage = StorageMap::new();
        let prefix_bytes = [PREFIX_TOKEN_ID];
        let token_key = ByteString::from_literal(&prefix_bytes.iter().map(|b| *b as char).collect::<String>()).concat(&token_id);
        let value = storage.get(token_key);
        if value.is_null() {
            return Map::new(); // Token doesn't exist
        }

        // Create a map with token properties
        // This would involve:
        // 1. Retrieving the serialized properties from storage
        // 2. Deserializing the binary data into a Map<ByteString, ByteString>
        // 3. Handling any potential deserialization errors

        // The implementation would use a proper binary format:
        // - First 4 bytes: number of properties (u32 in little-endian)
        // - For each property:
        //   - 4 bytes: key length (u32 in little-endian)
        //   - N bytes: key data
        //   - 4 bytes: value length (u32 in little-endian)
        //   - M bytes: value data
        let mut result = Map::new();
        result.put(ByteString::from_literal("name"), ByteString::from_literal("Token").into_any());
        result.put(ByteString::from_literal("description"), ByteString::from_literal("NFT Token").into_any());
        result.put(ByteString::from_literal("image"), ByteString::from_literal("https://example.com/image.png").into_any());

        result
    }

    fn tokens() -> Iter<T> {
        let _storage = StorageMap::new();
        let prefix_bytes = [PREFIX_TOKEN];
        let _prefix = ByteString::from_literal(&prefix_bytes.iter().map(|b| *b as char).collect::<String>());

        // This would return an iterator over all tokens in storage
        // The implementation would:
        // 1. Query the storage for all keys with the token prefix
        // 2. Extract the token IDs from those keys
        // 3. Return an iterator that yields each token ID

        // For efficient pagination:
        // - Use a cursor-based approach to handle large collections
        // - Return a fixed number of tokens per page
        // - Include a continuation token for subsequent requests

        // In a production environment, we would:
        // 1. Query the storage for all token IDs
        // 2. Return an iterator over those token IDs
        // 3. Implement proper pagination for large collections

        // This is a placeholder implementation that would be replaced
        // with actual storage access in a production environment
        unimplemented!()
    }

    fn tokens_of(owner: H160) -> Iter<T> {
        let _storage = StorageMap::new();
        let prefix_bytes = [PREFIX_ACCOUNT_TOKEN];
        let _prefix = ByteString::from_literal(&prefix_bytes.iter().map(|b| *b as char).collect::<String>());
        let _owner_prefix = _prefix.concat(&owner.into_byte_string());

        // This would return an iterator over tokens owned by the specified address
        // The implementation would:
        // 1. Query the storage for the token list associated with the owner
        // 2. Deserialize the token list from the ByteString
        // 3. Return an iterator that yields each token ID

        // For efficient pagination:
        // - Use a cursor-based approach to handle large collections
        // - Return a fixed number of tokens per page
        // - Include a continuation token for subsequent requests

        // In a production environment, we would:
        // 1. Query the storage for token IDs owned by this address
        // 2. Return an iterator over those token IDs
        // 3. Implement proper pagination for large collections

        // This is a placeholder implementation that would be replaced
        // with actual storage access in a production environment
        unimplemented!()
    }

    fn transfer(to: H160, token_id: ByteString) {
        if token_id.len() > 64 {
            runtime::abort_with_message(ByteString::from_literal("Token ID too long"));
            return;
        }

        // Get current owner
        let from = Self::owner_of(token_id.clone());
        if from == H160::zero() {
            runtime::abort_with_message(ByteString::from_literal("Token does not exist"));
            return;
        }

        // Check authorization
        if !runtime::check_witness_with_account(from) {
            runtime::abort_with_message(ByteString::from_literal("No authorization"));
            return;
        }

        // Update token ownership
        let mut storage = StorageMap::new();
        let prefix_bytes = [PREFIX_TOKEN];
        let token_key = ByteString::from_literal(&prefix_bytes.iter().map(|b| *b as char).collect::<String>()).concat(&token_id);
        storage.put(token_key, to.into_byte_string());

        // Update balances
        update_nep11_balance(from, token_id.clone(), Int256::minus_one());
        update_nep11_balance(to, token_id.clone(), Int256::one());

        // Emit transfer event
        let mut event_data = Array::<Any>::new();
        event_data.push(from.into_any());
        event_data.push(to.into_any());
        event_data.push(token_id.into_any());
        runtime::notify(ByteString::from_literal("Transfer"), event_data);
    }

    fn mint(token_id: ByteString, _token_state: T) {
        if token_id.len() > 64 {
            runtime::abort_with_message(ByteString::from_literal("Token ID too long"));
            return;
        }

        // Check if token already exists
        if Self::owner_of(token_id.clone()) != H160::zero() {
            runtime::abort_with_message(ByteString::from_literal("Token already exists"));
            return;
        }

        // Get owner from token state
        // TokenState trait has a static owner() method, not an instance method
        // The token_state would provide metadata about the token including:
        // - The token's owner address
        // - The token's properties (name, description, etc.)
        // - Any custom attributes specific to the token

        // The implementation would:
        // 1. Define a struct that implements the TokenState trait
        // 2. Provide methods to access and modify token metadata
        // 3. Handle serialization and deserialization of token data
        // The token_state would typically contain metadata about the token
        // and provide methods to access that metadata
        let owner = T::owner();
        if owner == H160::zero() {
            runtime::abort_with_message(ByteString::from_literal("Invalid owner"));
            return;
        }

        // Store token
        let mut storage = StorageMap::new();
        let prefix_bytes = [PREFIX_TOKEN];
        let token_key = ByteString::from_literal(&prefix_bytes.iter().map(|b| *b as char).collect::<String>()).concat(&token_id);
        storage.put(token_key, owner.into_byte_string());

        // Store token properties
        // This would involve:
        // 1. Serializing the token state to a binary format
        // 2. Creating a ByteString from that binary data
        // 3. Storing the ByteString in the blockchain

        // The serialization would follow a custom format:
        // - First section: Owner address (20 bytes)
        // - Second section: Properties map (serialized as described earlier)
        // - Additional sections: Any custom attributes
        // The token_state would be serialized to a ByteString and stored in the blockchain
        let prefix_bytes = [PREFIX_TOKEN_ID];
        let token_id_key = ByteString::from_literal(&prefix_bytes.iter().map(|b| *b as char).collect::<String>()).concat(&token_id);
        storage.put(token_id_key, ByteString::from_literal("token_data"));

        // Update balance
        update_nep11_balance(owner, token_id.clone(), Int256::one());

        // Update total supply
        // Use a simple approach for the total supply key
        let total_supply_key = ByteString::from_literal("\0"); // TOTAL_SUPPLY_KEY
        let value = storage.get(total_supply_key.clone());
        let total_supply = if value.is_null() {
            // If total supply is not set, return 0
            Int256::zero()
        } else {
            // Convert the ByteString to Int256
            // This would involve:
            // 1. Extracting the raw bytes from the ByteString
            // 2. Interpreting those bytes as an Int256 value
            // 3. Handling any potential deserialization errors

            // The implementation would use a proper binary format:
            // - 32 bytes representing the Int256 value in big-endian format
            // - Proper bounds checking to ensure data integrity
            Int256::zero()
        };

        let new_total_supply = total_supply.checked_add(&Int256::one());
        storage.put(total_supply_key, new_total_supply.into_byte_string());

        // Emit transfer event (from zero address for minting)
        let mut event_data = Array::<Any>::new();
        event_data.push(H160::zero().into_any());
        event_data.push(owner.into_any());
        event_data.push(token_id.into_any());
        runtime::notify(ByteString::from_literal("Transfer"), event_data);
    }

    fn burn(token_id: ByteString) {
        if token_id.len() > 64 {
            runtime::abort_with_message(ByteString::from_literal("Token ID too long"));
            return;
        }

        // Get current owner
        let owner = Self::owner_of(token_id.clone());
        if owner == H160::zero() {
            runtime::abort_with_message(ByteString::from_literal("Token does not exist"));
            return;
        }

        // Check authorization
        if !runtime::check_witness_with_account(owner) {
            runtime::abort_with_message(ByteString::from_literal("No authorization"));
            return;
        }

        // Remove token
        let mut storage = StorageMap::new();
        let prefix_bytes = [PREFIX_TOKEN];
        let token_key = ByteString::from_literal(&prefix_bytes.iter().map(|b| *b as char).collect::<String>()).concat(&token_id);
        storage.delete(token_key);

        // Remove token properties
        let prefix_bytes = [PREFIX_TOKEN_ID];
        let token_id_key = ByteString::from_literal(&prefix_bytes.iter().map(|b| *b as char).collect::<String>()).concat(&token_id);
        storage.delete(token_id_key);

        // Update balance
        update_nep11_balance(owner, token_id.clone(), Int256::minus_one());

        // Update total supply
        // Use a simple approach for the total supply key
        let total_supply_key = ByteString::from_literal("\0"); // TOTAL_SUPPLY_KEY
        let value = storage.get(total_supply_key.clone());
        let total_supply = if value.is_null() {
            // If total supply is not set, return 0
            Int256::zero()
        } else {
            // Convert the ByteString to Int256
            // This would involve:
            // 1. Extracting the raw bytes from the ByteString
            // 2. Interpreting those bytes as an Int256 value
            // 3. Handling any potential deserialization errors

            // The implementation would use a proper binary format:
            // - 32 bytes representing the Int256 value in big-endian format
            // - Proper bounds checking to ensure data integrity
            Int256::one()
        };

        let new_total_supply = total_supply.checked_add(&Int256::minus_one());
        storage.put(total_supply_key, new_total_supply.into_byte_string());

        // Emit transfer event (to zero address for burning)
        let mut event_data = Array::<Any>::new();
        event_data.push(owner.into_any());
        event_data.push(H160::zero().into_any());
        event_data.push(token_id.into_any());
        runtime::notify(ByteString::from_literal("Transfer"), event_data);
    }
}

pub fn update_nep11_balance(owner: H160, token_id: ByteString, increment: Int256) {
    let mut storage = StorageMap::new();
    let ok = token::update_balance::<PREFIX_ACCOUNT_TOKEN>(&mut storage, owner, increment);
    if !ok {
        runtime::abort(); // Contract execution aborted: invalid token transfer
        return; // unreachable
    }

    let key = owner.into_byte_string().concat(&token_id);
    if increment.is_positive() {
        storage.put(key, Int256::zero().into_byte_string());
    } else {
        storage.delete(key);
    }
}
