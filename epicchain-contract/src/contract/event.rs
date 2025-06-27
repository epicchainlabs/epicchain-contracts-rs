// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

pub trait OnNep11Transfer {
    fn on_nep11_transfer(from: H160, to: H160, amount: Int256, token_id: ByteString);
}

pub trait OnNep17Transfer {
    fn on_nep17_transfer(from: H160, to: H160, amount: Int256);
}

pub trait PostNep11Transfer {
    fn post_nep11_transfer(from: H160, to: H160, amount: Int256, token_id: ByteString);
}

pub trait PostNep17Transfer {
    fn post_nep17_transfer(from: H160, to: H160, amount: Int256);
}
