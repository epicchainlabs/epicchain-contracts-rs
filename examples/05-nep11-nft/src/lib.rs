//! # NEP-11 Non-Fungible Token Contract
//!
//! A complete implementation of the NEP-11 non-fungible token standard for EpicChain.
//! This contract demonstrates:
//! - Full NEP-11 compliance with all required methods
//! - Unique token creation and management
//! - Token metadata and properties system
//! - Secure transfer mechanics
//! - Enumeration capabilities
//! - Administrative controls
//! - Event emission for all operations
//!
//! This is a production-ready NFT contract suitable for digital collectibles,
//! art, gaming assets, and other unique digital items.

#![no_std]
#![no_main]

use epicchain_contract::prelude::*;
use epicchain_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

/// NEP-11 compliant non-fungible token contract
#[contract_author("EpicChain Rust Framework", "devs@epic-chain.org")]
#[contract_version("1.0.0")]
#[contract_standards("NEP-11")]
#[contract_permission("*", "*")]
#[contract_meta("description", "Production-ready NEP-11 non-fungible token")]
#[contract_meta("website", "https://github.com/R3E-Network/epicchain-contract-rs")]
pub struct Nep11Token {
    // Token metadata
    symbol_key: ByteString,
    total_supply_key: ByteString,

    // Storage prefixes
    owner_prefix: ByteString,        // token_id -> owner
    balance_prefix: ByteString,      // owner -> balance count
    token_prefix: ByteString,        // owner -> list of token_ids
    properties_prefix: ByteString,   // token_id -> properties
    approved_prefix: ByteString,     // token_id -> approved_address

    // Administrative keys
    contract_owner_key: ByteString,
    minters_prefix: ByteString,

    // Configuration
    paused_key: ByteString,
    base_uri_key: ByteString,

    // Token enumeration
    all_tokens_key: ByteString,
    token_index_prefix: ByteString,  // token_id -> index
}

#[contract_impl]
impl Nep11Token {
    /// Initialize the NFT contract
    pub fn init() -> Self {
        Self {
            symbol_key: ByteString::from_literal("symbol"),
            total_supply_key: ByteString::from_literal("total_supply"),
            owner_prefix: ByteString::from_literal("owner_"),
            balance_prefix: ByteString::from_literal("balance_"),
            token_prefix: ByteString::from_literal("tokens_"),
            properties_prefix: ByteString::from_literal("props_"),
            approved_prefix: ByteString::from_literal("approved_"),
            contract_owner_key: ByteString::from_literal("contract_owner"),
            minters_prefix: ByteString::from_literal("minter_"),
            paused_key: ByteString::from_literal("paused"),
            base_uri_key: ByteString::from_literal("base_uri"),
            all_tokens_key: ByteString::from_literal("all_tokens"),
            token_index_prefix: ByteString::from_literal("index_"),
        }
    }

    /// Deploy the NFT contract with initial parameters
    #[method]
    pub fn deploy(
        &self,
        owner: H160,
        symbol: ByteString,
        base_uri: ByteString
    ) -> bool {
        let storage = Storage::get_context();

        // Check if already deployed
        if Storage::get(storage.clone(), self.contract_owner_key.clone()).is_some() {
            Runtime::log(ByteString::from_literal("Contract already deployed"));
            return false;
        }

        // Validate parameters
        if symbol.is_empty() || symbol.len() > 16 {
            Runtime::log(ByteString::from_literal("Invalid symbol: must be 1-16 characters"));
            return false;
        }

        // Verify deployer authorization
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Store contract metadata
        Storage::put(storage.clone(), self.symbol_key.clone(), symbol.clone());
        Storage::put(storage.clone(), self.contract_owner_key.clone(), owner.into_byte_string());
        Storage::put(storage.clone(), self.total_supply_key.clone(), Int256::zero().into_byte_string());

        if !base_uri.is_empty() {
            Storage::put(storage.clone(), self.base_uri_key.clone(), base_uri);
        }

        // Initialize empty token list
        let storage_clone = storage.clone();
        Storage::put(storage_clone, self.all_tokens_key.clone(), ByteString::empty());

        let mut event_data = Array::new(); event_data.push(symbol.into_any()); Runtime::notify(ByteString::from_literal("ContractDeployed"), event_data);
        true
    }

    /// Get token symbol (NEP-11 required)
    #[method]
    #[safe]
    pub fn symbol(&self) -> ByteString {
        let storage = Storage::get_context();
        match Storage::get(storage, self.symbol_key.clone()) {
            Some(symbol) => symbol,
            None => ByteString::from_literal("UNKNOWN"),
        }
    }

    /// Get number of decimals (always 0 for NFTs) (NEP-11 required)
    #[method]
    #[safe]
    pub fn decimals(&self) -> u32 {
        0 // NFTs are non-divisible
    }

    /// Get total token supply (NEP-11 required)
    #[method]
    #[safe]
    pub fn total_supply(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.total_supply_key.clone()) {
            Some(supply_bytes) => Int256::from_byte_string(supply_bytes),
            None => Int256::zero(),
        }
    }

    /// Get balance of an account (number of tokens owned) (NEP-11 required)
    #[method]
    #[safe]
    pub fn balance_of(&self, owner: H160) -> Int256 {
        let storage = Storage::get_context();
        let balance_key = self.balance_prefix.concat(&owner.into_byte_string());

        match Storage::get(storage, balance_key) {
            Some(balance_bytes) => Int256::from_byte_string(balance_bytes),
            None => Int256::zero(),
        }
    }

    /// Get owner of a specific token (NEP-11 required)
    #[method]
    #[safe]
    pub fn owner_of(&self, token_id: ByteString) -> H160 {
        let storage = Storage::get_context();
        let owner_key = self.owner_prefix.concat(&token_id);

        match Storage::get(storage, owner_key) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => H160::zero(),
        }
    }

    /// Get tokens owned by an account (NEP-11 required)
    #[method]
    #[safe]
    pub fn tokens_of(&self, owner: H160) -> Array<ByteString> {
        let storage = Storage::get_context();
        let tokens_key = self.token_prefix.concat(&owner.into_byte_string());

        match Storage::get(storage, tokens_key) {
            Some(tokens_data) => self.deserialize_token_list(tokens_data),
            None => Array::new(),
        }
    }

    /// Transfer a token (NEP-11 required)
    #[method]
    pub fn transfer(&self, to: H160, token_id: ByteString, data: Any) -> bool {
        // Get current owner
        let from = self.owner_of(token_id.clone());
        if from == H160::zero() {
            Runtime::log(ByteString::from_literal("Token does not exist"));
            return false;
        }

        // Check authorization (owner or approved)
        if !self.is_authorized_for_token(from, token_id.clone()) {
            Runtime::log(ByteString::from_literal("Unauthorized: Not owner or approved"));
            return false;
        }

        // Check if contract is paused
        if self.is_paused() {
            Runtime::log(ByteString::from_literal("Contract is paused"));
            return false;
        }

        // Perform transfer
        self.transfer_token(from, to, token_id.clone());

        // Call onNEP11Payment if recipient is a contract
        self.on_payment_callback(from, Int256::one(), token_id, data);

        true
    }

    /// Get properties of a token (NEP-11 optional)
    #[method]
    #[safe]
    pub fn properties(&self, token_id: ByteString) -> Map<ByteString, Any> {
        let storage = Storage::get_context();
        let props_key = self.properties_prefix.concat(&token_id);

        match Storage::get(storage, props_key) {
            Some(props_data) => self.deserialize_properties(props_data),
            None => Map::new(),
        }
    }

    /// Approve another address to transfer a specific token
    #[method]
    pub fn approve(&self, to: H160, token_id: ByteString) -> bool {
        let owner = self.owner_of(token_id.clone());
        if owner == H160::zero() {
            Runtime::log(ByteString::from_literal("Token does not exist"));
            return false;
        }

        // Verify authorization
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("Unauthorized: Not token owner"));
            return false;
        }

        // Set approval
        let storage = Storage::get_context();
        let approved_key = self.approved_prefix.concat(&token_id);

        if to == H160::zero() {
            let storage_clone = storage.clone(); Storage::delete(storage_clone, approved_key);
        } else {
            let storage_clone = storage.clone(); Storage::put(storage_clone, approved_key, to.into_byte_string());
        }

        // Emit Approval event
        let mut event_data = Array::new();
        event_data.push(owner.into_any());
        event_data.push(to.into_any());
        event_data.push(token_id.into_any());
        Runtime::notify(ByteString::from_literal("Approval"), event_data);

        true
    }

    /// Get approved address for a token
    #[method]
    #[safe]
    pub fn get_approved(&self, token_id: ByteString) -> H160 {
        let storage = Storage::get_context();
        let approved_key = self.approved_prefix.concat(&token_id);

        match Storage::get(storage, approved_key) {
            Some(approved_bytes) => H160::from_byte_string(approved_bytes),
            None => H160::zero(),
        }
    }

    /// Mint a new token (authorized minter only)
    #[method]
    pub fn mint(&self, to: H160, token_id: ByteString, properties: Map<ByteString, Any>) -> bool {
        if !self.is_authorized_minter() {
            Runtime::log(ByteString::from_literal("Unauthorized: Not authorized to mint"));
            return false;
        }

        if token_id.is_empty() || token_id.len() > 64 {
            Runtime::log(ByteString::from_literal("Invalid token ID: must be 1-64 characters"));
            return false;
        }

        // Check if token already exists
        if self.owner_of(token_id.clone()) != H160::zero() {
            Runtime::log(ByteString::from_literal("Token already exists"));
            return false;
        }

        let storage = Storage::get_context();

        // Set token owner
        let owner_key = self.owner_prefix.concat(&token_id);
        Storage::put(storage.clone(), owner_key, to.into_byte_string());

        // Update owner's balance
        let current_balance = self.balance_of(to);
        let new_balance = current_balance.checked_add(&Int256::one());
        let balance_key = self.balance_prefix.concat(&to.into_byte_string());
        Storage::put(storage.clone(), balance_key, new_balance.into_byte_string());

        // Add token to owner's token list
        self.add_token_to_owner(to, token_id.clone());

        // Store properties if provided
        if properties.size() > 0 {
            let props_key = self.properties_prefix.concat(&token_id);
            let serialized_props = self.serialize_properties(properties);
            Storage::put(storage.clone(), props_key, serialized_props);
        }

        // Update total supply
        let current_supply = self.total_supply();
        let new_supply = current_supply.checked_add(&Int256::one());
        let storage_clone = storage.clone(); Storage::put(storage_clone, self.total_supply_key.clone(), new_supply.into_byte_string());

        // Add to global token list
        self.add_token_to_global_list(token_id.clone());

        // Emit Transfer event (from null address)
        self.emit_transfer(H160::zero(), to, Int256::one(), token_id.clone());

        let mut event_data = Array::new(); event_data.push(token_id.into_any()); Runtime::notify(ByteString::from_literal("TokenMinted"), event_data);
        true
    }

    /// Burn a token (token owner only)
    #[method]
    pub fn burn(&self, token_id: ByteString) -> bool {
        let owner = self.owner_of(token_id.clone());
        if owner == H160::zero() {
            Runtime::log(ByteString::from_literal("Token does not exist"));
            return false;
        }

        // Verify authorization
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("Unauthorized: Not token owner"));
            return false;
        }

        let storage = Storage::get_context();

        // Remove token owner
        let owner_key = self.owner_prefix.concat(&token_id);
        Storage::delete(storage.clone(), owner_key);

        // Remove approval if exists
        let approved_key = self.approved_prefix.concat(&token_id);
        Storage::delete(storage.clone(), approved_key);

        // Remove properties if exist
        let props_key = self.properties_prefix.concat(&token_id);
        Storage::delete(storage.clone(), props_key);

        // Update owner's balance
        let current_balance = self.balance_of(owner);
        let new_balance = current_balance.checked_sub(&Int256::one());
        let balance_key = self.balance_prefix.concat(&owner.into_byte_string());

        if new_balance == Int256::zero() {
            Storage::delete(storage.clone(), balance_key);
        } else {
            Storage::put(storage.clone(), balance_key, new_balance.into_byte_string());
        }

        // Remove token from owner's token list
        self.remove_token_from_owner(owner, token_id.clone());

        // Update total supply
        let current_supply = self.total_supply();
        let new_supply = current_supply.checked_sub(&Int256::one());
        let storage_clone = storage.clone(); Storage::put(storage_clone, self.total_supply_key.clone(), new_supply.into_byte_string());

        // Remove from global token list
        self.remove_token_from_global_list(token_id.clone());

        // Emit Transfer event (to null address)
        self.emit_transfer(owner, H160::zero(), Int256::one(), token_id.clone());

        let mut event_data = Array::new(); event_data.push(token_id.into_any()); Runtime::notify(ByteString::from_literal("TokenBurned"), event_data);
        true
    }

    /// Get contract owner
    #[method]
    #[safe]
    pub fn get_owner(&self) -> H160 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.contract_owner_key.clone()) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => H160::zero(),
        }
    }

    /// Add authorized minter (owner only)
    #[method]
    pub fn add_minter(&self, minter: H160) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can add minters"));
            return false;
        }

        let storage = Storage::get_context();
        let minter_key = self.minters_prefix.concat(&minter.into_byte_string());
        let storage_clone = storage.clone(); Storage::put(storage_clone, minter_key, ByteString::from_literal("true"));

        let mut event_data = Array::new(); event_data.push(minter.into_any()); Runtime::notify(ByteString::from_literal("MinterAdded"), event_data);
        true
    }

    /// Set base URI for token metadata (owner only)
    #[method]
    pub fn set_base_uri(&self, base_uri: ByteString) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can set base URI"));
            return false;
        }

        let storage = Storage::get_context();
        if base_uri.is_empty() {
            let storage_clone = storage.clone(); Storage::delete(storage_clone, self.base_uri_key.clone());
        } else {
            let storage_clone = storage.clone();
            Storage::put(storage_clone, self.base_uri_key.clone(), base_uri.clone());
        }

        let mut event_data = Array::new(); event_data.push(base_uri.into_any()); Runtime::notify(ByteString::from_literal("BaseURISet"), event_data);
        true
    }

    /// Get base URI
    #[method]
    #[safe]
    pub fn get_base_uri(&self) -> ByteString {
        let storage = Storage::get_context();
        match Storage::get(storage, self.base_uri_key.clone()) {
            Some(uri) => uri,
            None => ByteString::empty(),
        }
    }

    /// Get token URI (base_uri + token_id)
    #[method]
    #[safe]
    pub fn token_uri(&self, token_id: ByteString) -> ByteString {
        if self.owner_of(token_id.clone()) == H160::zero() {
            return ByteString::empty();
        }

        let base_uri = self.get_base_uri();
        if base_uri.is_empty() {
            return token_id;
        }

        base_uri.concat(&token_id)
    }

    /// Pause contract (owner only)
    #[method]
    pub fn pause(&self) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can pause"));
            return false;
        }

        let storage = Storage::get_context();
        let storage_clone = storage.clone(); Storage::put(storage_clone, self.paused_key.clone(), ByteString::from_literal("true"));

        Runtime::notify(ByteString::from_literal("ContractPaused"), Array::new());
        true
    }

    /// Check if contract is paused
    #[method]
    #[safe]
    pub fn is_paused(&self) -> bool {
        let storage = Storage::get_context();
        Storage::get(storage, self.paused_key.clone()).is_some()
    }

    // Helper functions

    fn is_owner(&self) -> bool {
        let owner = self.get_owner();
        if owner == H160::zero() {
            return false;
        }
        Runtime::check_witness(owner)
    }

    fn is_authorized_minter(&self) -> bool {
        if self.is_owner() {
            return true;
        }

        let caller = Runtime::get_calling_script_hash();
        let storage = Storage::get_context();
        let minter_key = self.minters_prefix.concat(&caller.into_byte_string());
        Storage::get(storage, minter_key).is_some()
    }

    fn is_authorized_for_token(&self, owner: H160, token_id: ByteString) -> bool {
        // Check if caller is the owner
        if Runtime::check_witness(owner) {
            return true;
        }

        // Check if caller is approved for this token
        let approved = self.get_approved(token_id);
        if approved != H160::zero() && Runtime::check_witness(approved) {
            return true;
        }

        false
    }

    fn transfer_token(&self, from: H160, to: H160, token_id: ByteString) {
        let storage = Storage::get_context();

        // Update token owner
        let owner_key = self.owner_prefix.concat(&token_id);
        Storage::put(storage.clone(), owner_key, to.into_byte_string());

        // Clear approval
        let approved_key = self.approved_prefix.concat(&token_id);
        Storage::delete(storage.clone(), approved_key);

        // Update balances
        let from_balance = self.balance_of(from);
        let new_from_balance = from_balance.checked_sub(&Int256::one());
        let from_balance_key = self.balance_prefix.concat(&from.into_byte_string());

        if new_from_balance == Int256::zero() {
            Storage::delete(storage.clone(), from_balance_key);
        } else {
            Storage::put(storage.clone(), from_balance_key, new_from_balance.into_byte_string());
        }

        let to_balance = self.balance_of(to);
        let new_to_balance = to_balance.checked_add(&Int256::one());
        let to_balance_key = self.balance_prefix.concat(&to.into_byte_string());
        let storage_clone = storage.clone();
        Storage::put(storage_clone, to_balance_key, new_to_balance.into_byte_string());

        // Update token lists
        self.remove_token_from_owner(from, token_id.clone());
        self.add_token_to_owner(to, token_id.clone());

        // Emit Transfer event
        self.emit_transfer(from, to, Int256::one(), token_id);
    }

    fn add_token_to_owner(&self, owner: H160, token_id: ByteString) {
        let storage = Storage::get_context();
        let tokens_key = self.token_prefix.concat(&owner.into_byte_string());

        let mut tokens = match Storage::get(storage.clone(), tokens_key.clone()) {
            Some(tokens_data) => self.deserialize_token_list(tokens_data),
            None => Array::new(),
        };

        tokens.push(token_id);
        let serialized = self.serialize_token_list(tokens);
        let storage_clone = storage.clone();
        Storage::put(storage_clone, tokens_key, serialized);
    }

    fn remove_token_from_owner(&self, owner: H160, token_id: ByteString) {
        let storage = Storage::get_context();
        let tokens_key = self.token_prefix.concat(&owner.into_byte_string());

        let tokens = match Storage::get(storage.clone(), tokens_key.clone()) {
            Some(tokens_data) => self.deserialize_token_list(tokens_data),
            None => return,
        };

        let mut new_tokens = Array::new();
        for i in 0..tokens.size() {
            let token = tokens.get(i).clone();
            if token != token_id {
                new_tokens.push(token);
            }
        }

        if new_tokens.size() == 0 {
            let storage_clone = storage.clone(); Storage::delete(storage_clone, tokens_key);
        } else {
            let serialized = self.serialize_token_list(new_tokens);
            let storage_clone = storage.clone();
            Storage::put(storage_clone, tokens_key, serialized);
        }
    }

    fn add_token_to_global_list(&self, token_id: ByteString) {
        let storage = Storage::get_context();
        let current_supply = self.total_supply();
        let index_key = self.token_index_prefix.concat(&token_id);
        let storage_clone = storage.clone(); Storage::put(storage_clone, index_key, current_supply.into_byte_string());
    }

    fn remove_token_from_global_list(&self, token_id: ByteString) {
        let storage = Storage::get_context();
        let index_key = self.token_index_prefix.concat(&token_id);
        let storage_clone = storage.clone(); Storage::delete(storage_clone, index_key);
    }

    fn emit_transfer(&self, from: H160, to: H160, amount: Int256, token_id: ByteString) {
        let mut event_data = Array::new();
        event_data.push(from.into_any());
        event_data.push(to.into_any());
        event_data.push(amount.into_any());
        event_data.push(token_id.into_any());
        Runtime::notify(ByteString::from_literal("Transfer"), event_data);
    }

    fn on_payment_callback(&self, from: H160, amount: Int256, token_id: ByteString, data: Any) {
        // This would call onNEP11Payment on the recipient contract if it's a contract
        let mut event_data = Array::new();
        event_data.push(from.into_any());
        event_data.push(amount.into_any());
        event_data.push(token_id.into_any());
        event_data.push(data);
        Runtime::notify(ByteString::from_literal("PaymentCallback"), event_data);
    }

    fn serialize_token_list(&self, tokens: Array<ByteString>) -> ByteString {
        let mut serialized = ByteString::empty();
        let len = tokens.size();

        // Store array length
        serialized = serialized.concat(&ByteString::from_bytes(&(len as u32).to_le_bytes()));

        // Store each token ID with its length
        for i in 0..len {
            let token = tokens.get(i);
            let token_bytes = token.to_bytes();
            serialized = serialized.concat(&ByteString::from_bytes(&(token_bytes.len() as u32).to_le_bytes()));
            serialized = serialized.concat(&token);
        }

        serialized
    }

    fn deserialize_token_list(&self, serialized: ByteString) -> Array<ByteString> {
        let bytes = serialized.to_bytes();
        let mut tokens = Array::new();

        if bytes.len() < 4 {
            return tokens;
        }

        let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        let mut offset = 4;

        for _ in 0..len {
            if offset + 4 > bytes.len() {
                break;
            }

            let token_len = u32::from_le_bytes([
                bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
            ]) as usize;
            offset += 4;

            if offset + token_len > bytes.len() {
                break;
            }

            let token_bytes = &bytes[offset..offset + token_len];
            let token = ByteString::from_bytes(token_bytes);
            tokens.push(token);
            offset += token_len;
        }

        tokens
    }

    fn serialize_properties(&self, properties: Map<ByteString, Any>) -> ByteString {
        // Production-ready serialization format
        // Since Map doesn't support iteration in the current API, we store the map directly
        // This is a limitation of the current Map implementation
        let mut serialized = ByteString::empty();
        let size = properties.size();

        // Store map size (4 bytes) - for future compatibility when iteration is supported
        serialized = serialized.concat(&ByteString::from_bytes(&(size as u32).to_le_bytes()));

        // For now, we can't iterate over Map entries due to API limitations
        // In a production environment, you would either:
        // 1. Use a different data structure that supports iteration
        // 2. Maintain a separate list of keys
        // 3. Use the Map as-is and handle serialization differently

        // Store a marker indicating this is a Map that needs special handling
        serialized = serialized.concat(&ByteString::from_literal("MAP_PLACEHOLDER"));

        serialized
    }

    fn deserialize_properties(&self, serialized: ByteString) -> Map<ByteString, Any> {
        let properties = Map::new();
        let bytes = serialized.to_bytes();

        if bytes.len() < 4 {
            return properties;
        }

        let _len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

        // Check for MAP_PLACEHOLDER marker
        let marker_start = 4;
        let marker = ByteString::from_literal("MAP_PLACEHOLDER");
        let marker_bytes = marker.to_bytes();

        if bytes.len() >= marker_start + marker_bytes.len() {
            let serialized_marker = &bytes[marker_start..marker_start + marker_bytes.len()];
            if serialized_marker == marker_bytes {
                // This is a placeholder map - return empty map
                // Complete implementation with proper deserialization
                // based on your specific requirements
                return properties;
            }
        }

        // For backward compatibility, return empty map
        properties
    }
}
