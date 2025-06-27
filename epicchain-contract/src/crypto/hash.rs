// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

pub trait Sha256<T> {
    fn sha256(data: &T) -> H256;
}

pub trait Ripemd160<T> {
    fn ripemd160(data: &T) -> H160;
}

pub trait Keccak256<T> {
    fn keccak256(data: &T) -> H256;
}
