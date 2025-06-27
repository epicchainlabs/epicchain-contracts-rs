// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![allow(dead_code)]

use crate::types::{consts::OracleResponseCode, *};

/// NEP-24: Royalty Standard for Non-Fungible Tokens
///
/// This trait defines the interface for royalty information in NFT contracts.
/// It allows creators to receive royalty payments when their NFTs are sold.
pub trait Nep24 {
    /// Returns royalty information for a given token
    ///
    /// # Arguments
    ///
    /// * `token_id` - The unique identifier of the token
    /// * `royalty_token` - The token contract hash used for royalty payment
    /// * `sale_price` - The sale price of the token
    ///
    /// # Returns
    ///
    /// An array of maps containing royalty recipient and amount information.
    /// Each map should contain:
    /// - "royaltyRecipient": H160 address of the royalty recipient
    /// - "royaltyAmount": Int256 amount to be paid as royalty
    fn royalty_info(
        token_id: ByteString,
        royalty_token: H160,
        sale_price: Int256
    ) -> Array<Map<ByteString, Any>>;
}

pub trait Nep26 {
    fn on_nep11_payment(from: H160, amount: Int256, token_id: ByteString);

    // fn on_nep17_payment_with_data(from: H160, amount: Int256, data: Any);
}

pub trait Nep27 {
    fn on_nep17_payment(from: H160, amount: Int256);

    // fn on_nep17_payment_with_data(from: H160, amount: Int256, data: Any);
}

pub trait Nep28 {
    fn _deploy(data: Any, is_update: bool);
}

pub trait Nep30 {
    fn verify(args: Array<Any>) -> bool;
}

pub trait OnOracleResponse {
    fn on_oracle_response(
        request_url: ByteString,
        request_user_data: Any,
        response_code: OracleResponseCode,
        response_json: ByteString,
    );
}
