// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
#[allow(dead_code)] // May be used in future implementations
pub struct ContractHash {
    id: u32,
    hash: H160,
}

#[repr(C)]
pub struct Contract {
    id: u32,
    update_counter: u32,
    hash: H160,
    nef: ByteString,
    manifest: ContractManifest,
}

impl Contract {
    #[inline(always)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[inline(always)]
    pub fn update_counter(&self) -> u32 {
        self.update_counter
    }

    #[inline(always)]
    pub fn hash(&self) -> H160 {
        self.hash
    }

    #[inline(always)]
    pub fn nef(&self) -> ByteString {
        self.nef.clone()
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            id: 0,
            update_counter: 0,
            hash: H160::zero(),
            nef: ByteString::empty(),
            manifest: ContractManifest::default(),
        }
    }
}

impl Default for ContractManifest {
    fn default() -> Self {
        Self {
            name: ByteString::empty(),
            groups: Array::new(),
            _reserved: Any::default(),
            supported_standards: Array::new(),
            abi: ContractAbi::default(),
            permissions: Array::new(),
            trusts: Array::new(),
            extra: ByteString::empty(),
        }
    }
}

impl Default for ContractAbi {
    fn default() -> Self {
        Self {
            methods: Array::new(),
            events: Array::new(),
        }
    }
}

#[repr(C)]
pub struct ContractManifest {
    name: ByteString,
    groups: Array<ContractGroup>,
    _reserved: Any,
    supported_standards: Array<ByteString>,
    abi: ContractAbi,
    permissions: Array<ContractPermission>,
    trusts: Array<ByteString>,
    extra: ByteString,
}

#[repr(C)]
pub struct ContractGroup {
    public_key: PublicKey,
    sign: ByteString,
}

impl Default for ContractGroup {
    fn default() -> Self {
        Self {
            public_key: PublicKey::default(),
            sign: ByteString::empty(),
        }
    }
}

#[repr(C)]
pub struct ContractPermission {
    contract: ByteString,
    methods: Array<ByteString>,
}

impl Default for ContractPermission {
    fn default() -> Self {
        Self {
            contract: ByteString::empty(),
            methods: Array::new(),
        }
    }
}

#[repr(C)]
pub struct ContractAbi {
    methods: Array<ContractMethodDescriptor>,
    events: Array<ContractEventDescriptor>,
}

#[repr(C)]
pub struct ContractMethodDescriptor {
    name: ByteString,
    params: Array<ContractParam>,
    return_type: ContractParamType,
    offset: u32,
    safe: bool,
}

impl Default for ContractMethodDescriptor {
    fn default() -> Self {
        Self {
            name: ByteString::empty(),
            params: Array::new(),
            return_type: ContractParamType::Any,
            offset: 0,
            safe: false,
        }
    }
}

#[repr(C)]
pub struct ContractEventDescriptor {
    name: ByteString,
    params: Array<ContractParam>,
}

impl Default for ContractEventDescriptor {
    fn default() -> Self {
        Self {
            name: ByteString::empty(),
            params: Array::new(),
        }
    }
}

#[repr(C)]
pub struct ContractParam {
    name: ByteString,
    param_type: ContractParamType,
}

impl Default for ContractParam {
    fn default() -> Self {
        Self {
            name: ByteString::empty(),
            param_type: ContractParamType::Any,
        }
    }
}

#[repr(C)]
pub struct NeoCandidate {
    public_key: PublicKey,
    votes: Int256,
}

impl NeoCandidate {
    #[inline(always)]
    pub fn public_key(&self) -> PublicKey {
        self.public_key.clone()
    }

    #[inline(always)]
    pub fn votes(&self) -> Int256 {
        self.votes
    }
}

#[repr(C)]
pub struct NeoAccountState {
    balance: Int256,
    height: Int256,
    vote_to: PublicKey,
}

impl NeoAccountState {
    #[inline(always)]
    pub fn balance(&self) -> Int256 {
        self.balance
    }

    #[inline(always)]
    pub fn height(&self) -> Int256 {
        self.height
    }

    #[inline(always)]
    pub fn vote_to(&self) -> PublicKey {
        self.vote_to.clone()
    }
}

impl Default for NeoAccountState {
    fn default() -> Self {
        Self {
            balance: Int256::zero(),
            height: Int256::zero(),
            vote_to: PublicKey::default(),
        }
    }
}
