// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};

pub struct EpicChain;

impl EpicChain {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn symbol() -> ByteString {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_symbol() }

        #[cfg(not(target_family = "wasm"))]
        ByteString::new("XPR".as_bytes().to_vec())
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn decimals() -> u32 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_decimals() }

        #[cfg(not(target_family = "wasm"))]
        0
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn total_supply() -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_total_supply() }

        #[cfg(not(target_family = "wasm"))]
        Int256::new(1_0000_0000)
    }

    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn balance_of(account: H160) -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_balance_of(account) }

        #[cfg(not(target_family = "wasm"))]
        Int256::new(0) // Mock implementation for non-WASM targets
    }

    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_transfer(from, to, amount) }

        #[cfg(not(target_family = "wasm"))]
        false // Mock implementation for non-WASM targets
    }

    /// Gets the number of EpicPulse generated for each block
    #[inline(always)]
    #[rustfmt::skip]
    pub fn get_epicpulse_per_block() -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_get_epicpulse_per_block() }

        #[cfg(not(target_family = "wasm"))]
        Int256::new(5_00000000) // Mock: 5 EpicPulse per block
    }

    /// Gets the number of unclaimed EpicPulse for an account
    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn unclaimed_epicpulse(account: H160, end_block: u32) -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_unclaimed_epicpulse(account, end_block) }

        #[cfg(not(target_family = "wasm"))]
        Int256::zero() // Mock implementation for non-WASM targets
    }

    /// Registers as a candidate
    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn register_candidate(public_key: PublicKey) -> bool {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_register_candidate(public_key) }

        #[cfg(not(target_family = "wasm"))]
        false // Mock implementation for non-WASM targets
    }

    /// Unregisters as a candidate
    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn unregister_candidate(public_key: PublicKey) -> bool {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_unregister_candidate(public_key) }

        #[cfg(not(target_family = "wasm"))]
        false // Mock implementation for non-WASM targets
    }

    /// Votes for a candidate
    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn vote(account: H160, vote_to: PublicKey) -> bool {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_vote(account, vote_to) }

        #[cfg(not(target_family = "wasm"))]
        false // Mock implementation for non-WASM targets
    }

    /// Unvotes (removes vote)
    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn unvote(account: H160) -> bool {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_unvote(account) }

        #[cfg(not(target_family = "wasm"))]
        false // Mock implementation for non-WASM targets
    }

    /// Gets the candidates list
    #[inline(always)]
    #[rustfmt::skip]
    pub fn get_candidates() -> Array<contract::NeoCandidate> {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_get_candidates() }

        #[cfg(not(target_family = "wasm"))]
        Array::new() // Mock implementation for non-WASM targets
    }

    /// Gets the committee members list
    #[inline(always)]
    #[rustfmt::skip]
    pub fn get_committee() -> Array<PublicKey> {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_get_committee() }

        #[cfg(not(target_family = "wasm"))]
        Array::new() // Mock implementation for non-WASM targets
    }

    /// Gets the validators list for the next block
    #[inline(always)]
    #[rustfmt::skip]
    pub fn get_next_block_validators() -> Array<PublicKey> {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_get_next_block_validators() }

        #[cfg(not(target_family = "wasm"))]
        Array::new() // Mock implementation for non-WASM targets
    }

    /// Gets the account state for the specified account
    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn get_account_state(account: H160) -> contract::NeoAccountState {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_get_account_state(account) }

        #[cfg(not(target_family = "wasm"))]
        contract::NeoAccountState::default() // Mock implementation for non-WASM targets
    }

    /// Gets the candidate votes for a specific public key
    #[inline(always)]
    #[rustfmt::skip]
    #[allow(unused_variables)]
    pub fn get_candidate_votes(public_key: PublicKey) -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_epicchain_get_candidate_votes(public_key) }

        #[cfg(not(target_family = "wasm"))]
        Int256::zero() // Mock implementation for non-WASM targets
    }
}
