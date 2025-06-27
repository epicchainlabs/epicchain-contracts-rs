// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod builtin;

pub(crate) mod block;
pub(crate) mod consts;
pub(crate) mod contract;
pub(crate) mod key;
pub(crate) mod neo;
pub(crate) mod notification;
pub(crate) mod signer;
pub mod storage;
pub(crate) mod tx;

pub mod placeholder;

// Export specific types instead of using glob imports
pub use block::Block;
pub use consts::{CallFlags, FindOptions, TriggerType, WitnessScope, WitnessRuleAction, WitnessConditionType, ContractParamType, OracleResponseCode, NamedCurveHash};
pub use contract::Contract;
pub use key::PublicKey;
pub use epicchain::{NeoCandidate, NeoAccountState, TxAttrType, Role, VmState};
pub use notification::Notification;
pub use signer::Signer;
pub use storage::{FindOptions as StorageFindOptions, StorageItem};
pub use tx::Tx;

// Re-export types from builtin
pub use builtin::{
    array::Array,
    string::ByteString,
    h160::H160,
    h256::H256,
    int256::Int256,
    map::Map,
};

// Re-export Any type
pub use builtin::any::Any;

// Re-export Bytes
pub use builtin::bytes::Bytes;

// Re-export other types
pub use builtin::buffer::Buffer;
pub use builtin::interop::Interop;
pub use builtin::nullable::Nullable;
pub use builtin::primitive::Primitive;

// Re-export traits
pub use builtin::{
    IntoByteString,
    FromByteString,
};
