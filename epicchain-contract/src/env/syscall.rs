// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![allow(unused)]

#[cfg(target_family = "wasm")]
use crate::{
    storage::{StorageContext, ReadOnlyStorageContext},
    types::{
        builtin::{
            array::Array,
            string::ByteString,
            h160::H160,
            int256::Int256,
            bytes::Bytes,
        },
        placeholder::Placeholder,
        Any, CallFlags, FindOptions, Notification, PublicKey, Signer, TriggerType, Tx,
    },
};

#[link(wasm_import_module = "epicchain.syscall")]
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
extern "C" {
    /// System.Runtime.GetTrigger
    pub(crate) fn system_runtime_trigger() -> TriggerType;

    /// syscall System.Runtime.Platform
    pub(crate) fn system_runtime_platform() -> ByteString;

    /// syscall System.Runtime.ScriptContainer
    pub(crate) fn system_runtime_tx() -> Tx;

    /// syscall System.Runtime.GetExecutingScriptHash
    pub(crate) fn system_runtime_executing_script_hash() -> H160;

    /// syscall System.Runtime.GetCallingScriptHash
    pub(crate) fn system_runtime_calling_script_hash() -> H160;

    /// syscall System.Runtime.GetEntryScriptHash
    pub(crate) fn system_runtime_entry_script_hash() -> H160;

    /// syscall System.Runtime.GetTime
    pub(crate) fn system_runtime_time() -> u64;

    /// syscall System.Runtime.GetInvocationCounter
    pub(crate) fn system_runtime_invocation_counter() -> u32;

    /// syscall System.Runtime.GasLeft
    pub(crate) fn system_runtime_epicpulse_left() -> Int256;

    /// syscall System.Runtime.GetAddressVersion
    pub(crate) fn system_runtime_address_version() -> u32;

    /// syscall System.Runtime.GetNotifications
    pub(crate) fn system_runtime_notifications() -> Array<Notification>;

    /// syscall System.Runtime.GetNotifications with script hash
    pub(crate) fn system_runtime_get_notifications(script_hash: H160) -> Array<Notification>;

    /// System.Runtime.CheckWitness
    pub(crate) fn system_runtime_check_witness_with_account(account: H160) -> bool;

    /// System.Runtime.CheckWitness
    pub(crate) fn system_runtime_check_witness_with_public_key(public_key: PublicKey) -> bool;

    /// System.Runtime.Log
    pub(crate) fn system_runtime_log(message: ByteString);

    /// System.Runtime.BurnGas
    pub(crate) fn system_runtime_burn_epicpulse(amount: Int256);

    /// System.Runtime.GetRandom
    pub(crate) fn system_runtime_get_random() -> Int256;

    /// System.Runtime.GetNetwork
    pub(crate) fn system_runtime_get_network() -> u32;

    /// System.Runtime.LoadScript
    pub(crate) fn system_runtime_load_script(script_hash: H160, call_flags: CallFlags, args: Array<Any>) -> Any;

    /// System.Runtime.CurrentSigners
    pub(crate) fn system_runtime_current_signers() -> Array<Signer>;

    /// System.Runtime.Notify
    pub(crate) fn system_runtime_notify(name: ByteString, data: Array<Any>);

    /// System.Contract.Call
    pub(crate) fn system_contract_call(
        contract: H160,
        method: ByteString,
        call_flags: CallFlags,
        args: Array<Any>,
    ) -> Any;

    /// System.Contract.GetCallFlags
    pub(crate) fn system_contract_get_call_flags() -> CallFlags;

    /// System.Contract.CreateStandardAccount
    pub(crate) fn system_contract_create_standard_account(public_key: PublicKey) -> H160;

    /// System.Contract.CreateMultisigAccount
    /// m: The number of correct signatures that need to be provided in order for the verification to pass.
    /// public_keys: The public keys
    pub(crate) fn system_contract_create_multi_signs_account(m: u32, public_keys: Array<PublicKey>) -> H160;

    /// System.Crypto.CheckSig
    pub(crate) fn system_crypto_check_sign(public_key: PublicKey, sign: ByteString) -> bool;

    /// System.Crypto.CheckMultisig
    pub(crate) fn system_crypto_check_multi_signs(public_keys: Array<PublicKey>, signs: Array<ByteString>) -> bool;

    /// System.Iterator.Next
    pub(crate) fn system_iterator_next(iterator: Placeholder) -> bool;

    /// System.Iterator.Value
    pub(crate) fn system_iterator_value(iterator: Placeholder) -> Placeholder;

    /// System.Storage.GetContext
    pub(crate) fn system_storage_get_context() -> StorageContext;

    /// System.Storage.GetReadOnlyContext
    pub(crate) fn system_storage_get_readonly_context() -> ReadOnlyStorageContext;

    /// System.Storage.AsReadOnly
    pub(crate) fn system_storage_as_readonly(cx: StorageContext) -> ReadOnlyStorageContext;

    /// System.Storage.Get
    pub(crate) fn system_storage_string_key_get(context: StorageContext, key: ByteString) -> Placeholder;

    /// System.Storage.Get
    pub(crate) fn system_storage_bytes_key_get(context: StorageContext, key: Bytes) -> Placeholder;

    /// System.Storage.Put
    pub(crate) fn system_storage_string_key_put(context: StorageContext, key: ByteString, value: ByteString);

    /// System.Storage.Put
    pub(crate) fn system_storage_bytes_key_put(context: StorageContext, key: Bytes, value: ByteString);

    /// System.Storage.Delete
    pub(crate) fn system_storage_string_key_delete(context: StorageContext, key: ByteString);

    /// System.Storage.Delete
    pub(crate) fn system_storage_bytes_key_delete(context: StorageContext, key: Bytes);

    /// System.Storage.Find
    pub(crate) fn system_storage_string_key_scan_prefix(
        context: StorageContext,
        prefix: ByteString,
        options: FindOptions,
    ) -> Placeholder;

    /// System.Storage.Find
    pub(crate) fn system_storage_bytes_key_scan_prefix(
        context: StorageContext,
        prefix: Bytes,
        options: FindOptions,
    ) -> Placeholder;
}
