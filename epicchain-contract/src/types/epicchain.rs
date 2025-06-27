// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

//! EpicChain-specific types for the EpicChain smart contract framework

use crate::types::{PublicKey, Int256};

/// Represents a candidate in the EpicChain network
#[derive(Debug, Clone, PartialEq)]
pub struct NeoCandidate {
    /// The public key of the candidate
    pub public_key: PublicKey,
    /// The number of votes received by the candidate
    pub votes: Int256,
}

impl NeoCandidate {
    /// Creates a new NeoCandidate
    pub fn new(public_key: PublicKey, votes: Int256) -> Self {
        Self { public_key, votes }
    }
}

impl Default for NeoCandidate {
    fn default() -> Self {
        Self {
            public_key: PublicKey::default(),
            votes: Int256::zero(),
        }
    }
}

/// Represents the account state in the EpicChain network
#[derive(Debug, Clone, PartialEq)]
pub struct NeoAccountState {
    /// The balance of EpicChain tokens
    pub balance: Int256,
    /// The height at which the balance was last updated
    pub balance_height: u32,
    /// The public key that the account voted for (if any)
    pub vote_to: Option<PublicKey>,
}

impl NeoAccountState {
    /// Creates a new NeoAccountState
    pub fn new(balance: Int256, balance_height: u32, vote_to: Option<PublicKey>) -> Self {
        Self {
            balance,
            balance_height,
            vote_to,
        }
    }
}

impl Default for NeoAccountState {
    fn default() -> Self {
        Self {
            balance: Int256::zero(),
            balance_height: 0,
            vote_to: None,
        }
    }
}

/// Transaction attribute types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxAttrType {
    /// High priority attribute
    HighPriority = 0x01,
    /// Oracle response attribute
    OracleResponse = 0x11,
    /// Not valid before attribute
    NotValidBefore = 0x20,
    /// Conflicts attribute
    Conflicts = 0x21,
    /// Not valid after attribute (reserved)
    NotValidAfter = 0x22,
}

impl TxAttrType {
    /// Converts the attribute type to a byte value
    pub fn to_byte(self) -> u8 {
        self as u8
    }

    /// Creates an attribute type from a byte value
    pub fn from_byte(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(TxAttrType::HighPriority),
            0x11 => Some(TxAttrType::OracleResponse),
            0x20 => Some(TxAttrType::NotValidBefore),
            0x21 => Some(TxAttrType::Conflicts),
            0x22 => Some(TxAttrType::NotValidAfter),
            _ => None,
        }
    }
}

/// Role types in the EpicChain network
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// State validator role
    StateValidator = 4,
    /// Oracle role
    Oracle = 8,
    /// EpicChain FS Alphabet role
    NeoFSAlphabet = 16,
    /// P2P notary role
    P2PNotary = 32,
}

impl Role {
    /// Converts the role to a byte value
    pub fn to_byte(self) -> u8 {
        self as u8
    }

    /// Creates a role from a byte value
    pub fn from_byte(value: u8) -> Option<Self> {
        match value {
            4 => Some(Role::StateValidator),
            8 => Some(Role::Oracle),
            16 => Some(Role::NeoFSAlphabet),
            32 => Some(Role::P2PNotary),
            _ => None,
        }
    }
}

/// VM state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmState {
    /// None state
    None = 0,
    /// Halt state (successful execution)
    Halt = 1,
    /// Fault state (execution failed)
    Fault = 2,
    /// Break state (debugging)
    Break = 4,
}

impl VmState {
    /// Converts the VM state to a byte value
    pub fn to_byte(self) -> u8 {
        self as u8
    }

    /// Creates a VM state from a byte value
    pub fn from_byte(value: u8) -> Option<Self> {
        match value {
            0 => Some(VmState::None),
            1 => Some(VmState::Halt),
            2 => Some(VmState::Fault),
            4 => Some(VmState::Break),
            _ => None,
        }
    }

    /// Returns true if the VM state indicates successful execution
    pub fn is_success(self) -> bool {
        matches!(self, VmState::Halt)
    }

    /// Returns true if the VM state indicates failed execution
    pub fn is_fault(self) -> bool {
        matches!(self, VmState::Fault)
    }
}
