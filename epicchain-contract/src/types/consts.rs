// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerType {
    /// OnPersist indicates that the contract is triggered by the system
    /// to execute the OnPersist method of the native contracts.
    OnPersist = 0x01,

    /// PostPersist indicates that the contract is triggered by the system
    ///  to execute the PostPersist method of the native contracts.
    PostPersist = 0x02,

    /// Verification indicates that the contract is triggered by the verification.
    Verification = 0x20,

    /// Application indicates that the contract is triggered by the execution of transactions.
    Application = 0x40,

    /// System indicates the combination of all system triggers.
    System = 0x01 | 0x02,

    /// All indicates the combination of all triggers.
    All = 0x01 | 0x02 | 0x20 | 0x40,
}

impl Default for TriggerType {
    fn default() -> Self {
        TriggerType::Application
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallFlags {
    /// None indicates that no flags are set.
    None = 0x00,

    /// ReadStates indicates that the script is allowed to read states.
    ReadStates = 0x01,

    /// WriteStates indicates that the script is allowed to write states.
    WriteStates = 0x02,

    /// AllowCall indicates that the script is allowed to call other contracts.
    AllowCall = 0x04,

    /// AllowNotify indicates that the script is allowed to send notifications.
    AllowNotify = 0x08,

    /// States indicates that the script is allowed to read and write states.
    States = 0x01 | 0x02,

    /// ReadOnly indicates that the script is allowed to read states and call other contracts.
    ReadOnly = 0x01 | 0x04,

    /// All indicates all flags are set.
    All = 0x01 | 0x02 | 0x04 | 0x08,
}

impl Default for CallFlags {
    fn default() -> Self {
        CallFlags::All
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WitnessScope {
    /// None indicates that no witness is required.
    None = 0x00,

    /// Indicates that the calling contract must be the entry contract.
    /// The witness/permission/signature given on first invocation will automatically expire if entering deeper internal invokes.
    /// This can be the default safe choice for native NEO/GAS (previously used on EpicChain 2 as "attach" mode).
    CalledByEntry = 0x01,

    /// Custom hash for contract-specific
    CustomContracts = 0x10,

    /// Custom public key for group members.
    CustomGroups = 0x20,

    /// Indicates that the current context must satisfy the specified rules
    WitnessRules = 0x40,

    /// This allows the witness in all contexts (default Neo2 behavior).
    Global = 0x80,
}

impl Default for WitnessScope {
    fn default() -> Self {
        WitnessScope::CalledByEntry
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WitnessRuleAction {
    Deny = 0x00,
    Allow = 0x01,
}

impl Default for WitnessRuleAction {
    fn default() -> Self {
        WitnessRuleAction::Allow
    }
}

#[repr(u32)]
pub enum FindOptions {
    None = 0,
    KeysOnly = 1 << 0,
    RemovePrefix = 1 << 1,
    ValuesOnly = 1 << 2,
    DeserializeValues = 1 << 3,
    PickField0 = 1 << 4,
    PickField1 = 1 << 5,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WitnessConditionType {
    /// Indicates that the condition will always be met or not met. i.e. Boolean
    Bool = 0x00,

    /// Reverse another condition.
    Not = 0x01,

    /// Indicates that all conditions must be met.
    And = 0x02,

    /// Indicates that any of the conditions meets.
    Or = 0x03,

    /// Indicates that the condition is met when the current context has the specified script hash.
    ScriptHash = 0x18,

    /// Indicates that the condition is met when the current context has the specified group.
    Group = 0x19,

    /// Indicates that the condition is met when the current context is the entry point or is called by the entry point.
    CalledByEntry = 0x20,

    /// Indicates that the condition is met when the current context is called by the specified contract.
    CalledByContract = 0x28,

    /// Indicates that the condition is met when the current context is called by the specified group.
    CalledByGroup = 0x29,
}

impl Default for WitnessConditionType {
    fn default() -> Self {
        WitnessConditionType::Bool
    }
}

#[repr(u32)]
pub enum NamedCurveHash {
    /// The secp256k1 curve and SHA256 hash algorithm.
    Secp256k1SHA256 = 22,

    /// The secp256r1 curve, which known as prime256v1 or nistP-256, and SHA256 hash algorithm.
    Secp256r1SHA256 = 23,

    /// The secp256k1 curve and Keccak256 hash algorithm.
    Secp256k1Keccak256 = 122,

    /// The secp256r1 curve, which known as prime256v1 or nistP-256, and Keccak256 hash algorithm.
    Secp256r1Keccak256 = 123,
}

/// VM execution states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // May be used in future implementations
pub enum VmState {
    None = 0,
    Halt = 1,
    Fault = 2,
    Break = 4,
}

impl VmState {
    /// Convert from u8 value
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => VmState::None,
            1 => VmState::Halt,
            2 => VmState::Fault,
            4 => VmState::Break,
            _ => VmState::None,
        }
    }

    /// Convert to u8 value
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

#[repr(u32)]
pub enum OracleResponseCode {
    /// Indicates that the request has been successfully completed.
    Success = 0x00,

    /// Indicates that the protocol of the request is not supported.
    ProtocolNotSupported = 0x10,

    /// Indicates that the oracle nodes cannot reach a consensus on the result of the request.
    ConsensusUnreachable = 0x12,

    /// Indicates that the requested Uri does not exist.
    NotFound = 0x14,

    /// Indicates that the request was not completed within the specified time.
    Timeout = 0x16,

    /// Indicates that there is no permission to request the resource.
    Forbidden = 0x18,

    /// Indicates that the data for the response is too large.
    ResponseTooLarge = 0x1a,

    /// Indicates that the request failed due to insufficient balance.
    InsufficientFunds = 0x1c,

    /// Indicates that the content-type of the request is not supported.
    ContentTypeNotSupported = 0x1f,

    /// Indicates that the request failed due to other errors.
    Error = 0xff,
}

/// Designation roles for committee members
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // May be used in future implementations
pub enum Role {
    StateValidator = 4,
    Oracle = 8,
    NeoFSAlphabet = 16,
    P2PNotary = 32,
}

#[repr(u32)]
pub enum TxAttrType {
    /// Indicates that the transaction is of high priority.
    HighPriority = 0x01,

    /// Indicates that the transaction is an oracle response.
    OracleResponse = 0x11,

    /// Indicates that the transaction is not valid before <see cref="NotValidBefore.Height"/>.
    NotValidBefore = 0x20,

    /// Indicates that the transaction conflicts with <see cref="Conflicts.Hash"/>.
    Conflicts = 0x21,
}

#[repr(u32)]
pub enum ContractParamType {
    Any = 0x00,

    /// i.e. Boolean
    Bool = 0x10,

    /// i.e. Integer
    Int = 0x11,

    /// i.e. ByteArray
    Bytes = 0x12,

    /// i.e. ByteString
    String = 0x13,

    Hash160 = 0x14,

    Hash256 = 0x15,

    PublicKey = 0x16,

    /// i.e. Signature
    Sign = 0x17,

    Array = 0x20,

    Map = 0x22,

    /// i.e.InteropInterface
    Interop = 0x30,

    Void = 0xff,
}
