// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
#[derive(Default)]
pub struct Signer {
    account: H160,
    scopes: WitnessScope,
    allowed_contracts: Array<H160>,
    allowed_groups: Array<PublicKey>,
    rules: Array<WitnessRule>,
}

#[repr(C)]
#[derive(Default)]
pub struct WitnessRule {
    action: WitnessRuleAction,
    condition: WitnessCondition,
}

#[repr(C)]
#[derive(Default)]
pub struct WitnessCondition {
    condition_type: WitnessConditionType,
    condition: i32, // placeholder, and cannot use directly
}

#[repr(C)]
#[allow(dead_code)] // May be used in future implementations
pub struct AndCondition {
    expressions: Array<WitnessCondition>,
}

#[repr(C)]
#[allow(dead_code)] // May be used in future implementations
pub struct OrCondition {
    expressions: Array<WitnessCondition>,
}

#[repr(C)]
#[allow(dead_code)] // May be used in future implementations
pub struct NotCondition {
    expression: WitnessCondition,
}

#[repr(C)]
#[allow(dead_code)] // May be used in future implementations
pub struct BoolCondition {
    expression: bool,
}

#[repr(C)]
#[allow(dead_code)] // May be used in future implementations
pub struct CalledByContractCondition {
    hash: H160, // contract hash
}

#[repr(C)]
#[allow(dead_code)] // May be used in future implementations
pub struct CalledByGroupCondition {
    group: PublicKey, // group public key
}

#[repr(C)]
#[allow(dead_code)] // May be used in future implementations
pub struct GroupCondition {
    group: PublicKey, // group public key
}

#[repr(C)]
#[allow(dead_code)] // May be used in future implementations
pub struct ScriptHashCondition {
    hash: H160, // script hash
}
