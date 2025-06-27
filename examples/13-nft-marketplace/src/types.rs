//! # Marketplace Types
//! 
//! Data structures and enums used throughout the NFT marketplace contract.

use epicchain_contract::prelude::*;
extern crate alloc;
use alloc::vec::Vec;

/// Listing status enumeration
#[derive(Clone, Copy, PartialEq)]
pub enum ListingStatus {
    Active = 0,
    Sold = 1,
    Cancelled = 2,
    Expired = 3,
}

impl ListingStatus {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => ListingStatus::Sold,
            2 => ListingStatus::Cancelled,
            3 => ListingStatus::Expired,
            _ => ListingStatus::Active,
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// Auction status enumeration
#[derive(Clone, Copy, PartialEq)]
pub enum AuctionStatus {
    Active = 0,
    Ended = 1,
    Cancelled = 2,
    Settled = 3,
}

impl AuctionStatus {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => AuctionStatus::Ended,
            2 => AuctionStatus::Cancelled,
            3 => AuctionStatus::Settled,
            _ => AuctionStatus::Active,
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// Offer status enumeration
#[derive(Clone, Copy, PartialEq)]
pub enum OfferStatus {
    Active = 0,
    Accepted = 1,
    Rejected = 2,
    Expired = 3,
    Withdrawn = 4,
}

impl OfferStatus {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => OfferStatus::Accepted,
            2 => OfferStatus::Rejected,
            3 => OfferStatus::Expired,
            4 => OfferStatus::Withdrawn,
            _ => OfferStatus::Active,
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// NFT listing information
#[derive(Clone)]
pub struct Listing {
    pub id: Int256,
    pub nft_contract: H160,
    pub token_id: ByteString,
    pub seller: H160,
    pub price: Int256,
    pub payment_token: H160,
    pub created_at: u64,
    pub expires_at: u64,
    pub status: ListingStatus,
}

/// NFT auction information
#[derive(Clone)]
pub struct Auction {
    pub id: Int256,
    pub nft_contract: H160,
    pub token_id: ByteString,
    pub seller: H160,
    pub starting_price: Int256,
    pub reserve_price: Int256,
    pub current_bid: Int256,
    pub highest_bidder: H160,
    pub payment_token: H160,
    pub created_at: u64,
    pub ends_at: u64,
    pub status: AuctionStatus,
    pub bid_count: u32,
}

/// Bid information
#[derive(Clone)]
pub struct Bid {
    pub auction_id: Int256,
    pub bidder: H160,
    pub amount: Int256,
    pub timestamp: u64,
}

/// Offer information
#[derive(Clone)]
pub struct Offer {
    pub id: Int256,
    pub nft_contract: H160,
    pub token_id: ByteString,
    pub offerer: H160,
    pub amount: Int256,
    pub payment_token: H160,
    pub created_at: u64,
    pub expires_at: u64,
    pub status: OfferStatus,
}

/// Royalty recipient information
#[derive(Clone)]
pub struct RoyaltyRecipient {
    pub recipient: H160,
    pub percentage: u32, // Basis points
}

/// Sale information for tracking
#[derive(Clone)]
pub struct Sale {
    pub nft_contract: H160,
    pub token_id: ByteString,
    pub seller: H160,
    pub buyer: H160,
    pub price: Int256,
    pub payment_token: H160,
    pub platform_fee: Int256,
    pub royalty_fee: Int256,
    pub timestamp: u64,
    pub sale_type: SaleType,
}

/// Sale type enumeration
#[derive(Clone, Copy, PartialEq)]
pub enum SaleType {
    DirectSale = 0,
    Auction = 1,
    Offer = 2,
}

impl SaleType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => SaleType::Auction,
            2 => SaleType::Offer,
            _ => SaleType::DirectSale,
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// Fee calculation result
#[derive(Clone)]
pub struct FeeCalculation {
    pub platform_fee: Int256,
    pub royalty_fees: Vec<RoyaltyRecipient>,
    pub seller_proceeds: Int256,
}

/// Marketplace configuration
#[derive(Clone)]
pub struct MarketplaceConfig {
    pub platform_fee_rate: u32,
    pub min_listing_duration: u64,
    pub max_listing_duration: u64,
    pub min_auction_duration: u64,
    pub max_auction_duration: u64,
    pub bid_extension_time: u64,
    pub min_bid_increment: u32,
}

impl MarketplaceConfig {
    pub fn default() -> Self {
        Self {
            platform_fee_rate: 250,      // 2.5%
            min_listing_duration: 3600,  // 1 hour
            max_listing_duration: 2592000, // 30 days
            min_auction_duration: 3600,  // 1 hour
            max_auction_duration: 604800, // 7 days
            bid_extension_time: 600,     // 10 minutes
            min_bid_increment: 500,      // 5%
        }
    }
}

/// Event data structures for better organization
pub struct ListingEvent {
    pub listing_id: Int256,
    pub nft_contract: H160,
    pub token_id: ByteString,
    pub seller: H160,
    pub price: Int256,
}

pub struct AuctionEvent {
    pub auction_id: Int256,
    pub nft_contract: H160,
    pub token_id: ByteString,
    pub seller: H160,
    pub starting_price: Int256,
    pub ends_at: u64,
}

pub struct BidEvent {
    pub auction_id: Int256,
    pub bidder: H160,
    pub amount: Int256,
    pub is_winning: bool,
}

pub struct SaleEvent {
    pub nft_contract: H160,
    pub token_id: ByteString,
    pub seller: H160,
    pub buyer: H160,
    pub price: Int256,
    pub sale_type: SaleType,
}

pub struct OfferEvent {
    pub offer_id: Int256,
    pub nft_contract: H160,
    pub token_id: ByteString,
    pub offerer: H160,
    pub amount: Int256,
}

/// Validation helpers
impl Listing {
    pub fn is_active(&self) -> bool {
        self.status == ListingStatus::Active
    }

    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time > self.expires_at
    }

    pub fn can_be_purchased(&self, current_time: u64) -> bool {
        self.is_active() && !self.is_expired(current_time)
    }
}

impl Auction {
    pub fn is_active(&self) -> bool {
        self.status == AuctionStatus::Active
    }

    pub fn is_ended(&self, current_time: u64) -> bool {
        current_time >= self.ends_at || self.status == AuctionStatus::Ended
    }

    pub fn can_receive_bids(&self, current_time: u64) -> bool {
        self.is_active() && !self.is_ended(current_time)
    }

    pub fn has_reserve_met(&self) -> bool {
        self.current_bid >= self.reserve_price
    }

    pub fn calculate_min_bid(&self, min_increment_rate: u32) -> Int256 {
        if self.current_bid == Int256::zero() {
            self.starting_price
        } else {
            let increment = self.current_bid
                .checked_mul(&Int256::new(min_increment_rate as i64))
                .checked_div(&Int256::new(10000));
            self.current_bid.checked_add(&increment)
        }
    }
}

impl Offer {
    pub fn is_active(&self) -> bool {
        self.status == OfferStatus::Active
    }

    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time > self.expires_at
    }

    pub fn can_be_accepted(&self, current_time: u64) -> bool {
        self.is_active() && !self.is_expired(current_time)
    }
}

/// Utility functions for type conversions
pub fn bytes_to_listing_status(bytes: &[u8]) -> ListingStatus {
    if bytes.len() > 0 {
        ListingStatus::from_u8(bytes[0])
    } else {
        ListingStatus::Active
    }
}

pub fn bytes_to_auction_status(bytes: &[u8]) -> AuctionStatus {
    if bytes.len() > 0 {
        AuctionStatus::from_u8(bytes[0])
    } else {
        AuctionStatus::Active
    }
}

pub fn bytes_to_offer_status(bytes: &[u8]) -> OfferStatus {
    if bytes.len() > 0 {
        OfferStatus::from_u8(bytes[0])
    } else {
        OfferStatus::Active
    }
}

pub fn bytes_to_u64(bytes: &[u8]) -> u64 {
    if bytes.len() >= 8 {
        u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]
        ])
    } else {
        0
    }
}

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    if bytes.len() >= 4 {
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    } else {
        0
    }
}
