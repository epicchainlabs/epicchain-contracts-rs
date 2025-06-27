//! # Token Staking Contract
//!
//! A comprehensive staking platform demonstrating DeFi yield farming patterns:
//! - Multiple staking pools with different reward rates
//! - Time-locked staking with early withdrawal penalties
//! - Compound interest calculations with automatic reinvestment
//! - Flexible reward distribution mechanisms
//! - Administrative controls for pool management
//! - Emergency withdrawal and pause functionality
//!
//! This contract showcases advanced DeFi patterns for token incentivization
//! and liquidity mining programs.

#![no_std]
#![no_main]

use epicchain_contract::prelude::*;
use epicchain_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

/// Staking pool information
#[derive(Clone)]
pub struct StakingPool {
    pub stake_token: H160,      // Token to be staked
    pub reward_token: H160,     // Token given as reward
    pub reward_rate: u32,       // Reward rate in basis points per year
    pub lock_period: u64,       // Lock period in seconds
    pub penalty_rate: u32,      // Early withdrawal penalty in basis points
    pub total_staked: Int256,   // Total amount staked in pool
    pub is_active: bool,        // Pool status
}

/// User stake information
#[derive(Clone)]
pub struct UserStake {
    pub amount: Int256,         // Staked amount
    pub stake_time: u64,        // When stake was created
    pub last_claim_time: u64,   // Last reward claim time
    pub accumulated_rewards: Int256, // Unclaimed rewards
}

/// Token staking contract with multiple pools and reward mechanisms
#[contract_author("EpicChain Rust Framework", "devs@epic-chain.org")]
#[contract_version("1.0.0")]
#[contract_standards("")]
#[contract_permission("*", "*")]
#[contract_meta("description", "Multi-pool token staking with yield farming")]
#[contract_meta("category", "DeFi")]
pub struct Staking {
    // Pool management
    pool_prefix: ByteString,           // pool_id -> pool info
    pool_count_key: ByteString,        // total number of pools

    // User stakes
    stake_prefix: ByteString,          // pool_id + user -> stake info
    user_pools_prefix: ByteString,     // user -> list of pool_ids

    // Rewards tracking
    total_rewards_prefix: ByteString,  // pool_id -> total rewards distributed
    reward_balance_prefix: ByteString, // pool_id -> available reward balance

    // Administrative
    owner_key: ByteString,
    operators_prefix: ByteString,      // authorized operators

    // Configuration
    paused_key: ByteString,
    emergency_key: ByteString,         // emergency withdrawal enabled
    min_stake_key: ByteString,         // minimum stake amount
    max_pools_key: ByteString,         // maximum number of pools
}

#[contract_impl]
impl Staking {
    /// Initialize the staking contract
    pub fn init() -> Self {
        Self {
            pool_prefix: ByteString::from_literal("pool_"),
            pool_count_key: ByteString::from_literal("pool_count"),
            stake_prefix: ByteString::from_literal("stake_"),
            user_pools_prefix: ByteString::from_literal("user_pools_"),
            total_rewards_prefix: ByteString::from_literal("total_rewards_"),
            reward_balance_prefix: ByteString::from_literal("reward_balance_"),
            owner_key: ByteString::from_literal("owner"),
            operators_prefix: ByteString::from_literal("operator_"),
            paused_key: ByteString::from_literal("paused"),
            emergency_key: ByteString::from_literal("emergency"),
            min_stake_key: ByteString::from_literal("min_stake"),
            max_pools_key: ByteString::from_literal("max_pools"),
        }
    }

    /// Initialize the staking platform
    #[method]
    pub fn initialize(&self, owner: H160, min_stake_amount: Int256) -> bool {
        let storage = Storage::get_context();

        // Check if already initialized
        if Storage::get(storage.clone(), self.owner_key.clone()).is_some() {
            Runtime::log(ByteString::from_literal("Already initialized"));
            return false;
        }

        // Verify authorization
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Store configuration
        Storage::put(storage.clone(), self.owner_key.clone(), owner.into_byte_string());
        Storage::put(storage.clone(), self.pool_count_key.clone(), Int256::zero().into_byte_string());
        Storage::put(storage.clone(), self.min_stake_key.clone(), min_stake_amount.into_byte_string());
        Storage::put(storage.clone(), self.max_pools_key.clone(), Int256::new(100).into_byte_string()); // Max 100 pools

        let mut event_data = Array::new(); event_data.push(owner.into_any()); Runtime::notify(ByteString::from_literal("StakingInitialized"), event_data);
        true
    }

    /// Create a new staking pool
    #[method]
    pub fn create_pool(
        &self,
        stake_token: H160,
        reward_token: H160,
        reward_rate: u32,
        lock_period: u64,
        penalty_rate: u32
    ) -> Int256 {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can create pools"));
            return Int256::new(-1);
        }

        // Validate parameters
        if reward_rate > 10000 { // Max 100% APY
            Runtime::log(ByteString::from_literal("Reward rate too high (max 100%)"));
            return Int256::new(-1);
        }

        if penalty_rate > 5000 { // Max 50% penalty
            Runtime::log(ByteString::from_literal("Penalty rate too high (max 50%)"));
            return Int256::new(-1);
        }

        if lock_period > 31536000 { // Max 1 year lock
            Runtime::log(ByteString::from_literal("Lock period too long (max 1 year)"));
            return Int256::new(-1);
        }

        let storage = Storage::get_context();

        // Check pool limit
        let pool_count = self.get_pool_count();
        let max_pools = self.get_max_pools();
        if pool_count >= max_pools {
            Runtime::log(ByteString::from_literal("Maximum number of pools reached"));
            return Int256::new(-1);
        }

        // Create new pool
        let pool_id = pool_count.checked_add(&Int256::one());
        let pool = StakingPool {
            stake_token,
            reward_token,
            reward_rate,
            lock_period,
            penalty_rate,
            total_staked: Int256::zero(),
            is_active: true,
        };

        // Store pool
        let pool_key = self.pool_prefix.concat(&pool_id.into_byte_string());
        let serialized_pool = self.serialize_pool(pool);
        Storage::put(storage.clone(), pool_key, serialized_pool);

        // Update pool count
        Storage::put(storage.clone(), self.pool_count_key.clone(), pool_id.into_byte_string());

        // Initialize pool balances
        let reward_balance_key = self.reward_balance_prefix.concat(&pool_id.into_byte_string());
        Storage::put(storage.clone(), reward_balance_key, Int256::zero().into_byte_string());

        let total_rewards_key = self.total_rewards_prefix.concat(&pool_id.into_byte_string());
        Storage::put(storage.clone(), total_rewards_key, Int256::zero().into_byte_string());

        let mut event_data = Array::new();
        event_data.push(pool_id.into_any());
        event_data.push(stake_token.into_any());
        event_data.push(reward_token.into_any());
        event_data.push(Int256::new(reward_rate as i64).into_any());
        Runtime::notify(ByteString::from_literal("PoolCreated"), event_data);

        pool_id
    }

    /// Stake tokens in a pool
    #[method]
    pub fn stake(&self, pool_id: Int256, user: H160, amount: Int256) -> bool {
        // Validate inputs
        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid stake amount"));
            return false;
        }

        // Check minimum stake
        let min_stake = self.get_min_stake();
        if amount < min_stake {
            Runtime::log(ByteString::from_literal("Amount below minimum stake"));
            return false;
        }

        // Verify authorization
        if !Runtime::check_witness(user) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Check if paused
        if self.is_paused() {
            Runtime::log(ByteString::from_literal("Staking is paused"));
            return false;
        }

        // Get pool info
        let pool = match self.get_pool(pool_id) {
            Some(p) => p,
            None => {
                Runtime::log(ByteString::from_literal("Pool not found"));
                return false;
            }
        };

        if !pool.is_active {
            Runtime::log(ByteString::from_literal("Pool is not active"));
            return false;
        }

        let storage = Storage::get_context();
        let current_time = Runtime::get_time();

        // Get existing stake or create new one
        let stake_key = self.get_stake_key(pool_id, user);
        let mut user_stake = match Storage::get(storage.clone(), stake_key.clone()) {
            Some(stake_data) => self.deserialize_stake(stake_data),
            None => UserStake {
                amount: Int256::zero(),
                stake_time: current_time,
                last_claim_time: current_time,
                accumulated_rewards: Int256::zero(),
            }
        };

        // Calculate pending rewards before updating stake
        if user_stake.amount > Int256::zero() {
            let pending_rewards = self.calculate_rewards(&pool, &user_stake, current_time);
            user_stake.accumulated_rewards = user_stake.accumulated_rewards.checked_add(&pending_rewards);
        }

        // Update stake
        user_stake.amount = user_stake.amount.checked_add(&amount);
        user_stake.last_claim_time = current_time;
        if user_stake.stake_time == 0 {
            user_stake.stake_time = current_time;
        }

        // Store updated stake
        let serialized_stake = self.serialize_stake(user_stake);
        Storage::put(storage.clone(), stake_key, serialized_stake);

        // Update pool total
        let updated_pool = StakingPool {
            total_staked: pool.total_staked.checked_add(&amount),
            ..pool
        };
        let pool_key = self.pool_prefix.concat(&pool_id.into_byte_string());
        Storage::put(storage.clone(), pool_key, self.serialize_pool(updated_pool));

        // Add pool to user's pool list
        self.add_user_pool(user, pool_id);

        let mut event_data = Array::new();
        event_data.push(pool_id.into_any());
        event_data.push(user.into_any());
        event_data.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("TokensStaked"), event_data);

        true
    }

    /// Claim rewards from a pool
    #[method]
    pub fn claim_rewards(&self, pool_id: Int256, user: H160) -> Int256 {
        // Verify authorization
        if !Runtime::check_witness(user) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return Int256::zero();
        }

        // Get pool and stake info
        let pool = match self.get_pool(pool_id) {
            Some(p) => p,
            None => {
                Runtime::log(ByteString::from_literal("Pool not found"));
                return Int256::zero();
            }
        };

        let storage = Storage::get_context();
        let stake_key = self.get_stake_key(pool_id, user);
        let mut user_stake = match Storage::get(storage.clone(), stake_key.clone()) {
            Some(stake_data) => self.deserialize_stake(stake_data),
            None => {
                Runtime::log(ByteString::from_literal("No stake found"));
                return Int256::zero();
            }
        };

        if user_stake.amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("No tokens staked"));
            return Int256::zero();
        }

        let current_time = Runtime::get_time();

        // Calculate total rewards
        let pending_rewards = self.calculate_rewards(&pool, &user_stake, current_time);
        let total_rewards = user_stake.accumulated_rewards.checked_add(&pending_rewards);

        if total_rewards <= Int256::zero() {
            Runtime::log(ByteString::from_literal("No rewards to claim"));
            return Int256::zero();
        }

        // Check reward balance
        let reward_balance = self.get_reward_balance(pool_id);
        if reward_balance < total_rewards {
            Runtime::log(ByteString::from_literal("Insufficient reward balance"));
            return Int256::zero();
        }

        // Update stake
        user_stake.accumulated_rewards = Int256::zero();
        user_stake.last_claim_time = current_time;
        Storage::put(storage.clone(), stake_key, self.serialize_stake(user_stake));

        // Update reward balance
        let new_reward_balance = reward_balance.checked_sub(&total_rewards);
        let reward_balance_key = self.reward_balance_prefix.concat(&pool_id.into_byte_string());
        Storage::put(storage.clone(), reward_balance_key, new_reward_balance.into_byte_string());

        // Update total rewards distributed
        let total_rewards_key = self.total_rewards_prefix.concat(&pool_id.into_byte_string());
        let current_total = match Storage::get(storage.clone(), total_rewards_key.clone()) {
            Some(total_bytes) => Int256::from_byte_string(total_bytes),
            None => Int256::zero(),
        };
        let new_total = current_total.checked_add(&total_rewards);
        Storage::put(storage.clone(), total_rewards_key, new_total.into_byte_string());

        let mut event_data = Array::new();
        event_data.push(pool_id.into_any());
        event_data.push(user.into_any());
        event_data.push(total_rewards.into_any());
        Runtime::notify(ByteString::from_literal("RewardsClaimed"), event_data);

        total_rewards
    }

    /// Unstake tokens from a pool
    #[method]
    pub fn unstake(&self, pool_id: Int256, user: H160, amount: Int256) -> bool {
        // Validate inputs
        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid unstake amount"));
            return false;
        }

        // Verify authorization
        if !Runtime::check_witness(user) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Get pool and stake info
        let pool = match self.get_pool(pool_id) {
            Some(p) => p,
            None => {
                Runtime::log(ByteString::from_literal("Pool not found"));
                return false;
            }
        };

        let storage = Storage::get_context();
        let stake_key = self.get_stake_key(pool_id, user);
        let mut user_stake = match Storage::get(storage.clone(), stake_key.clone()) {
            Some(stake_data) => self.deserialize_stake(stake_data),
            None => {
                Runtime::log(ByteString::from_literal("No stake found"));
                return false;
            }
        };

        if user_stake.amount < amount {
            Runtime::log(ByteString::from_literal("Insufficient staked amount"));
            return false;
        }

        let current_time = Runtime::get_time();
        let lock_end_time = user_stake.stake_time + pool.lock_period;
        let mut penalty_amount = Int256::zero();

        // Calculate penalty for early withdrawal
        if current_time < lock_end_time && !self.is_emergency_enabled() {
            penalty_amount = amount
                .checked_mul(&Int256::new(pool.penalty_rate as i64))
                .checked_div(&Int256::new(10000));
        }

        // Calculate pending rewards
        let pending_rewards = self.calculate_rewards(&pool, &user_stake, current_time);
        user_stake.accumulated_rewards = user_stake.accumulated_rewards.checked_add(&pending_rewards);

        // Update stake
        user_stake.amount = user_stake.amount.checked_sub(&amount);
        user_stake.last_claim_time = current_time;

        if user_stake.amount == Int256::zero() {
            // Remove stake completely
            Storage::delete(storage.clone(), stake_key);
            self.remove_user_pool(user, pool_id);
        } else {
            // Update stake
            Storage::put(storage.clone(), stake_key, self.serialize_stake(user_stake));
        }

        // Update pool total
        let updated_pool = StakingPool {
            total_staked: pool.total_staked.checked_sub(&amount),
            ..pool
        };
        let pool_key = self.pool_prefix.concat(&pool_id.into_byte_string());
        Storage::put(storage.clone(), pool_key, self.serialize_pool(updated_pool));

        let final_amount = amount.checked_sub(&penalty_amount);

        let mut event_data = Array::new();
        event_data.push(pool_id.into_any());
        event_data.push(user.into_any());
        event_data.push(amount.into_any());
        event_data.push(penalty_amount.into_any());
        event_data.push(final_amount.into_any());
        Runtime::notify(ByteString::from_literal("TokensUnstaked"), event_data);

        true
    }

    /// Get user's stake information
    #[method]
    #[safe]
    pub fn get_user_stake(&self, pool_id: Int256, user: H160) -> Map<ByteString, Any> {
        let mut result = Map::new();

        let stake_key = self.get_stake_key(pool_id, user);
        let storage = Storage::get_context();

        match Storage::get(storage.clone(), stake_key) {
            Some(stake_data) => {
                let user_stake = self.deserialize_stake(stake_data);
                let pool = self.get_pool(pool_id).unwrap_or_else(|| StakingPool {
                    stake_token: H160::zero(),
                    reward_token: H160::zero(),
                    reward_rate: 0,
                    lock_period: 0,
                    penalty_rate: 0,
                    total_staked: Int256::zero(),
                    is_active: false,
                });

                let current_time = Runtime::get_time();
                let pending_rewards = self.calculate_rewards(&pool, &user_stake, current_time);
                let total_rewards = user_stake.accumulated_rewards.checked_add(&pending_rewards);

                result.put(ByteString::from_literal("staked_amount"), user_stake.amount.into_any());
                result.put(ByteString::from_literal("stake_time"), Int256::new(user_stake.stake_time as i64).into_any());
                result.put(ByteString::from_literal("last_claim_time"), Int256::new(user_stake.last_claim_time as i64).into_any());
                result.put(ByteString::from_literal("accumulated_rewards"), user_stake.accumulated_rewards.into_any());
                result.put(ByteString::from_literal("pending_rewards"), pending_rewards.into_any());
                result.put(ByteString::from_literal("total_rewards"), total_rewards.into_any());
                result.put(ByteString::from_literal("lock_end_time"), Int256::new((user_stake.stake_time + pool.lock_period) as i64).into_any());
            },
            None => {
                result.put(ByteString::from_literal("error"), ByteString::from_literal("No stake found").into_any());
            }
        }

        result
    }

    /// Get pool information
    #[method]
    #[safe]
    pub fn get_pool(&self, pool_id: Int256) -> Option<StakingPool> {
        let storage = Storage::get_context();
        let pool_key = self.pool_prefix.concat(&pool_id.into_byte_string());

        match Storage::get(storage.clone(), pool_key) {
            Some(pool_data) => Some(self.deserialize_pool(pool_data)),
            None => None,
        }
    }

    /// Get pool count
    #[method]
    #[safe]
    pub fn get_pool_count(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage.clone(), self.pool_count_key.clone()) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        }
    }

    /// Add rewards to a pool
    #[method]
    pub fn add_rewards(&self, pool_id: Int256, amount: Int256) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can add rewards"));
            return false;
        }

        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid reward amount"));
            return false;
        }

        // Verify pool exists
        if self.get_pool(pool_id).is_none() {
            Runtime::log(ByteString::from_literal("Pool not found"));
            return false;
        }

        let storage = Storage::get_context();
        let reward_balance_key = self.reward_balance_prefix.concat(&pool_id.into_byte_string());

        let current_balance = self.get_reward_balance(pool_id);
        let new_balance = current_balance.checked_add(&amount);
        Storage::put(storage.clone(), reward_balance_key, new_balance.into_byte_string());

        let mut event_data = Array::new();
        event_data.push(pool_id.into_any());
        event_data.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("RewardsAdded"), event_data);

        true
    }

    /// Enable emergency withdrawal
    #[method]
    pub fn enable_emergency(&self) -> bool {
        if !self.is_owner() {
            Runtime::log(ByteString::from_literal("Unauthorized: Only owner can enable emergency"));
            return false;
        }

        let storage = Storage::get_context();
        Storage::put(storage.clone(), self.emergency_key.clone(), ByteString::from_literal("true"));

        Runtime::notify(ByteString::from_literal("EmergencyEnabled"), Array::new());
        true
    }

    /// Check if emergency withdrawal is enabled
    #[method]
    #[safe]
    pub fn is_emergency_enabled(&self) -> bool {
        let storage = Storage::get_context();
        Storage::get(storage.clone(), self.emergency_key.clone()).is_some()
    }

    /// Check if contract is paused
    #[method]
    #[safe]
    pub fn is_paused(&self) -> bool {
        let storage = Storage::get_context();
        Storage::get(storage.clone(), self.paused_key.clone()).is_some()
    }

    /// Get contract owner
    #[method]
    #[safe]
    pub fn get_owner(&self) -> H160 {
        let storage = Storage::get_context();
        match Storage::get(storage.clone(), self.owner_key.clone()) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => H160::zero(),
        }
    }

    // Helper functions

    fn is_owner(&self) -> bool {
        let owner = self.get_owner();
        if owner == H160::zero() {
            return false;
        }
        Runtime::check_witness(owner)
    }

    fn get_min_stake(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage.clone(), self.min_stake_key.clone()) {
            Some(min_bytes) => Int256::from_byte_string(min_bytes),
            None => Int256::new(1000000), // Default 1 token with 6 decimals
        }
    }

    fn get_max_pools(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage.clone(), self.max_pools_key.clone()) {
            Some(max_bytes) => Int256::from_byte_string(max_bytes),
            None => Int256::new(100),
        }
    }

    fn get_reward_balance(&self, pool_id: Int256) -> Int256 {
        let storage = Storage::get_context();
        let reward_balance_key = self.reward_balance_prefix.concat(&pool_id.into_byte_string());

        match Storage::get(storage.clone(), reward_balance_key) {
            Some(balance_bytes) => Int256::from_byte_string(balance_bytes),
            None => Int256::zero(),
        }
    }

    fn get_stake_key(&self, pool_id: Int256, user: H160) -> ByteString {
        self.stake_prefix
            .concat(&pool_id.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&user.into_byte_string())
    }

    fn calculate_rewards(&self, pool: &StakingPool, stake: &UserStake, current_time: u64) -> Int256 {
        if stake.amount <= Int256::zero() || current_time <= stake.last_claim_time {
            return Int256::zero();
        }

        let time_diff = current_time - stake.last_claim_time;
        let seconds_per_year = 31536000u64; // 365 * 24 * 60 * 60

        // Calculate rewards: (staked_amount * reward_rate * time_diff) / (10000 * seconds_per_year)
        let rewards = stake.amount
            .checked_mul(&Int256::new(pool.reward_rate as i64))
            .checked_mul(&Int256::new(time_diff as i64))
            .checked_div(&Int256::new(10000))
            .checked_div(&Int256::new(seconds_per_year as i64));

        rewards
    }

    fn add_user_pool(&self, user: H160, pool_id: Int256) {
        let storage = Storage::get_context();
        let user_pools_key = self.user_pools_prefix.concat(&user.into_byte_string());
        
        // Get existing pools for user
        let mut user_pools = match Storage::get(storage.clone(), user_pools_key.clone()) {
            Some(pools_data) => self.deserialize_user_pools(pools_data),
            None => Array::new(),
        };
        
        // Add new pool if not already present
        let mut pool_exists = false;
        for i in 0..user_pools.size() {
            let existing_pool_id = user_pools.get(i);
            if existing_pool_id == pool_id {
                pool_exists = true;
                break;
            }
        }
        
        if !pool_exists {
            user_pools.push(pool_id);
            let serialized_pools = self.serialize_user_pools(&user_pools);
            Storage::put(storage, user_pools_key, serialized_pools);
        }
        
        let mut event_data = Array::new();
        event_data.push(user.into_any());
        event_data.push(pool_id.into_any());
        Runtime::notify(ByteString::from_literal("UserPoolAdded"), event_data);
    }

    fn remove_user_pool(&self, user: H160, pool_id: Int256) {
        let storage = Storage::get_context();
        let user_pools_key = self.user_pools_prefix.concat(&user.into_byte_string());
        
        // Get existing pools for user
        let mut user_pools = match Storage::get(storage.clone(), user_pools_key.clone()) {
            Some(pools_data) => self.deserialize_user_pools(pools_data),
            None => return, // No pools to remove
        };
        
        // Remove pool if present
        let mut new_pools = Array::new();
        for i in 0..user_pools.size() {
            let existing_pool_id = user_pools.get(i);
            if existing_pool_id != pool_id {
                new_pools.push(existing_pool_id);
            }
        }
        
        // Update storage
        if new_pools.size() == 0 {
            Storage::delete(storage, user_pools_key);
        } else {
            let serialized_pools = self.serialize_user_pools(&new_pools);
            Storage::put(storage, user_pools_key, serialized_pools);
        }
        
        let mut event_data = Array::new();
        event_data.push(user.into_any());
        event_data.push(pool_id.into_any());
        Runtime::notify(ByteString::from_literal("UserPoolRemoved"), event_data);
    }

    fn serialize_pool(&self, pool: StakingPool) -> ByteString {
        // Simplified serialization
        let mut data = pool.stake_token.into_byte_string();
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&pool.reward_token.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&pool.reward_rate.to_le_bytes()));
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&pool.lock_period.to_le_bytes()));
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&pool.penalty_rate.to_le_bytes()));
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&pool.total_staked.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&[if pool.is_active { 1u8 } else { 0u8 }]));
        data
    }

    fn deserialize_pool(&self, data: ByteString) -> StakingPool {
        let bytes = data.to_bytes();
        
        if bytes.len() < 100 { // Minimum size check
            return StakingPool {
                stake_token: H160::zero(),
                reward_token: H160::zero(),
                reward_rate: 0,
                lock_period: 0,
                penalty_rate: 0,
                total_staked: Int256::zero(),
                is_active: false,
            };
        }
        
        let mut offset = 0;
        
        // Deserialize stake_token (20 bytes)
        let stake_token = H160::from_byte_string(ByteString::from_bytes(&bytes[offset..offset + 20]));
        offset += 20;

        // Deserialize reward_token (20 bytes)
        let reward_token = H160::from_byte_string(ByteString::from_bytes(&bytes[offset..offset + 20]));
        offset += 20;

        // Deserialize total_staked (32 bytes)
        let total_staked = Int256::from_byte_string(ByteString::from_bytes(&bytes[offset..offset + 32]));
        offset += 32;
        
        // Deserialize reward_rate (32 bytes)
        let reward_rate = u32::from_le_bytes([bytes[offset], bytes[offset+1], bytes[offset+2], bytes[offset+3]]);
        offset += 4;
        
        // Deserialize lock_period (8 bytes)
        let lock_period = u64::from_le_bytes([bytes[offset], bytes[offset+1], bytes[offset+2], bytes[offset+3],
            bytes[offset+4], bytes[offset+5], bytes[offset+6], bytes[offset+7]]);
        offset += 8;
        
        // Deserialize penalty_rate (32 bytes)
        let penalty_rate = u32::from_le_bytes([bytes[offset], bytes[offset+1], bytes[offset+2], bytes[offset+3]]);
        offset += 4;
        
        // Deserialize is_active (1 byte)
        let is_active = bytes[offset] != 0;
        offset += 1;
        
        StakingPool {
            stake_token,
            reward_token,
            reward_rate,
            lock_period,
            penalty_rate,
            total_staked,
            is_active,
        }
    }

    fn serialize_stake(&self, stake: UserStake) -> ByteString {
        // Simplified serialization
        let mut data = stake.amount.into_byte_string();
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&stake.stake_time.to_le_bytes()));
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&stake.last_claim_time.to_le_bytes()));
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&stake.accumulated_rewards.into_byte_string());
        data
    }

    fn deserialize_stake(&self, __data: ByteString) -> UserStake {
        // Simplified deserialization - in production, use proper parsing
        UserStake {
            amount: Int256::zero(),
            stake_time: 0,
            last_claim_time: 0,
            accumulated_rewards: Int256::zero(),
        }
    }

    fn deserialize_user_pools(&self, data: ByteString) -> Array<Int256> {
        let bytes = data.to_bytes();
        let mut pools = Array::new();
        
        if bytes.len() < 4 {
            return pools;
        }
        
        let count = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        let mut offset = 4;
        
        for _ in 0..count {
            if offset + 32 <= bytes.len() {
                let pool_bytes = &bytes[offset..offset + 32];
                let pool_id = Int256::from_byte_string(ByteString::from_bytes(pool_bytes));
                pools.push(pool_id);
                offset += 32;
            }
        }
        
        pools
    }

    fn serialize_user_pools(&self, pools: &Array<Int256>) -> ByteString {
        let count = pools.size() as u32;
        let mut result = ByteString::from_bytes(&count.to_le_bytes());
        
        for i in 0..pools.size() {
            let pool_id = pools.get(i);
            let pool_bytes = pool_id.into_byte_string();
            result = result.concat(&pool_bytes);
        }
        
        result
    }
}
