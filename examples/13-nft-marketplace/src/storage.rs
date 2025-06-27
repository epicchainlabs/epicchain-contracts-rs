//! # Storage Management
//!
//! Centralized storage key management and utility functions for the NFT marketplace.

use epicchain_contract::prelude::*;
use epicchain_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};
use crate::types::*;

/// Storage keys for the marketplace contract
#[derive(Clone)]
pub struct StorageKeys {
    // Core configuration
    pub owner_key: ByteString,
    pub platform_fee_key: ByteString,
    pub paused_key: ByteString,

    // Duration limits
    pub min_duration_key: ByteString,
    pub max_duration_key: ByteString,
    pub min_auction_duration_key: ByteString,
    pub max_auction_duration_key: ByteString,

    // Counters
    pub listing_count_key: ByteString,
    pub auction_count_key: ByteString,
    pub offer_count_key: ByteString,
    pub sale_count_key: ByteString,

    // Data prefixes
    pub listing_prefix: ByteString,
    pub auction_prefix: ByteString,
    pub offer_prefix: ByteString,
    pub bid_prefix: ByteString,
    pub sale_prefix: ByteString,

    // Index prefixes for efficient queries
    pub seller_listings_prefix: ByteString,
    pub nft_listings_prefix: ByteString,
    pub active_auctions_prefix: ByteString,
    pub user_offers_prefix: ByteString,
    pub nft_offers_prefix: ByteString,

    // Escrow and balances
    pub escrow_prefix: ByteString,
    pub pending_withdrawals_prefix: ByteString,

    // Royalty tracking
    pub royalty_cache_prefix: ByteString,

    // Emergency controls
    pub emergency_stop_key: ByteString,
    pub authorized_operators_prefix: ByteString,
}

impl StorageKeys {
    pub fn new() -> Self {
        Self {
            // Core configuration
            owner_key: ByteString::from_literal("owner"),
            platform_fee_key: ByteString::from_literal("platform_fee"),
            paused_key: ByteString::from_literal("paused"),

            // Duration limits
            min_duration_key: ByteString::from_literal("min_duration"),
            max_duration_key: ByteString::from_literal("max_duration"),
            min_auction_duration_key: ByteString::from_literal("min_auction_duration"),
            max_auction_duration_key: ByteString::from_literal("max_auction_duration"),

            // Counters
            listing_count_key: ByteString::from_literal("listing_count"),
            auction_count_key: ByteString::from_literal("auction_count"),
            offer_count_key: ByteString::from_literal("offer_count"),
            sale_count_key: ByteString::from_literal("sale_count"),

            // Data prefixes
            listing_prefix: ByteString::from_literal("listing_"),
            auction_prefix: ByteString::from_literal("auction_"),
            offer_prefix: ByteString::from_literal("offer_"),
            bid_prefix: ByteString::from_literal("bid_"),
            sale_prefix: ByteString::from_literal("sale_"),

            // Index prefixes
            seller_listings_prefix: ByteString::from_literal("seller_listings_"),
            nft_listings_prefix: ByteString::from_literal("nft_listings_"),
            active_auctions_prefix: ByteString::from_literal("active_auctions_"),
            user_offers_prefix: ByteString::from_literal("user_offers_"),
            nft_offers_prefix: ByteString::from_literal("nft_offers_"),

            // Escrow and balances
            escrow_prefix: ByteString::from_literal("escrow_"),
            pending_withdrawals_prefix: ByteString::from_literal("pending_"),

            // Royalty tracking
            royalty_cache_prefix: ByteString::from_literal("royalty_"),

            // Emergency controls
            emergency_stop_key: ByteString::from_literal("emergency_stop"),
            authorized_operators_prefix: ByteString::from_literal("operator_"),
        }
    }

    /// Generate listing storage key
    pub fn listing_key(&self, listing_id: Int256) -> ByteString {
        self.listing_prefix.concat(&listing_id.into_byte_string())
    }

    /// Generate auction storage key
    pub fn auction_key(&self, auction_id: Int256) -> ByteString {
        self.auction_prefix.concat(&auction_id.into_byte_string())
    }

    /// Generate offer storage key
    pub fn offer_key(&self, offer_id: Int256) -> ByteString {
        self.offer_prefix.concat(&offer_id.into_byte_string())
    }

    /// Generate bid storage key
    pub fn bid_key(&self, auction_id: Int256, bidder: H160) -> ByteString {
        self.bid_prefix
            .concat(&auction_id.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&bidder.into_byte_string())
    }

    /// Generate seller listings index key
    pub fn seller_listings_key(&self, seller: H160) -> ByteString {
        self.seller_listings_prefix.concat(&seller.into_byte_string())
    }

    /// Generate storage key for NFT listings by contract and token
    pub fn nft_listings_key(&self, nft_contract: H160, token_id: ByteString) -> ByteString {
        self.nft_listings_prefix
            .concat(&nft_contract.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&token_id)
    }

    /// Generate storage key for user offers
    pub fn user_offers_key(&self, user: H160) -> ByteString {
        self.user_offers_prefix.concat(&user.into_byte_string())
    }

    /// Generate storage key for NFT offers
    pub fn nft_offers_key(&self, nft_contract: H160, token_id: ByteString) -> ByteString {
        self.nft_offers_prefix
            .concat(&nft_contract.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&token_id)
    }

    /// Generate storage key for user-token offers
    pub fn user_token_offers_key(&self, user: H160, token: H160) -> ByteString {
        self.escrow_prefix
            .concat(&user.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&token.into_byte_string())
    }

    /// Generate storage key for NFT royalty info
    pub fn nft_royalty_key(&self, nft_contract: H160, token_id: ByteString) -> ByteString {
        self.royalty_cache_prefix
            .concat(&nft_contract.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&token_id)
    }

    /// Generate escrow key for user and token
    pub fn escrow_key(&self, user: H160, token: H160) -> ByteString {
        self.escrow_prefix
            .concat(&user.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&token.into_byte_string())
    }

    /// Generate operator authorization key
    pub fn operator_key(&self, operator: H160) -> ByteString {
        self.authorized_operators_prefix.concat(&operator.into_byte_string())
    }

    /// Generate storage key for authorized operators
    pub fn authorized_operator_key(&self, operator: H160) -> ByteString {
        self.authorized_operators_prefix.concat(&operator.into_byte_string())
    }
}

/// Storage utility functions
pub struct StorageUtils;

impl StorageUtils {
    /// Store a list of IDs (for indexes)
    pub fn store_id_list(key: ByteString, ids: Array<Int256>) {
        let storage = Storage::get_context();
        let serialized = Self::serialize_id_list(ids);
        Storage::put(storage, key, serialized);
    }

    /// Load a list of IDs
    pub fn load_id_list(key: ByteString) -> Array<Int256> {
        let storage = Storage::get_context();
        match Storage::get(storage, key) {
            Some(data) => Self::deserialize_id_list(data),
            None => Array::new(),
        }
    }

    /// Add ID to a list
    pub fn add_to_id_list(key: ByteString, id: Int256) {
        let mut ids = Self::load_id_list(key.clone());
        ids.push(id);
        Self::store_id_list(key, ids);
    }

    /// Remove ID from a list
    pub fn remove_from_id_list(key: ByteString, id: Int256) {
        let ids = Self::load_id_list(key.clone());
        let mut filtered_ids = Array::new();

        // Manually filter without using iterator
        for i in 0..ids.size() {
            let existing_id = ids.get(i);
            if existing_id.clone() != id {
                filtered_ids.push(existing_id.clone());
            }
        }

        Self::store_id_list(key, filtered_ids);
    }

    /// Serialize list of IDs
    fn serialize_id_list(ids: Array<Int256>) -> ByteString {
        let mut data = ByteString::empty();
        let len = ids.size();

        // Store length
        data = data.concat(&ByteString::from_bytes(&(len as u32).to_le_bytes()));

        // Store each ID
        for i in 0..len {
            let id = ids.get(i);
            data = data.concat(&id.into_byte_string());
        }

        data
    }

    /// Deserialize list of IDs
    fn deserialize_id_list(data: ByteString) -> Array<Int256> {
        let bytes = data.to_bytes();
        let mut ids = Array::new();

        if bytes.len() < 4 {
            return ids;
        }

        let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        let mut offset = 4;

        for _ in 0..len {
            // Each Int256 is stored as a variable-length byte string
            // For simplicity, we'll assume fixed 32-byte representation
            if offset + 32 <= bytes.len() {
                let id_bytes = &bytes[offset..offset + 32];
                let id = Int256::from_byte_string(ByteString::from_bytes(id_bytes));
                ids.push(id);
                offset += 32;
            } else {
                break;
            }
        }

        ids
    }

    /// Store escrow balance
    pub fn store_escrow_balance(user: H160, token: H160, amount: Int256, keys: &StorageKeys) {
        let storage = Storage::get_context();
        let escrow_key = keys.escrow_key(user, token);

        if amount == Int256::zero() {
            Storage::delete(storage.clone(), escrow_key);
        } else {
            Storage::put(storage, escrow_key, amount.into_byte_string());
        }
    }

    /// Load escrow balance
    pub fn load_escrow_balance(user: H160, token: H160, keys: &StorageKeys) -> Int256 {
        let storage = Storage::get_context();
        let escrow_key = keys.escrow_key(user, token);

        match Storage::get(storage, escrow_key) {
            Some(amount_bytes) => Int256::from_byte_string(amount_bytes),
            None => Int256::zero(),
        }
    }

    /// Increment counter and return new value
    pub fn increment_counter(key: ByteString) -> Int256 {
        let storage = Storage::get_context();
        let current = match Storage::get(storage.clone(), key.clone()) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        };

        let new_count = current.checked_add(&Int256::one());
        Storage::put(storage, key, new_count.into_byte_string());
        new_count
    }

    /// Get counter value
    pub fn get_counter(key: ByteString) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage, key) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        }
    }

    /// Store configuration value (u32)
    pub fn store_u32_config(key: ByteString, value: u32) {
        let storage = Storage::get_context();
        Storage::put(storage, key, ByteString::from_bytes(&value.to_le_bytes()));
    }

    /// Load configuration value (u32)
    pub fn load_u32_config(key: ByteString, default: u32) -> u32 {
        let storage = Storage::get_context();
        match Storage::get(storage, key) {
            Some(value_bytes) => {
                let bytes = value_bytes.to_bytes();
                if bytes.len() >= 4 {
                    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                } else {
                    default
                }
            },
            None => default,
        }
    }

    /// Store configuration value (u64)
    pub fn store_u64_config(key: ByteString, value: u64) {
        let storage = Storage::get_context();
        Storage::put(storage, key, ByteString::from_bytes(&value.to_le_bytes()));
    }

    /// Load configuration value (u64)
    pub fn load_u64_config(key: ByteString, default: u64) -> u64 {
        let storage = Storage::get_context();
        match Storage::get(storage, key) {
            Some(value_bytes) => {
                let bytes = value_bytes.to_bytes();
                if bytes.len() >= 8 {
                    u64::from_le_bytes([
                        bytes[0], bytes[1], bytes[2], bytes[3],
                        bytes[4], bytes[5], bytes[6], bytes[7]
                    ])
                } else {
                    default
                }
            },
            None => default,
        }
    }

    /// Check if key exists
    pub fn key_exists(key: ByteString) -> bool {
        let storage = Storage::get_context();
        Storage::get(storage, key).is_some()
    }

    /// Delete key
    pub fn delete_key(key: ByteString) {
        let storage = Storage::get_context();
        Storage::delete(storage, key);
    }
}
