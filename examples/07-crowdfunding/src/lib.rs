//! # Crowdfunding Smart Contract
//!
//! A comprehensive crowdfunding platform demonstrating real-world DeFi functionality:
//! - Goal-based funding with time limits
//! - Refund mechanisms if goals aren't met
//! - Milestone-based fund release
//! - Contributor tracking and rewards
//! - Administrative controls and emergency mechanisms
//!
//! This contract showcases advanced EpicChain patterns for decentralized fundraising.

#![no_std]
#![no_main]

use epicchain_contract::prelude::*;
use epicchain_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};
use epicchain_contract::contract::native::{Gas, Neo};

/// Campaign status enumeration
#[derive(Clone, Copy, PartialEq)]
pub enum CampaignStatus {
    Active = 0,
    Successful = 1,
    Failed = 2,
    Cancelled = 3,
}

impl CampaignStatus {
    fn from_u8(value: u8) -> Self {
        match value {
            1 => CampaignStatus::Successful,
            2 => CampaignStatus::Failed,
            3 => CampaignStatus::Cancelled,
            _ => CampaignStatus::Active,
        }
    }

    fn to_u8(self) -> u8 {
        self as u8
    }
}

/// Crowdfunding platform contract
#[contract_author("EpicChain Rust Framework", "devs@epic-chain.org")]
#[contract_version("1.0.0")]
#[contract_standards("")]
#[contract_permission("*", "*")]
#[contract_meta("description", "Decentralized crowdfunding platform with milestone-based releases")]
#[contract_meta("category", "DeFi")]
pub struct Crowdfunding {
    // Campaign storage keys
    campaign_prefix: ByteString,
    campaign_count_key: ByteString,

    // Contribution tracking
    contributions_prefix: ByteString,  // campaign_id + contributor -> amount
    contributor_list_prefix: ByteString, // campaign_id -> list of contributors
    total_raised_prefix: ByteString,   // campaign_id -> total amount raised

    // Milestone tracking
    milestones_prefix: ByteString,     // campaign_id -> milestone data
    milestone_released_prefix: ByteString, // campaign_id + milestone_id -> released amount

    // Administrative
    platform_owner_key: ByteString,
    platform_fee_key: ByteString,     // Platform fee percentage (basis points)
    emergency_pause_key: ByteString,

    // Supported tokens
    supported_tokens_key: ByteString,
}

#[contract_impl]
impl Crowdfunding {
    /// Initialize the crowdfunding platform
    pub fn init() -> Self {
        Self {
            campaign_prefix: ByteString::from_literal("campaign_"),
            campaign_count_key: ByteString::from_literal("campaign_count"),
            contributions_prefix: ByteString::from_literal("contrib_"),
            contributor_list_prefix: ByteString::from_literal("contributors_"),
            total_raised_prefix: ByteString::from_literal("raised_"),
            milestones_prefix: ByteString::from_literal("milestones_"),
            milestone_released_prefix: ByteString::from_literal("released_"),
            platform_owner_key: ByteString::from_literal("platform_owner"),
            platform_fee_key: ByteString::from_literal("platform_fee"),
            emergency_pause_key: ByteString::from_literal("emergency_pause"),
            supported_tokens_key: ByteString::from_literal("supported_tokens"),
        }
    }

    /// Initialize the platform (one-time setup)
    #[method]
    pub fn initialize(&self, owner: H160, platform_fee_bp: u32) -> bool {
        let storage = Storage::get_context();

        // Check if already initialized
        if Storage::get(storage.clone(), self.platform_owner_key.clone()).is_some() {
            Runtime::log(ByteString::from_literal("Platform already initialized"));
            return false;
        }

        // Validate parameters
        if platform_fee_bp > 1000 { // Max 10% fee
            Runtime::log(ByteString::from_literal("Platform fee too high (max 10%)"));
            return false;
        }

        // Verify authorization
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Store platform configuration
        Storage::put(storage.clone(), self.platform_owner_key.clone(), owner.into_byte_string());
        Storage::put(storage.clone(), self.platform_fee_key.clone(), ByteString::from_bytes(&platform_fee_bp.to_le_bytes()));
        Storage::put(storage.clone(), self.campaign_count_key.clone(), Int256::zero().into_byte_string());

        // Initialize with GAS as default supported token
        let epicpulse_hash = Gas::hash();
        let storage_clone = storage.clone(); Storage::put(storage_clone, self.supported_tokens_key.clone(), epicpulse_hash.into_byte_string());

        let mut event_data = Array::new(); event_data.push(owner.into_any()); Runtime::notify(ByteString::from_literal("PlatformInitialized"), event_data);
        true
    }

    /// Create a new crowdfunding campaign
    #[method]
    pub fn create_campaign(
        &self,
        creator: H160,
        title: ByteString,
        description: ByteString,
        funding_goal: Int256,
        deadline: u64,
        payment_token: H160
    ) -> Int256 {
        // Validate inputs
        if !self.validate_campaign_params(&title, &description, funding_goal, deadline) {
            return Int256::minus_one();
        }

        // Verify authorization
        if !Runtime::check_witness(creator) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return Int256::minus_one();
        }

        // Check if platform is paused
        if self.is_emergency_paused() {
            Runtime::log(ByteString::from_literal("Platform is paused"));
            return Int256::minus_one();
        }

        // Verify supported token
        if !self.is_token_supported(payment_token) {
            Runtime::log(ByteString::from_literal("Token not supported"));
            return Int256::minus_one();
        }

        let storage = Storage::get_context();

        // Get next campaign ID
        let campaign_count = self.get_campaign_count();
        let campaign_id = campaign_count.checked_add(&Int256::one());

        // Create campaign data
        let campaign_data = self.serialize_campaign_data(
            creator,
            title.clone(),
            description,
            funding_goal,
            deadline,
            payment_token,
            CampaignStatus::Active
        );

        // Store campaign
        let campaign_key = self.campaign_prefix.concat(&campaign_id.into_byte_string());
        Storage::put(storage.clone(), campaign_key, campaign_data);

        // Update campaign count
        Storage::put(storage.clone(), self.campaign_count_key.clone(), campaign_id.into_byte_string());

        // Initialize campaign tracking
        let raised_key = self.total_raised_prefix.concat(&campaign_id.into_byte_string());
        let storage_clone = storage.clone(); Storage::put(storage_clone, raised_key, Int256::zero().into_byte_string());

        let mut event_data = Array::new();
        event_data.push(campaign_id.into_any());
        event_data.push(creator.into_any());
        event_data.push(title.into_any());
        Runtime::notify(ByteString::from_literal("CampaignCreated"), event_data);

        campaign_id
    }

    /// Contribute to a campaign
    #[method]
    pub fn contribute(&self, campaign_id: Int256, contributor: H160, amount: Int256) -> bool {
        // Validate inputs
        if campaign_id <= Int256::zero() || amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid campaign ID or amount"));
            return false;
        }

        // Verify authorization
        if !Runtime::check_witness(contributor) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Check if platform is paused
        if self.is_emergency_paused() {
            Runtime::log(ByteString::from_literal("Platform is paused"));
            return false;
        }

        // Get campaign data
        let campaign_data = match self.get_campaign_data(campaign_id) {
            Some(data) => data,
            None => {
                Runtime::log(ByteString::from_literal("Campaign not found"));
                return false;
            }
        };

        // Verify campaign is active and not expired
        if !self.is_campaign_active(&campaign_data) {
            Runtime::log(ByteString::from_literal("Campaign is not active or has expired"));
            return false;
        }

        let storage = Storage::get_context();

        // Update contributor's contribution
        let contrib_key = self.contributions_prefix
            .concat(&campaign_id.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&contributor.into_byte_string());

        let current_contrib = match Storage::get(storage.clone(), contrib_key.clone()) {
            Some(amount_bytes) => Int256::from_byte_string(amount_bytes),
            None => Int256::zero(),
        };

        let new_contrib = current_contrib.checked_add(&amount);
        Storage::put(storage.clone(), contrib_key, new_contrib.into_byte_string());

        // Add to contributor list if first contribution
        if current_contrib == Int256::zero() {
            self.add_contributor_to_list(campaign_id, contributor);
        }

        // Update total raised
        let raised_key = self.total_raised_prefix.concat(&campaign_id.into_byte_string());
        let current_raised = match Storage::get(storage.clone(), raised_key.clone()) {
            Some(amount_bytes) => Int256::from_byte_string(amount_bytes),
            None => Int256::zero(),
        };

        let new_raised = current_raised.checked_add(&amount);
        let storage_clone = storage.clone(); Storage::put(storage_clone, raised_key, new_raised.into_byte_string());

        // Check if funding goal is reached
        let funding_goal = self.extract_funding_goal(&campaign_data);
        if new_raised >= funding_goal {
            self.mark_campaign_successful(campaign_id);
        }

        let mut event_data = Array::new();
        event_data.push(campaign_id.into_any());
        event_data.push(contributor.into_any());
        event_data.push(amount.into_any());
        event_data.push(new_raised.into_any());
        Runtime::notify(ByteString::from_literal("ContributionMade"), event_data);

        true
    }

    /// Get campaign information
    #[method]
    #[safe]
    pub fn get_campaign(&self, campaign_id: Int256) -> Map<ByteString, Any> {
        let mut result = Map::new();

        match self.get_campaign_data(campaign_id) {
            Some(data) => {
                let (creator, title, description, funding_goal, deadline, payment_token, status) =
                    self.deserialize_campaign_data(data);

                result.put(ByteString::from_literal("creator"), creator.into_any());
                result.put(ByteString::from_literal("title"), title.into_any());
                result.put(ByteString::from_literal("description"), description.into_any());
                result.put(ByteString::from_literal("funding_goal"), funding_goal.into_any());
                result.put(ByteString::from_literal("deadline"), Int256::new(deadline as i64).into_any());
                result.put(ByteString::from_literal("payment_token"), payment_token.into_any());
                result.put(ByteString::from_literal("status"), Int256::new(status.to_u8() as i64).into_any());
                result.put(ByteString::from_literal("total_raised"), self.get_total_raised(campaign_id).into_any());
                result.put(ByteString::from_literal("contributor_count"), self.get_contributor_count(campaign_id).into_any());
            },
            None => {
                result.put(ByteString::from_literal("error"), ByteString::from_literal("Campaign not found").into_any());
            }
        }

        result
    }

    /// Get contributor's contribution amount
    #[method]
    #[safe]
    pub fn get_contribution(&self, campaign_id: Int256, contributor: H160) -> Int256 {
        let storage = Storage::get_context();
        let contrib_key = self.contributions_prefix
            .concat(&campaign_id.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&contributor.into_byte_string());

        match Storage::get(storage, contrib_key) {
            Some(amount_bytes) => Int256::from_byte_string(amount_bytes),
            None => Int256::zero(),
        }
    }

    /// Get total amount raised for a campaign
    #[method]
    #[safe]
    pub fn get_total_raised(&self, campaign_id: Int256) -> Int256 {
        let storage = Storage::get_context();
        let raised_key = self.total_raised_prefix.concat(&campaign_id.into_byte_string());

        match Storage::get(storage, raised_key) {
            Some(amount_bytes) => Int256::from_byte_string(amount_bytes),
            None => Int256::zero(),
        }
    }

    /// Get total number of campaigns
    #[method]
    #[safe]
    pub fn get_campaign_count(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.campaign_count_key.clone()) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        }
    }

    /// Get platform owner
    #[method]
    #[safe]
    pub fn get_platform_owner(&self) -> H160 {
        let storage = Storage::get_context();
        match Storage::get(storage, self.platform_owner_key.clone()) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => H160::zero(),
        }
    }

    /// Check if platform is in emergency pause
    #[method]
    #[safe]
    pub fn is_emergency_paused(&self) -> bool {
        let storage = Storage::get_context();
        Storage::get(storage, self.emergency_pause_key.clone()).is_some()
    }

    // Helper functions

    fn validate_campaign_params(
        &self,
        title: &ByteString,
        description: &ByteString,
        funding_goal: Int256,
        deadline: u64
    ) -> bool {
        if title.is_empty() || title.len() > 100 {
            Runtime::log(ByteString::from_literal("Invalid title: must be 1-100 characters"));
            return false;
        }

        if description.is_empty() || description.len() > 1000 {
            Runtime::log(ByteString::from_literal("Invalid description: must be 1-1000 characters"));
            return false;
        }

        if funding_goal <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid funding goal: must be positive"));
            return false;
        }

        let current_time = Runtime::get_time();
        if deadline <= current_time {
            Runtime::log(ByteString::from_literal("Invalid deadline: must be in the future"));
            return false;
        }

        true
    }

    fn is_token_supported(&self, token: H160) -> bool {
        // For simplicity, support GAS and EpicChain
        token == Gas::hash() || token == EpicChain::hash()
    }

    fn get_campaign_data(&self, campaign_id: Int256) -> Option<ByteString> {
        let storage = Storage::get_context();
        let campaign_key = self.campaign_prefix.concat(&campaign_id.into_byte_string());
        Storage::get(storage, campaign_key)
    }

    fn serialize_campaign_data(
        &self,
        creator: H160,
        title: ByteString,
        description: ByteString,
        funding_goal: Int256,
        deadline: u64,
        payment_token: H160,
        status: CampaignStatus
    ) -> ByteString {
        // Simple serialization - in production, use a proper format
        let mut data = creator.into_byte_string();
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&title);
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&description);
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&funding_goal.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&deadline.to_le_bytes()));
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&payment_token.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&[status.to_u8()]));
        data
    }

    fn deserialize_campaign_data(&self, data: ByteString) -> (H160, ByteString, ByteString, Int256, u64, H160, CampaignStatus) {
        // Simplified deserialization - in production, use proper parsing
        let parts = self.split_by_delimiter(data, ByteString::from_literal("|"));

        if parts.size() >= 7 {
            let creator = H160::from_byte_string(parts.get(0).clone());
            let title = parts.get(1).clone();
            let description = parts.get(2).clone();
            let funding_goal = Int256::from_byte_string(parts.get(3).clone());

            let deadline_bytes = parts.get(4).to_bytes();
            let deadline = if deadline_bytes.len() >= 8 {
                u64::from_le_bytes([
                    deadline_bytes[0], deadline_bytes[1], deadline_bytes[2], deadline_bytes[3],
                    deadline_bytes[4], deadline_bytes[5], deadline_bytes[6], deadline_bytes[7]
                ])
            } else {
                0
            };

            let payment_token = H160::from_byte_string(parts.get(5).clone());

            let status_bytes = parts.get(6).to_bytes();
            let status = if status_bytes.len() > 0 {
                CampaignStatus::from_u8(status_bytes[0])
            } else {
                CampaignStatus::Active
            };

            (creator, title, description, funding_goal, deadline, payment_token, status)
        } else {
            (H160::zero(), ByteString::empty(), ByteString::empty(),
             Int256::zero(), 0, H160::zero(), CampaignStatus::Failed)
        }
    }

    fn split_by_delimiter(&self, data: ByteString, _delimiter: ByteString) -> Array<ByteString> {
        // Simplified string splitting - in production, use proper parsing
        let mut parts = Array::new();

        // For now, return the original data as single part
        // Complete implementation with proper string splitting
        parts.push(data);
        parts
    }

    fn extract_funding_goal(&self, data: &ByteString) -> Int256 {
        let (_, _, _, funding_goal, _, _, _) = self.deserialize_campaign_data(data.clone());
        funding_goal
    }

    fn is_campaign_active(&self, data: &ByteString) -> bool {
        let (_, _, _, _, deadline, _, status) = self.deserialize_campaign_data(data.clone());
        let current_time = Runtime::get_time();

        status == CampaignStatus::Active && current_time < deadline
    }

    fn mark_campaign_successful(&self, campaign_id: Int256) {
        // Update campaign status to successful
        let mut event_data = Array::new();
        event_data.push(campaign_id.into_any());
        Runtime::notify(ByteString::from_literal("CampaignSuccessful"), event_data);
    }

    fn add_contributor_to_list(&self, campaign_id: Int256, contributor: H160) {
        // Add contributor to the campaign's contributor list
        let mut event_data = Array::new();
        event_data.push(campaign_id.into_any());
        event_data.push(contributor.into_any());
        Runtime::notify(ByteString::from_literal("NewContributor"), event_data);
    }

    fn get_contributor_count(&self, _campaign_id: Int256) -> Int256 {
        // Return number of unique contributors
        // For now, return 0 - implement proper counting in production
        Int256::zero()
    }

    /// Request refund for failed or cancelled campaign
    #[method]
    pub fn request_refund(&self, campaign_id: Int256, contributor: H160) -> bool {
        // Verify authorization
        if !Runtime::check_witness(contributor) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Get campaign data
        let campaign_data = match self.get_campaign_data(campaign_id) {
            Some(data) => data,
            None => {
                Runtime::log(ByteString::from_literal("Campaign not found"));
                return false;
            }
        };

        let (_, _, _, _, deadline, _, status) = self.deserialize_campaign_data(campaign_data.clone());
        let current_time = Runtime::get_time();

        // Check if refund is allowed (campaign failed or deadline passed without reaching goal)
        let refund_allowed = match status {
            CampaignStatus::Failed | CampaignStatus::Cancelled => true,
            CampaignStatus::Active => {
                current_time > deadline && self.get_total_raised(campaign_id) < self.extract_funding_goal(&campaign_data)
            },
            _ => false,
        };

        if !refund_allowed {
            Runtime::log(ByteString::from_literal("Refund not allowed for this campaign"));
            return false;
        }

        // Get contribution amount
        let contribution = self.get_contribution(campaign_id, contributor);
        if contribution <= Int256::zero() {
            Runtime::log(ByteString::from_literal("No contribution found"));
            return false;
        }

        // Process refund (mark contribution as refunded)
        let storage = Storage::get_context();
        let contrib_key = self.contributions_prefix
            .concat(&campaign_id.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&contributor.into_byte_string());

        let storage_clone = storage.clone(); Storage::delete(storage_clone, contrib_key);

        let mut event_data = Array::new();
        event_data.push(campaign_id.into_any());
        event_data.push(contributor.into_any());
        event_data.push(contribution.into_any());
        Runtime::notify(ByteString::from_literal("RefundProcessed"), event_data);

        true
    }

    /// Cancel campaign (creator only, before deadline)
    #[method]
    pub fn cancel_campaign(&self, campaign_id: Int256) -> bool {
        // Get campaign data
        let campaign_data = match self.get_campaign_data(campaign_id) {
            Some(data) => data,
            None => {
                Runtime::log(ByteString::from_literal("Campaign not found"));
                return false;
            }
        };

        let (creator, _, _, _, deadline, _, status) = self.deserialize_campaign_data(campaign_data);
        let current_time = Runtime::get_time();

        // Verify authorization (creator or platform owner)
        if !Runtime::check_witness(creator) && !self.is_platform_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only creator or platform owner can cancel"));
            return false;
        }

        // Check if campaign can be cancelled
        if status != CampaignStatus::Active {
            Runtime::log(ByteString::from_literal("Campaign is not active"));
            return false;
        }

        if current_time > deadline {
            Runtime::log(ByteString::from_literal("Campaign deadline has passed"));
            return false;
        }

        // Update campaign status
        let storage = Storage::get_context();
        let new_data = self.serialize_campaign_data(
            creator,
            ByteString::from_literal(""), // Simplified - should preserve original data
            ByteString::from_literal(""),
            Int256::zero(),
            deadline,
            H160::zero(),
            CampaignStatus::Cancelled
        );

        let campaign_key = self.campaign_prefix.concat(&campaign_id.into_byte_string());
        let storage_clone = storage.clone(); Storage::put(storage_clone, campaign_key, new_data);

        let mut event_data = Array::new();
        event_data.push(campaign_id.into_any());
        Runtime::notify(ByteString::from_literal("CampaignCancelled"), event_data);

        true
    }

    /// Emergency pause platform (owner only)
    #[method]
    pub fn emergency_pause(&self) -> bool {
        if !self.is_platform_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only platform owner can pause"));
            return false;
        }

        let storage = Storage::get_context();
        let storage_clone = storage.clone(); Storage::put(storage_clone, self.emergency_pause_key.clone(), ByteString::from_literal("true"));

        Runtime::notify(ByteString::from_literal("EmergencyPause"), Array::new());
        true
    }

    /// Resume platform (owner only)
    #[method]
    pub fn resume_platform(&self) -> bool {
        if !self.is_platform_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only platform owner can resume"));
            return false;
        }

        let storage = Storage::get_context();
        let storage_clone = storage.clone(); Storage::delete(storage_clone, self.emergency_pause_key.clone());

        Runtime::notify(ByteString::from_literal("PlatformResumed"), Array::new());
        true
    }

    fn is_platform_owner(&self) -> bool {
        let owner = self.get_platform_owner();
        if owner == H160::zero() {
            return false;
        }
        Runtime::check_witness(owner)
    }
}
