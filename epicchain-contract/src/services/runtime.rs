// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[cfg(target_family = "wasm")]
use crate::{env, types::{Any, Array, ByteString, CallFlags, H160, Int256, Notification, PublicKey, Signer, TriggerType, Tx, FromByteString}};

#[cfg(not(target_family = "wasm"))]
use crate::types::{Any, Array, ByteString, CallFlags, H160, Int256, Notification, PublicKey, Signer, TriggerType, Tx};

/// Provides access to the execution environment of smart contracts.
pub struct Runtime;

#[cfg(not(target_family = "wasm"))]
impl Runtime {
    /// Gets the trigger type of the execution.
    #[inline(always)]
    pub fn get_trigger() -> TriggerType {
        // For non-WASM targets (tests), return Application trigger
        TriggerType::Application
    }

    /// Gets the platform name of the execution environment.
    #[inline(always)]
    pub fn get_platform() -> ByteString {
        // For non-WASM targets (tests), return EpicChain platform
        ByteString::from_literal("NEO")
    }

    /// Gets the transaction that triggered the execution.
    #[inline(always)]
    pub fn get_transaction() -> Tx {
        // For non-WASM targets (tests), return default transaction
        Tx::default()
    }

    /// Gets the script hash of the current executing script.
    #[inline(always)]
    pub fn get_executing_script_hash() -> H160 {
        // For non-WASM targets (tests), return zero hash
        H160::zero()
    }

    /// Gets the script hash of the calling script.
    #[inline(always)]
    pub fn get_calling_script_hash() -> H160 {
        // For non-WASM targets (tests), return zero hash
        H160::zero()
    }

    /// Gets the script hash of the entry script.
    #[inline(always)]
    pub fn get_entry_script_hash() -> H160 {
        // For non-WASM targets (tests), return zero hash
        H160::zero()
    }

    /// Gets the timestamp of the current block.
    #[inline(always)]
    pub fn get_time() -> u64 {
        // For non-WASM targets (tests), return current timestamp
        1640995200 // 2022-01-01 00:00:00 UTC as default
    }

    /// Gets the invocation counter of the current contract.
    #[inline(always)]
    pub fn get_invocation_counter() -> u32 {
        // For non-WASM targets (tests), return 1
        1
    }

    /// Gets the remaining GAS that can be spent in this execution.
    #[inline(always)]
    pub fn get_epicpulse_left() -> Int256 {
        // For non-WASM targets (tests), return a large amount
        Int256::new(1_000_000_000i64)
    }

    /// Gets the address version of the current network.
    #[inline(always)]
    pub fn get_address_version() -> u32 {
        // For non-WASM targets (tests), return version 53 (EpicChain mainnet)
        53
    }

    /// Gets the notifications from the specified contract.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn get_notifications(script_hash: Option<H160>) -> Array<Notification> {
        // For non-WASM targets (tests), return empty array
        Array::new()
    }

    /// Determines whether the specified account has witnessed the current transaction.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn check_witness(account: H160) -> bool {
        // For non-WASM targets (tests), return true (allow all)
        true
    }

    /// Determines whether the specified public key has witnessed the current transaction.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn check_witness_with_public_key(public_key: PublicKey) -> bool {
        // For non-WASM targets (tests), return true (allow all)
        true
    }

    /// Writes a log message to the execution log.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn log(message: ByteString) {
        // For non-WASM targets (tests), do nothing (no logging)
    }

    /// Sends a notification to the execution environment.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn notify(event_name: ByteString, state: Array<Any>) {
        // For non-WASM targets (tests), do nothing (no notifications)
    }

    /// Burns the specified amount of GAS.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn burn_epicpulse(amount: Int256) {
        // For non-WASM targets (tests), do nothing (no gas burning)
    }

    /// Gets a random number.
    #[inline(always)]
    pub fn get_random() -> Int256 {
        // For non-WASM targets (tests), return a fixed "random" number
        Int256::new(42)
    }

    /// Gets the network ID of the current network.
    #[inline(always)]
    pub fn get_network() -> u32 {
        // For non-WASM targets (tests), return mainnet ID
        860833102 // EpicChain mainnet
    }

    /// Load and execute a script with the specified call flags and arguments
    ///
    /// # Arguments
    /// * `script` - The script to load and execute
    /// * `call_flags` - Flags controlling the execution context
    /// * `args` - Arguments to pass to the script
    ///
    /// # Returns
    /// The result of script execution
    #[allow(unused_variables)]
    pub fn load_script(script: ByteString, call_flags: CallFlags, args: Array<Any>) -> Any {
        // For non-WASM targets, return a meaningful default
        Any::default()
    }

    /// Gets the current signers of the transaction.
    #[inline(always)]
    pub fn current_signers() -> Array<Signer> {
        // For non-WASM targets (tests), return empty array
        Array::new()
    }
}

#[cfg(target_family = "wasm")]
impl Runtime {
    /// Gets the trigger type of the execution.
    #[inline(always)]
    pub fn get_trigger() -> TriggerType {
        unsafe { env::syscall::system_runtime_trigger() }
    }

    /// Gets the platform name of the execution environment.
    #[inline(always)]
    pub fn get_platform() -> ByteString {
        unsafe { env::syscall::system_runtime_platform() }
    }

    /// Gets the transaction that triggered the execution.
    #[inline(always)]
    pub fn get_transaction() -> Tx {
        unsafe { env::syscall::system_runtime_tx() }
    }

    /// Gets the script hash of the current executing script.
    #[inline(always)]
    pub fn get_executing_script_hash() -> H160 {
        unsafe { env::syscall::system_runtime_executing_script_hash() }
    }

    /// Gets the script hash of the calling script.
    #[inline(always)]
    pub fn get_calling_script_hash() -> H160 {
        unsafe { env::syscall::system_runtime_calling_script_hash() }
    }

    /// Gets the script hash of the entry script.
    #[inline(always)]
    pub fn get_entry_script_hash() -> H160 {
        unsafe { env::syscall::system_runtime_entry_script_hash() }
    }

    /// Gets the timestamp of the current block.
    #[inline(always)]
    pub fn get_time() -> u64 {
        unsafe { env::syscall::system_runtime_time() }
    }

    /// Gets the invocation counter of the current contract.
    #[inline(always)]
    pub fn get_invocation_counter() -> u32 {
        unsafe { env::syscall::system_runtime_invocation_counter() }
    }

    /// Gets the remaining GAS that can be spent in this execution.
    #[inline(always)]
    pub fn get_epicpulse_left() -> Int256 {
        unsafe { env::syscall::system_runtime_epicpulse_left() }
    }

    /// Gets the address version of the current network.
    #[inline(always)]
    pub fn get_address_version() -> u32 {
        unsafe { env::syscall::system_runtime_address_version() }
    }

    /// Gets the notifications from the specified contract.
    #[inline(always)]
    pub fn get_notifications(script_hash: Option<H160>) -> Array<Notification> {
        match script_hash {
            Some(hash) => unsafe { env::syscall::system_runtime_get_notifications(hash) },
            None => unsafe { env::syscall::system_runtime_notifications() },
        }
    }

    /// Determines whether the specified account has witnessed the current transaction.
    #[inline(always)]
    pub fn check_witness(account: H160) -> bool {
        unsafe { env::syscall::system_runtime_check_witness_with_account(account) }
    }

    /// Determines whether the specified public key has witnessed the current transaction.
    #[inline(always)]
    pub fn check_witness_with_public_key(public_key: PublicKey) -> bool {
        unsafe { env::syscall::system_runtime_check_witness_with_public_key(public_key) }
    }

    /// Writes a log message to the execution log.
    #[inline(always)]
    pub fn log(message: ByteString) {
        unsafe { env::syscall::system_runtime_log(message) }
    }

    /// Sends a notification to the execution environment.
    #[inline(always)]
    pub fn notify(event_name: ByteString, state: Array<Any>) {
        unsafe { env::syscall::system_runtime_notify(event_name, state) }
    }

    /// Burns the specified amount of GAS.
    #[inline(always)]
    pub fn burn_epicpulse(amount: Int256) {
        unsafe { env::syscall::system_runtime_burn_epicpulse(amount) }
    }

    /// Gets a random number.
    #[inline(always)]
    pub fn get_random() -> Int256 {
        unsafe { env::syscall::system_runtime_get_random() }
    }

    /// Gets the network ID of the current network.
    #[inline(always)]
    pub fn get_network() -> u32 {
        unsafe { env::syscall::system_runtime_get_network() }
    }

    /// Load and execute a script with the specified call flags and arguments
    ///
    /// # Arguments
    /// * `script` - The script to load and execute
    /// * `call_flags` - Flags controlling the execution context
    /// * `args` - Arguments to pass to the script
    ///
    /// # Returns
    /// The result of script execution
    pub fn load_script(script: ByteString, call_flags: CallFlags, args: Array<Any>) -> Any {
        #[cfg(target_family = "wasm")]
        unsafe {
            // Convert script ByteString to H160 script hash for the syscall
            let script_hash = H160::from_byte_string(script);
            env::syscall::system_runtime_load_script(script_hash, call_flags, args)
        }

        #[cfg(not(target_family = "wasm"))]
        {
            // For non-WASM targets, return a meaningful default
            Any::default()
        }
    }

    /// Gets the current signers of the transaction.
    #[inline(always)]
    pub fn current_signers() -> Array<Signer> {
        unsafe { env::syscall::system_runtime_current_signers() }
    }
}
