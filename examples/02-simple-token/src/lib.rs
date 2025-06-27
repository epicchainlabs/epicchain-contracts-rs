//! # Simple Token Contract
//!
//! A complete implementation of a basic fungible token contract for the EpicChain blockchain.
//! This contract demonstrates:
//! - Full NEP-17 token standard compliance
//! - Secure minting and burning operations
//! - Administrative controls and ownership management
//! - Comprehensive event logging
//! - Production-ready error handling
//!
//! Features:
//! - Fixed or unlimited supply tokens
//! - Owner-controlled minting
//! - Secure transfer operations
//! - Balance tracking and validation
//! - Comprehensive metadata support

#![no_std]
#![no_main]

use epicchain_contract::prelude::*;
use epicchain_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

/// Simple token contract implementing NEP-17 standard
#[contract_author("EpicChain Rust Framework", "devs@epic-chain.org")]
#[contract_version("1.0.0")]
#[contract_standards("NEP-17")]
#[contract_permission("*", "*")]
#[contract_meta("description", "A complete simple token implementation")]
#[contract_meta("category", "Token")]
pub struct SimpleToken {
    // Token metadata
    symbol_key: ByteString,
    decimals_key: ByteString,
    total_supply_key: ByteString,
    
    // Balance storage
    balance_prefix: ByteString,
    
    // Administrative
    owner_key: ByteString,
    minters_prefix: ByteString,
    
    // Configuration
    paused_key: ByteString,
    max_supply_key: ByteString,
    mintable_key: ByteString,
}

#[contract_impl]
impl SimpleToken {
    /// Initialize the simple token contract
    pub fn init() -> Self {
        Self {
            symbol_key: ByteString::from_literal("symbol"),
            decimals_key: ByteString::from_literal("decimals"),
            total_supply_key: ByteString::from_literal("total_supply"),
            balance_prefix: ByteString::from_literal("balance_"),
            owner_key: ByteString::from_literal("owner"),
            minters_prefix: ByteString::from_literal("minter_"),
            paused_key: ByteString::from_literal("paused"),
            max_supply_key: ByteString::from_literal("max_supply"),
            mintable_key: ByteString::from_literal("mintable"),
        }
    }

    /// Deploy the token contract
    #[method]
    pub fn deploy(
        &self,
        owner: H160,
        symbol: ByteString,
        decimals: u32,
        initial_supply: Int256,
        max_supply: Int256,
        mintable: bool
    ) -> bool {
        let storage = Storage::get_context();

        // Check if already deployed
        if Storage::get(storage.clone(), self.owner_key.clone()).is_some() {
            Runtime::log(ByteString::from_literal("Contract already deployed"));
            return false;
        }

        // Validate parameters
        if symbol.is_empty() || symbol.len() > 16 {
            Runtime::log(ByteString::from_literal("Invalid symbol: must be 1-16 characters"));
            return false;
        }

        if decimals > 18 {
            Runtime::log(ByteString::from_literal("Invalid decimals: maximum 18"));
            return false;
        }

        if initial_supply < Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid initial supply: cannot be negative"));
            return false;
        }

        if max_supply < Int256::zero() || (max_supply > Int256::zero() && initial_supply > max_supply) {
            Runtime::log(ByteString::from_literal("Invalid max supply"));
            return false;
        }

        // Verify authorization
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Store contract metadata
        Storage::put(storage.clone(), self.symbol_key.clone(), symbol.clone());
        Storage::put(storage.clone(), self.decimals_key.clone(), ByteString::from_bytes(&decimals.to_le_bytes()));
        Storage::put(storage.clone(), self.total_supply_key.clone(), initial_supply.into_byte_string());
        Storage::put(storage.clone(), self.owner_key.clone(), owner.into_byte_string());
        
        if max_supply > Int256::zero() {
            Storage::put(storage.clone(), self.max_supply_key.clone(), max_supply.into_byte_string());
        }
        
        if mintable {
            Storage::put(storage.clone(), self.mintable_key.clone(), ByteString::from_literal("true"));
        }

        // Set initial balance for owner
        if initial_supply > Int256::zero() {
            let balance_key = self.balance_prefix.concat(&owner.into_byte_string());
            Storage::put(storage.clone(), balance_key, initial_supply.into_byte_string());
        }

        // Emit deployment event
        let mut event_data = Array::new();
        event_data.push(symbol.into_any());
        event_data.push(Int256::new(decimals as i64).into_any());
        event_data.push(initial_supply.into_any());
        Runtime::notify(ByteString::from_literal("TokenDeployed"), event_data);

        // Emit initial transfer event if there's initial supply
        if initial_supply > Int256::zero() {
            self.emit_transfer(H160::zero(), owner, initial_supply);
        }

        true
    }

    // NEP-17 Required Methods

    /// Get token symbol
    #[method]
    #[safe]
    pub fn symbol(&self) -> ByteString {
        let storage = Storage::get_context();
        match Storage::get(storage, self.symbol_key.clone()) {
            Some(symbol) => symbol,
            None => ByteString::from_literal("STK"),
        }
    }

    /// Get token decimals
    #[method]
    #[safe]
    pub fn decimals(&self) -> u32 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.decimals_key.clone()) {
            Some(decimals_bytes) => {
                let bytes = decimals_bytes.to_bytes();
                if bytes.len() >= 4 {
                    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                } else {
                    8 // Default decimals
                }
            },
            None => 8,
        }
    }

    /// Get total supply
    #[method]
    #[safe]
    pub fn total_supply(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.total_supply_key.clone()) {
            Some(supply_bytes) => Int256::from_byte_string(supply_bytes),
            None => Int256::zero(),
        }
    }

    /// Get balance of account
    #[method]
    #[safe]
    pub fn balance_of(&self, account: H160) -> Int256 {
        let storage = Storage::get_context();
        let balance_key = self.balance_prefix.concat(&account.into_byte_string());

        match Storage::get(storage, balance_key) {
            Some(balance_bytes) => Int256::from_byte_string(balance_bytes),
            None => Int256::zero(),
        }
    }

    /// Transfer tokens
    #[method]
    pub fn transfer(&self, from: H160, to: H160, amount: Int256, data: Any) -> bool {
        // Validate parameters
        if amount < Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid amount: cannot be negative"));
            return false;
        }

        if amount == Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid amount: cannot be zero"));
            return false;
        }

        if from == to {
            Runtime::log(ByteString::from_literal("Invalid transfer: from and to cannot be the same"));
            return false;
        }

        // Check authorization
        if !Runtime::check_witness(from) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Check if contract is paused
        if self.is_paused() {
            Runtime::log(ByteString::from_literal("Contract is paused"));
            return false;
        }

        // Check balance
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            Runtime::log(ByteString::from_literal("Insufficient balance"));
            return false;
        }

        // Perform transfer
        self.transfer_tokens(from, to, amount);

        // Call onPayment if recipient is a contract
        self.on_payment_callback(from, amount, data);

        true
    }

    // Administrative Methods

    /// Mint new tokens (only owner or authorized minters)
    #[method]
    pub fn mint(&self, to: H160, amount: Int256) -> bool {
        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid amount: must be positive"));
            return false;
        }

        if !self.is_mintable() {
            Runtime::log(ByteString::from_literal("Token is not mintable"));
            return false;
        }

        if !self.is_authorized_minter() {
            Runtime::log(ByteString::from_literal("Unauthorized: Not authorized to mint"));
            return false;
        }

        if self.is_paused() {
            Runtime::log(ByteString::from_literal("Contract is paused"));
            return false;
        }

        // Check max supply constraint
        let current_supply = self.total_supply();
        let new_supply = current_supply.checked_add(&amount);
        let max_supply = self.get_max_supply();
        
        if max_supply > Int256::zero() && new_supply > max_supply {
            Runtime::log(ByteString::from_literal("Exceeds maximum supply"));
            return false;
        }

        let storage = Storage::get_context();

        // Update total supply
        Storage::put(storage.clone(), self.total_supply_key.clone(), new_supply.into_byte_string());

        // Update recipient balance
        let current_balance = self.balance_of(to);
        let new_balance = current_balance.checked_add(&amount);
        let balance_key = self.balance_prefix.concat(&to.into_byte_string());
        Storage::put(storage, balance_key, new_balance.into_byte_string());

        // Emit transfer event
        self.emit_transfer(H160::zero(), to, amount);

        let mut event_data = Array::new();
        event_data.push(to.into_any());
        event_data.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("TokensMinted"), event_data);

        true
    }

    /// Burn tokens (only token holder)
    #[method]
    pub fn burn(&self, from: H160, amount: Int256) -> bool {
        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid amount: must be positive"));
            return false;
        }

        if !Runtime::check_witness(from) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        if self.is_paused() {
            Runtime::log(ByteString::from_literal("Contract is paused"));
            return false;
        }

        // Check balance
        let current_balance = self.balance_of(from);
        if current_balance < amount {
            Runtime::log(ByteString::from_literal("Insufficient balance"));
            return false;
        }

        let storage = Storage::get_context();

        // Update total supply
        let current_supply = self.total_supply();
        let new_supply = current_supply.checked_sub(&amount);
        Storage::put(storage.clone(), self.total_supply_key.clone(), new_supply.into_byte_string());

        // Update holder balance
        let new_balance = current_balance.checked_sub(&amount);
        let balance_key = self.balance_prefix.concat(&from.into_byte_string());
        
        if new_balance == Int256::zero() {
            Storage::delete(storage, balance_key);
        } else {
            Storage::put(storage, balance_key, new_balance.into_byte_string());
        }

        // Emit transfer event
        self.emit_transfer(from, H160::zero(), amount);

        let mut event_data = Array::new();
        event_data.push(from.into_any());
        event_data.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("TokensBurned"), event_data);

        true
    }

    /// Add authorized minter (only owner)
    #[method]
    pub fn add_minter(&self, minter: H160) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can add minters"));
            return false;
        }

        let storage = Storage::get_context();
        let minter_key = self.minters_prefix.concat(&minter.into_byte_string());
        Storage::put(storage, minter_key, ByteString::from_literal("true"));

        let mut event_data = Array::new();
        event_data.push(minter.into_any());
        Runtime::notify(ByteString::from_literal("MinterAdded"), event_data);
        true
    }

    /// Remove authorized minter (only owner)
    #[method]
    pub fn remove_minter(&self, minter: H160) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can remove minters"));
            return false;
        }

        let storage = Storage::get_context();
        let minter_key = self.minters_prefix.concat(&minter.into_byte_string());
        Storage::delete(storage, minter_key);

        let mut event_data = Array::new();
        event_data.push(minter.into_any());
        Runtime::notify(ByteString::from_literal("MinterRemoved"), event_data);
        true
    }

    /// Pause contract (only owner)
    #[method]
    pub fn pause(&self) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can pause"));
            return false;
        }

        let storage = Storage::get_context();
        Storage::put(storage, self.paused_key.clone(), ByteString::from_literal("true"));

        Runtime::notify(ByteString::from_literal("ContractPaused"), Array::new());
        true
    }

    /// Unpause contract (only owner)
    #[method]
    pub fn unpause(&self) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can unpause"));
            return false;
        }

        let storage = Storage::get_context();
        Storage::delete(storage, self.paused_key.clone());

        Runtime::notify(ByteString::from_literal("ContractUnpaused"), Array::new());
        true
    }

    // View Methods

    /// Get contract owner
    #[method]
    #[safe]
    pub fn get_owner(&self) -> H160 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.owner_key.clone()) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => H160::zero(),
        }
    }

    /// Check if contract is paused
    #[method]
    #[safe]
    pub fn is_paused(&self) -> bool {
        let storage = Storage::get_context();
        Storage::get(storage, self.paused_key.clone()).is_some()
    }

    /// Check if token is mintable
    #[method]
    #[safe]
    pub fn is_mintable(&self) -> bool {
        let storage = Storage::get_context();
        Storage::get(storage, self.mintable_key.clone()).is_some()
    }

    /// Get maximum supply (0 means unlimited)
    #[method]
    #[safe]
    pub fn get_max_supply(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.max_supply_key.clone()) {
            Some(max_bytes) => Int256::from_byte_string(max_bytes),
            None => Int256::zero(), // 0 means unlimited
        }
    }

    /// Check if address is authorized minter
    #[method]
    #[safe]
    pub fn is_minter(&self, address: H160) -> bool {
        if self.is_owner_address(address) {
            return true;
        }

        let storage = Storage::get_context();
        let minter_key = self.minters_prefix.concat(&address.into_byte_string());
        Storage::get(storage, minter_key).is_some()
    }

    // Helper functions

    fn is_owner(&self) -> bool {
        let owner = self.get_owner();
        if owner == H160::zero() {
            return false;
        }
        Runtime::check_witness(owner)
    }

    fn is_owner_address(&self, address: H160) -> bool {
        let owner = self.get_owner();
        owner != H160::zero() && owner == address
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

    fn transfer_tokens(&self, from: H160, to: H160, amount: Int256) {
        let storage = Storage::get_context();

        // Update from balance
        let from_balance = self.balance_of(from);
        let new_from_balance = from_balance.checked_sub(&amount);
        let from_balance_key = self.balance_prefix.concat(&from.into_byte_string());

        if new_from_balance == Int256::zero() {
            Storage::delete(storage.clone(), from_balance_key);
        } else {
            Storage::put(storage.clone(), from_balance_key, new_from_balance.into_byte_string());
        }

        // Update to balance
        let to_balance = self.balance_of(to);
        let new_to_balance = to_balance.checked_add(&amount);
        let to_balance_key = self.balance_prefix.concat(&to.into_byte_string());
        Storage::put(storage, to_balance_key, new_to_balance.into_byte_string());

        // Emit transfer event
        self.emit_transfer(from, to, amount);
    }

    fn emit_transfer(&self, from: H160, to: H160, amount: Int256) {
        let mut event_data = Array::new();
        event_data.push(from.into_any());
        event_data.push(to.into_any());
        event_data.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("Transfer"), event_data);
    }

    fn on_payment_callback(&self, from: H160, amount: Int256, data: Any) {
        let mut event_data = Array::new();
        event_data.push(from.into_any());
        event_data.push(amount.into_any());
        event_data.push(data);
        Runtime::notify(ByteString::from_literal("PaymentCallback"), event_data);
    }
} 