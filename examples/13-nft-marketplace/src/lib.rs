//! # NFT Marketplace Contract
//!
//! A comprehensive NFT marketplace demonstrating advanced DeFi patterns:
//! - Listing and trading of NFTs with escrow functionality
//! - Auction system with bidding and automatic settlement
//! - Royalty distribution for creators (NEP-24 integration)
//! - Offer system for direct purchases
//! - Platform fee management and revenue sharing
//! - Emergency controls and dispute resolution
//!
//! This contract showcases enterprise-grade marketplace functionality
//! with security, scalability, and user experience in mind.

#![no_std]
#![no_main]

use epicchain_contract::prelude::*;
use epicchain_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

mod types;
mod storage;
mod listings;
mod auctions;
mod offers;
mod royalties;

use types::*;
use storage::*;

/// NFT Marketplace contract
#[contract_author("EpicChain Rust Framework", "devs@epic-chain.org")]
#[contract_version("1.0.0")]
#[contract_standards("NEP-11,NEP-24")]
#[contract_permission("*", "*")]
#[contract_meta("description", "Comprehensive NFT marketplace with auctions and royalties")]
#[contract_meta("category", "Marketplace")]
pub struct NftMarketplace {
    // Core storage keys
    storage_keys: StorageKeys,
}

#[contract_impl]
impl NftMarketplace {
    /// Initialize the marketplace
    pub fn init() -> Self {
        Self {
            storage_keys: StorageKeys::new(),
        }
    }

    /// Initialize the marketplace with configuration
    #[method]
    pub fn initialize(
        &self,
        owner: H160,
        platform_fee_rate: u32,
        min_listing_duration: u64,
        max_listing_duration: u64
    ) -> bool {
        let storage = Storage::get_context();

        // Check if already initialized
        if Storage::get(storage.clone(), self.storage_keys.owner_key.clone()).is_some() {
            Runtime::log(ByteString::from_literal("Marketplace already initialized"));
            return false;
        }

        // Validate parameters
        if platform_fee_rate > 1000 { // Max 10%
            Runtime::log(ByteString::from_literal("Platform fee too high (max 10%)"));
            return false;
        }

        if min_listing_duration < 3600 || max_listing_duration > 2592000 { // 1 hour to 30 days
            Runtime::log(ByteString::from_literal("Invalid listing duration limits"));
            return false;
        }

        if min_listing_duration >= max_listing_duration {
            Runtime::log(ByteString::from_literal("Min duration must be less than max duration"));
            return false;
        }

        // Verify authorization
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Store configuration
        Storage::put(storage.clone(), self.storage_keys.owner_key.clone(), owner.into_byte_string());
        Storage::put(storage.clone(), self.storage_keys.platform_fee_key.clone(), ByteString::from_bytes(&platform_fee_rate.to_le_bytes()));
        Storage::put(storage.clone(), self.storage_keys.min_duration_key.clone(), ByteString::from_bytes(&min_listing_duration.to_le_bytes()));
        Storage::put(storage.clone(), self.storage_keys.max_duration_key.clone(), ByteString::from_bytes(&max_listing_duration.to_le_bytes()));

        // Initialize counters
        Storage::put(storage.clone(), self.storage_keys.listing_count_key.clone(), Int256::zero().into_byte_string());
        Storage::put(storage.clone(), self.storage_keys.auction_count_key.clone(), Int256::zero().into_byte_string());
        let storage_clone = storage.clone(); Storage::put(storage_clone, self.storage_keys.offer_count_key.clone(), Int256::zero().into_byte_string());

        let mut event_data = Array::new(); event_data.push(owner.into_any()); Runtime::notify(ByteString::from_literal("MarketplaceInitialized"), event_data);
        true
    }

    /// Get marketplace owner
    #[method]
    #[safe]
    pub fn get_owner(&self) -> H160 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.storage_keys.owner_key.clone()) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => H160::zero(),
        }
    }

    /// Get platform fee rate
    #[method]
    #[safe]
    pub fn get_platform_fee_rate(&self) -> u32 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.storage_keys.platform_fee_key.clone()) {
            Some(fee_bytes) => {
                let bytes = fee_bytes.to_bytes();
                if bytes.len() >= 4 {
                    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                } else {
                    250 // Default 2.5%
                }
            },
            None => 250,
        }
    }

    /// Check if marketplace is paused
    #[method]
    #[safe]
    pub fn is_paused(&self) -> bool {
        let storage = Storage::get_context();
        Storage::get(storage, self.storage_keys.paused_key.clone()).is_some()
    }

    /// Pause marketplace (owner only)
    #[method]
    pub fn pause(&self) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can pause"));
            return false;
        }

        let storage = Storage::get_context();
        let storage_clone = storage.clone(); Storage::put(storage_clone, self.storage_keys.paused_key.clone(), ByteString::from_literal("true"));

        Runtime::notify(ByteString::from_literal("MarketplacePaused"), Array::new());
        true
    }

    /// Unpause marketplace (owner only)
    #[method]
    pub fn unpause(&self) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can unpause"));
            return false;
        }

        let storage = Storage::get_context();
        let storage_clone = storage.clone(); Storage::delete(storage_clone, self.storage_keys.paused_key.clone());

        Runtime::notify(ByteString::from_literal("MarketplaceUnpaused"), Array::new());
        true
    }

    /// Get marketplace statistics
    #[method]
    #[safe]
    pub fn get_marketplace_stats(&self) -> Map<ByteString, Any> {
        let mut stats = Map::new();

        stats.put(
            ByteString::from_literal("total_listings"),
            self.get_listing_count().into_any()
        );
        stats.put(
            ByteString::from_literal("total_auctions"),
            self.get_auction_count().into_any()
        );
        stats.put(
            ByteString::from_literal("total_offers"),
            self.get_offer_count().into_any()
        );
        stats.put(
            ByteString::from_literal("platform_fee_rate"),
            Int256::new(self.get_platform_fee_rate() as i64).into_any()
        );
        stats.put(
            ByteString::from_literal("is_paused"),
            if self.is_paused() { Int256::one() } else { Int256::zero() }.into_any()
        );
        stats.put(
            ByteString::from_literal("owner"),
            self.get_owner().into_any()
        );

        stats
    }

    // Helper functions

    fn is_owner(&self) -> bool {
        let owner = self.get_owner();
        if owner == H160::zero() {
            return false;
        }
        Runtime::check_witness(owner)
    }

    fn get_listing_count(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.storage_keys.listing_count_key.clone()) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        }
    }

    fn get_auction_count(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.storage_keys.auction_count_key.clone()) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        }
    }

    fn get_offer_count(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.storage_keys.offer_count_key.clone()) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        }
    }
}
