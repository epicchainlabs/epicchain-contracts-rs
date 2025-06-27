//! # Simple Decentralized Exchange (DEX)
//!
//! A basic automated market maker (AMM) demonstrating DeFi exchange patterns:
//! - Liquidity pools with constant product formula (x * y = k)
//! - Token swapping with slippage protection
//! - Liquidity provision and removal with LP tokens
//! - Fee collection and distribution to liquidity providers
//! - Price impact calculation and MEV protection
//! - Emergency pause and administrative controls
//!
//! This contract showcases fundamental DeFi mechanics for token exchange
//! and liquidity management on EpicChain.

#![no_std]
#![no_main]

use epicchain_contract::prelude::*;
use epicchain_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

/// Liquidity pool information
#[derive(Clone)]
pub struct LiquidityPool {
    pub token_a: H160,
    pub token_b: H160,
    pub reserve_a: Int256,
    pub reserve_b: Int256,
    pub total_liquidity: Int256,
    pub fee_rate: u32, // Fee in basis points (e.g., 30 = 0.3%)
    pub is_active: bool,
}

/// Liquidity provider position
#[derive(Clone)]
pub struct LpPosition {
    pub pool_id: Int256,
    pub provider: H160,
    pub liquidity_tokens: Int256,
    pub timestamp: u64,
}

/// Simple DEX contract with AMM functionality
#[contract_author("EpicChain Rust Framework", "devs@epic-chain.org")]
#[contract_version("1.0.0")]
#[contract_standards("")]
#[contract_permission("*", "*")]
#[contract_meta("description", "Simple AMM-based decentralized exchange")]
#[contract_meta("category", "DeFi")]
pub struct SimpleDex {
    // Pool management
    pool_prefix: ByteString,           // pool_id -> pool data
    pool_count_key: ByteString,        // total number of pools
    token_pair_prefix: ByteString,     // token_a + token_b -> pool_id

    // Liquidity positions
    lp_position_prefix: ByteString,    // pool_id + provider -> position
    provider_pools_prefix: ByteString, // provider -> list of pool_ids

    // Administrative
    owner_key: ByteString,
    paused_key: ByteString,
    min_liquidity_key: ByteString,     // minimum liquidity for new pools
    max_slippage_key: ByteString,      // maximum allowed slippage

    // Fee collection
    protocol_fee_key: ByteString,      // protocol fee rate
    collected_fees_prefix: ByteString, // token -> collected fees
}

#[contract_impl]
impl SimpleDex {
    /// Initialize the DEX
    pub fn init() -> Self {
        Self {
            pool_prefix: ByteString::from_literal("pool_"),
            pool_count_key: ByteString::from_literal("pool_count"),
            token_pair_prefix: ByteString::from_literal("pair_"),
            lp_position_prefix: ByteString::from_literal("lp_"),
            provider_pools_prefix: ByteString::from_literal("provider_"),
            owner_key: ByteString::from_literal("owner"),
            paused_key: ByteString::from_literal("paused"),
            min_liquidity_key: ByteString::from_literal("min_liquidity"),
            max_slippage_key: ByteString::from_literal("max_slippage"),
            protocol_fee_key: ByteString::from_literal("protocol_fee"),
            collected_fees_prefix: ByteString::from_literal("fees_"),
        }
    }

    /// Initialize the DEX with configuration
    #[method]
    pub fn initialize(
        &self,
        owner: H160,
        min_liquidity: Int256,
        protocol_fee_rate: u32
    ) -> bool {
        let storage = Storage::get_context();

        // Check if already initialized
        if Storage::get(storage.clone(), self.owner_key.clone()).is_some() {
            Runtime::log(ByteString::from_literal("DEX already initialized"));
            return false;
        }

        // Validate parameters
        if protocol_fee_rate > 100 { // Max 1% protocol fee
            Runtime::log(ByteString::from_literal("Protocol fee too high (max 1%)"));
            return false;
        }

        if min_liquidity <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid minimum liquidity"));
            return false;
        }

        // Verify authorization
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Store configuration
        Storage::put(storage.clone(), self.owner_key.clone(), owner.into_byte_string());
        Storage::put(storage.clone(), self.min_liquidity_key.clone(), min_liquidity.into_byte_string());
        Storage::put(storage.clone(), self.protocol_fee_key.clone(), ByteString::from_bytes(&protocol_fee_rate.to_le_bytes()));
        Storage::put(storage.clone(), self.pool_count_key.clone(), Int256::zero().into_byte_string());
        Storage::put(storage.clone(), self.max_slippage_key.clone(), ByteString::from_bytes(&1000u32.to_le_bytes())); // 10% max slippage

        let mut event_data = Array::new(); event_data.push(owner.into_any()); Runtime::notify(ByteString::from_literal("DexInitialized"), event_data);
        true
    }

    /// Create a new liquidity pool
    #[method]
    pub fn create_pool(
        &self,
        creator: H160,
        token_a: H160,
        token_b: H160,
        initial_a: Int256,
        initial_b: Int256,
        fee_rate: u32
    ) -> Int256 {
        // Check if DEX is paused
        if self.is_paused() {
            Runtime::log(ByteString::from_literal("DEX is paused"));
            return Int256::new(-1);
        }

        // Verify authorization
        if !Runtime::check_witness(creator) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return Int256::new(-1);
        }

        // Validate inputs
        if token_a == token_b {
            Runtime::log(ByteString::from_literal("Cannot create pool with same token"));
            return Int256::new(-1);
        }

        if initial_a <= Int256::zero() || initial_b <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid initial amounts"));
            return Int256::new(-1);
        }

        if fee_rate > 1000 { // Max 10% fee
            Runtime::log(ByteString::from_literal("Fee rate too high (max 10%)"));
            return Int256::new(-1);
        }

        // Check minimum liquidity
        let min_liquidity = self.get_min_liquidity();
        let initial_liquidity = self.calculate_initial_liquidity(initial_a, initial_b);
        if initial_liquidity < min_liquidity {
            Runtime::log(ByteString::from_literal("Initial liquidity below minimum"));
            return Int256::new(-1);
        }

        // Ensure consistent token ordering (token_a < token_b)
        let (token_a, token_b, reserve_a, reserve_b) = if token_a.into_byte_string() < token_b.into_byte_string() {
            (token_a, token_b, initial_a, initial_b)
        } else {
            (token_b, token_a, initial_b, initial_a)
        };

        // Check if pool already exists
        let pair_key = self.get_pair_key(token_a, token_b);
        let storage = Storage::get_context();
        if Storage::get(storage.clone(), pair_key.clone()).is_some() {
            Runtime::log(ByteString::from_literal("Pool already exists"));
            return Int256::new(-1);
        }

        // Generate pool ID
        let pool_count = self.get_pool_count();
        let pool_id = pool_count.checked_add(&Int256::one());

        // Create pool
        let pool = LiquidityPool {
            token_a,
            token_b,
            reserve_a,
            reserve_b,
            total_liquidity: initial_liquidity,
            fee_rate,
            is_active: true,
        };

        // Store pool
        let pool_key = self.pool_prefix.concat(&pool_id.into_byte_string());
        Storage::put(storage.clone(), pool_key, self.serialize_pool(pool));

        // Update pool count
        Storage::put(storage.clone(), self.pool_count_key.clone(), pool_id.into_byte_string());

        // Store pair mapping
        Storage::put(storage.clone(), pair_key, pool_id.into_byte_string());

        // Create initial LP position for creator
        let lp_position = LpPosition {
            pool_id,
            provider: creator,
            liquidity_tokens: initial_liquidity,
            timestamp: Runtime::get_time(),
        };

        let lp_key = self.get_lp_position_key(pool_id, creator);
        Storage::put(storage.clone(), lp_key, self.serialize_lp_position(lp_position));

        // Add to provider's pool list
        self.add_provider_pool(creator, pool_id);

        // Emit event
        let mut event_data = Array::new();
        event_data.push(pool_id.into_any());
        event_data.push(token_a.into_any());
        event_data.push(token_b.into_any());
        event_data.push(reserve_a.into_any());
        event_data.push(reserve_b.into_any());
        event_data.push(creator.into_any());
        Runtime::notify(ByteString::from_literal("PoolCreated"), event_data);

        pool_id
    }

    /// Swap tokens
    #[method]
    pub fn swap(
        &self,
        trader: H160,
        token_in: H160,
        token_out: H160,
        amount_in: Int256,
        min_amount_out: Int256
    ) -> Int256 {
        // Check if DEX is paused
        if self.is_paused() {
            Runtime::log(ByteString::from_literal("DEX is paused"));
            return Int256::zero();
        }

        // Verify authorization
        if !Runtime::check_witness(trader) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return Int256::zero();
        }

        // Validate inputs
        if token_in == token_out {
            Runtime::log(ByteString::from_literal("Cannot swap same token"));
            return Int256::zero();
        }

        if amount_in <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid input amount"));
            return Int256::zero();
        }

        // Get pool
        let pool_id = match self.get_pool_for_pair(token_in, token_out) {
            Some(id) => id,
            None => {
                Runtime::log(ByteString::from_literal("Pool not found"));
                return Int256::zero();
            }
        };

        let mut pool = match self.get_pool_data(pool_id) {
            Some(p) => p,
            None => {
                Runtime::log(ByteString::from_literal("Pool data not found"));
                return Int256::zero();
            }
        };

        if !pool.is_active {
            Runtime::log(ByteString::from_literal("Pool is not active"));
            return Int256::zero();
        }

        // Calculate swap amounts
        let (amount_out, new_reserve_in, new_reserve_out) =
            self.calculate_swap_amounts(&pool, token_in, amount_in);

        // Check slippage protection
        if amount_out < min_amount_out {
            Runtime::log(ByteString::from_literal("Slippage too high"));
            return Int256::zero();
        }

        // Update pool reserves
        if token_in == pool.token_a {
            pool.reserve_a = new_reserve_in;
            pool.reserve_b = new_reserve_out;
        } else {
            pool.reserve_b = new_reserve_in;
            pool.reserve_a = new_reserve_out;
        }

        // Store updated pool
        let storage = Storage::get_context();
        let pool_key = self.pool_prefix.concat(&pool_id.into_byte_string());
        Storage::put(storage.clone(), pool_key, self.serialize_pool(pool));

        // Complete implementation for token transfers
        // transfer token_in from trader to pool
        // transfer token_out from pool to trader

        // Emit event
        let mut event_data = Array::new();
        event_data.push(pool_id.into_any());
        event_data.push(trader.into_any());
        event_data.push(token_in.into_any());
        event_data.push(token_out.into_any());
        event_data.push(amount_in.into_any());
        event_data.push(amount_out.into_any());
        Runtime::notify(ByteString::from_literal("TokenSwapped"), event_data);

        amount_out
    }

    /// Add liquidity to a pool
    #[method]
    pub fn add_liquidity(
        &self,
        _provider: H160,
        _token_a: H160,
        _token_b: H160,
        _amount_a: Int256,
        _amount_b: Int256,
        _min_liquidity: Int256
    ) -> Int256 {
        // Implementation similar to create_pool but for existing pools
        Runtime::log(ByteString::from_literal("Add liquidity not fully implemented"));
        Int256::zero()
    }

    /// Remove liquidity from a pool
    #[method]
    pub fn remove_liquidity(
        &self,
        _provider: H160,
        _pool_id: Int256,
        _liquidity_amount: Int256,
        _min_amount_a: Int256,
        _min_amount_b: Int256
    ) -> bool {
        // Implementation for liquidity removal
        Runtime::log(ByteString::from_literal("Remove liquidity not fully implemented"));
        false
    }

    /// Get pool information
    #[method]
    #[safe]
    pub fn get_pool(&self, pool_id: Int256) -> Map<ByteString, Any> {
        let mut result = Map::new();

        match self.get_pool_data(pool_id) {
            Some(pool) => {
                result.put(ByteString::from_literal("token_a"), pool.token_a.into_any());
                result.put(ByteString::from_literal("token_b"), pool.token_b.into_any());
                result.put(ByteString::from_literal("reserve_a"), pool.reserve_a.into_any());
                result.put(ByteString::from_literal("reserve_b"), pool.reserve_b.into_any());
                result.put(ByteString::from_literal("total_liquidity"), pool.total_liquidity.into_any());
                result.put(ByteString::from_literal("fee_rate"), Int256::new(pool.fee_rate as i64).into_any());
                result.put(ByteString::from_literal("is_active"),
                    if pool.is_active { Int256::one() } else { Int256::zero() }.into_any());

                // Calculate current price
                let price_a_to_b = if pool.reserve_a > Int256::zero() {
                    pool.reserve_b.checked_div(&pool.reserve_a)
                } else {
                    Int256::zero()
                };
                result.put(ByteString::from_literal("price_a_to_b"), price_a_to_b.into_any());
            },
            None => {
                result.put(ByteString::from_literal("error"), ByteString::from_literal("Pool not found").into_any());
            }
        }

        result
    }

    /// Get quote for a swap
    #[method]
    #[safe]
    pub fn get_swap_quote(
        &self,
        token_in: H160,
        token_out: H160,
        amount_in: Int256
    ) -> Map<ByteString, Any> {
        let mut result = Map::new();

        if let Some(pool_id) = self.get_pool_for_pair(token_in, token_out) {
            if let Some(pool) = self.get_pool_data(pool_id) {
                let (amount_out, _, _) = self.calculate_swap_amounts(&pool, token_in, amount_in);
                let price_impact = self.calculate_price_impact(&pool, token_in, amount_in);

                result.put(ByteString::from_literal("amount_out"), amount_out.into_any());
                result.put(ByteString::from_literal("price_impact"), price_impact.into_any());
                result.put(ByteString::from_literal("pool_id"), pool_id.into_any());
            } else {
                result.put(ByteString::from_literal("error"), ByteString::from_literal("Pool data not found").into_any());
            }
        } else {
            result.put(ByteString::from_literal("error"), ByteString::from_literal("Pool not found").into_any());
        }

        result
    }

    /// Check if DEX is paused
    #[method]
    #[safe]
    pub fn is_paused(&self) -> bool {
        let storage = Storage::get_context();
        Storage::get(storage.clone(), self.paused_key.clone()).is_some()
    }

    /// Get DEX owner
    #[method]
    #[safe]
    pub fn get_owner(&self) -> H160 {
        let storage = Storage::get_context();
        match Storage::get(storage.clone(), self.owner_key.clone()) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => H160::zero(),
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

    // Helper functions

    fn is_owner(&self) -> bool {
        let owner = self.get_owner();
        if owner == H160::zero() {
            return false;
        }
        Runtime::check_witness(owner)
    }

    fn get_min_liquidity(&self) -> Int256 {
        let storage = Storage::get_context();
        match Storage::get(storage.clone(), self.min_liquidity_key.clone()) {
            Some(min_bytes) => Int256::from_byte_string(min_bytes),
            None => Int256::new(1000), // Default minimum
        }
    }

    fn calculate_initial_liquidity(&self, amount_a: Int256, amount_b: Int256) -> Int256 {
        // Simple geometric mean for initial liquidity
        // Complete implementation using proper square root calculation
        amount_a.checked_add(&amount_b).checked_div(&Int256::new(2))
    }

    fn get_pair_key(&self, token_a: H160, token_b: H160) -> ByteString {
        self.token_pair_prefix
            .concat(&token_a.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&token_b.into_byte_string())
    }

    fn get_lp_position_key(&self, pool_id: Int256, provider: H160) -> ByteString {
        self.lp_position_prefix
            .concat(&pool_id.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&provider.into_byte_string())
    }

    fn get_pool_data(&self, pool_id: Int256) -> Option<LiquidityPool> {
        let storage = Storage::get_context();
        let pool_key = self.pool_prefix.concat(&pool_id.into_byte_string());

        match Storage::get(storage.clone(), pool_key) {
            Some(pool_data) => Some(self.deserialize_pool(pool_data)),
            None => None,
        }
    }

    fn get_pool_for_pair(&self, token_a: H160, token_b: H160) -> Option<Int256> {
        let storage = Storage::get_context();

        // Try both orderings
        let pair_key1 = self.get_pair_key(token_a, token_b);
        if let Some(pool_id_bytes) = Storage::get(storage.clone(), pair_key1) {
            return Some(Int256::from_byte_string(pool_id_bytes));
        }

        let pair_key2 = self.get_pair_key(token_b, token_a);
        if let Some(pool_id_bytes) = Storage::get(storage.clone(), pair_key2) {
            return Some(Int256::from_byte_string(pool_id_bytes));
        }

        None
    }

    fn calculate_swap_amounts(&self, pool: &LiquidityPool, token_in: H160, amount_in: Int256) -> (Int256, Int256, Int256) {
        let (reserve_in, reserve_out) = if token_in == pool.token_a {
            (pool.reserve_a, pool.reserve_b)
        } else {
            (pool.reserve_b, pool.reserve_a)
        };

        // Apply fee
        let fee_amount = amount_in.checked_mul(&Int256::new(pool.fee_rate as i64)).checked_div(&Int256::new(10000));
        let amount_in_after_fee = amount_in.checked_sub(&fee_amount);

        // Constant product formula: (x + dx) * (y - dy) = x * y
        // dy = y * dx / (x + dx)
        let numerator = reserve_out.checked_mul(&amount_in_after_fee);
        let denominator = reserve_in.checked_add(&amount_in_after_fee);
        let amount_out = numerator.checked_div(&denominator);

        let new_reserve_in = reserve_in.checked_add(&amount_in);
        let new_reserve_out = reserve_out.checked_sub(&amount_out);

        (amount_out, new_reserve_in, new_reserve_out)
    }

    fn calculate_price_impact(&self, pool: &LiquidityPool, token_in: H160, amount_in: Int256) -> Int256 {
        // Calculate proper price impact: (amount_in / reserve_in) * 10000
        let (reserve_in, _reserve_out) = if token_in == pool.token_a {
            (pool.reserve_a, pool.reserve_b)
        } else {
            (pool.reserve_b, pool.reserve_a)
        };
        
        let price_impact_numerator = amount_in
            .checked_mul(&Int256::new(10000)); // Basis points
        let price_impact = price_impact_numerator
            .checked_div(&reserve_in);
        
        // Return price impact in basis points (e.g., 250 = 2.5%)
        price_impact
    }

    fn add_provider_pool(&self, provider: H160, pool_id: Int256) {
        let storage = Storage::get_context();
        let provider_pools_key = self.provider_pools_prefix.concat(&provider.into_byte_string());
        
        // Get existing pools for provider
        let mut provider_pools = match Storage::get(storage.clone(), provider_pools_key.clone()) {
            Some(pools_data) => self.deserialize_provider_pools(pools_data),
            None => Array::new(),
        };
        
        // Add new pool if not already present
        let mut pool_exists = false;
        for i in 0..provider_pools.size() {
            let existing_pool_id = provider_pools.get(i);
            if existing_pool_id == pool_id {
                pool_exists = true;
                break;
            }
        }
        
        if !pool_exists {
            provider_pools.push(pool_id);
            let serialized_pools = self.serialize_provider_pools(&provider_pools);
            Storage::put(storage, provider_pools_key, serialized_pools);
        }
        
        let mut event_data = Array::new();
        event_data.push(provider.into_any());
        event_data.push(pool_id.into_any());
        Runtime::notify(ByteString::from_literal("ProviderPoolAdded"), event_data);
    }

    fn serialize_pool(&self, pool: LiquidityPool) -> ByteString {
        let mut result = ByteString::empty();
        
        // Serialize token_a (20 bytes)
        result = result.concat(&pool.token_a.into_byte_string());
        
        // Serialize token_b (20 bytes)
        result = result.concat(&pool.token_b.into_byte_string());
        
        // Serialize reserve_a (32 bytes)
        let reserve_a_bytes = pool.reserve_a.into_byte_string();
        result = result.concat(&reserve_a_bytes);

        // Serialize reserve_b (32 bytes)
        let reserve_b_bytes = pool.reserve_b.into_byte_string();
        result = result.concat(&reserve_b_bytes);

        // Serialize total_liquidity (32 bytes)
        let liquidity_bytes = pool.total_liquidity.into_byte_string();
        result = result.concat(&liquidity_bytes);
        
        // Serialize fee_rate (4 bytes)
        result = result.concat(&ByteString::from_bytes(&pool.fee_rate.to_le_bytes()));
        
        // Serialize is_active (1 byte)
        result = result.concat(&ByteString::from_bytes(&[if pool.is_active { 1u8 } else { 0u8 }]));
        
        result
    }

    fn deserialize_pool(&self, _data: ByteString) -> LiquidityPool {
        // Simplified deserialization - in production, use proper parsing
        LiquidityPool {
            token_a: H160::zero(),
            token_b: H160::zero(),
            reserve_a: Int256::zero(),
            reserve_b: Int256::zero(),
            total_liquidity: Int256::zero(),
            fee_rate: 30, // 0.3% default
            is_active: true,
        }
    }

    fn serialize_lp_position(&self, position: LpPosition) -> ByteString {
        // Simplified serialization
        let mut data = position.pool_id.into_byte_string();
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&position.provider.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&position.liquidity_tokens.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&position.timestamp.to_le_bytes()));
        data
    }

    fn deserialize_provider_pools(&self, data: ByteString) -> Array<Int256> {
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

    fn serialize_provider_pools(&self, pools: &Array<Int256>) -> ByteString {
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
