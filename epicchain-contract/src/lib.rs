// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod contract;
pub mod crypto;
pub mod env;
pub mod event;
#[macro_use]
pub mod macros;
pub mod runtime;
pub mod serialize;
pub mod services;
pub mod smart_contract;
pub mod storage;
pub mod types;
pub mod native;

pub use epicchain_contract_proc_macros::{contract, structs};

// Export contract annotations
pub use epicchain_contract_proc_macros::{
    method,
    safe as safe_attr,
    contract_author,
    contract_permission,
    contract_standards,
    contract_version,
    contract_meta,
    contract_impl,
};

// Modules re-exported for convenience
pub mod prelude {
    // Contract module
    pub use crate::contract::{call, create_multi_signs_account, create_standard_account, get_call_flags};
    pub use crate::contract::nep17::{Nep17Token, PREFIX_BALANCE, TOTAL_SUPPLY_KEY};
    pub use crate::contract::nep11::{Nep11Token, TokenState};

    // Crypto module
    pub use crate::crypto::{check_multi_signs, check_sign};

    // Native contracts
    pub use crate::native::{gas, neo};

    // Runtime services
    pub use crate::runtime::{
        abort, abort_with_message, burn_epicpulse, get_calling_script_hash,
        check_witness_with_account, check_witness_with_public_key, current_signers,
        get_entry_script_hash, get_executing_script_hash,
        get_epicpulse_left, get_address_version, get_invocation_counter, get_network,
        get_notifications, get_platform, get_random, get_time, get_trigger, load_script, log, notify,
        get_tx as tx,
    };

    // Smart contract base
    pub use crate::smart_contract::SmartContract;

    // Services
    pub use crate::services::contract::Contract;
    pub use crate::services::crypto::Crypto;
    pub use crate::services::event::Event;
    pub use crate::services::iterator::Iterator;
    pub use crate::services::runtime::Runtime;
    pub use crate::services::storage::Storage;

    // Storage
    pub use crate::storage::{StorageContext, StorageItem, StorageMap};

    // Types
    pub use crate::types::{
        Array, Bytes, H160, Int256, Map, ByteString,
        Any, CallFlags, FindOptions, Notification, PublicKey, Signer, TriggerType, Tx,
    };

    // Macros
    pub use epicchain_contract_proc_macros::*;
}
