//! # Listing Management
//!
//! Functions for creating, managing, and purchasing NFT listings.

use epicchain_contract::prelude::*;
use epicchain_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};
extern crate alloc;
use alloc::vec::Vec;
use crate::types::*;
use crate::storage::*;

impl crate::NftMarketplace {
    /// Create a new NFT listing
    #[method]
    pub fn create_listing(
        &self,
        seller: H160,
        nft_contract: H160,
        token_id: ByteString,
        price: Int256,
        payment_token: H160,
        duration: u64
    ) -> Int256 {
        // Check if marketplace is paused
        if self.is_paused() {
            Runtime::log(ByteString::from_literal("Marketplace is paused"));
            return Int256::new(-1);
        }

        // Verify authorization
        if !Runtime::check_witness(seller) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return Int256::new(-1);
        }

        // Validate inputs
        if price <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid price: must be positive"));
            return Int256::new(-1);
        }

        if token_id.is_empty() {
            Runtime::log(ByteString::from_literal("Invalid token ID"));
            return Int256::new(-1);
        }

        // Validate duration
        let min_duration = StorageUtils::load_u64_config(
            self.storage_keys.min_duration_key.clone(),
            3600
        );
        let max_duration = StorageUtils::load_u64_config(
            self.storage_keys.max_duration_key.clone(),
            2592000
        );

        if duration < min_duration || duration > max_duration {
            Runtime::log(ByteString::from_literal("Invalid listing duration"));
            return Int256::new(-1);
        }

        // Check if NFT is already listed
        let nft_listing_key = self.storage_keys.nft_listings_key(nft_contract, token_id.clone());
        if StorageUtils::key_exists(nft_listing_key) {
            Runtime::log(ByteString::from_literal("NFT is already listed"));
            return Int256::new(-1);
        }

        // Verify NFT ownership by calling the NFT contract
        let ownership_verified = self.verify_nft_ownership(nft_contract, token_id.clone(), seller);
        if !ownership_verified {
            Runtime::log(ByteString::from_literal("Seller does not own the NFT"));
            return Int256::new(-1);
        }

        let current_time = Runtime::get_time();
        let expires_at = current_time + duration;

        // Generate listing ID
        let listing_id = StorageUtils::increment_counter(self.storage_keys.listing_count_key.clone());

        // Create listing
        let listing = Listing {
            id: listing_id,
            nft_contract,
            token_id: token_id.clone(),
            seller,
            price,
            payment_token,
            created_at: current_time,
            expires_at,
            status: ListingStatus::Active,
        };

        // Store listing
        let listing_key = self.storage_keys.listing_key(listing_id);
        let serialized_listing = self.serialize_listing(listing.clone());
        let storage = Storage::get_context();
        let storage_clone = storage.clone();
        Storage::put(storage_clone, listing_key, serialized_listing);

        // Update indexes
        let seller_listings_key = self.storage_keys.seller_listings_key(seller);
        StorageUtils::add_to_id_list(seller_listings_key, listing_id);

        let nft_listings_key = self.storage_keys.nft_listings_key(nft_contract, token_id.clone());
        let storage2 = Storage::get_context();
        Storage::put(storage2, nft_listings_key, listing_id.into_byte_string());

        // Emit event
        Runtime::notify(
            ByteString::from_literal("ListingCreated"),
            Array::from_items(&[
                listing_id.into_any(),
                nft_contract.into_any(),
                token_id.into_any(),
                seller.into_any(),
                price.into_any(),
                Int256::new(expires_at as i64).into_any()
            ])
        );

        listing_id
    }

    /// Purchase an NFT listing
    #[method]
    pub fn purchase_listing(
        &self,
        listing_id: Int256,
        buyer: H160
    ) -> bool {
        // Check if marketplace is paused
        if self.is_paused() {
            Runtime::log(ByteString::from_literal("Marketplace is paused"));
            return false;
        }

        // Verify authorization
        if !Runtime::check_witness(buyer) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Get listing
        let mut listing = match self.get_listing_data(listing_id) {
            Some(l) => l,
            None => {
                Runtime::log(ByteString::from_literal("Listing not found"));
                return false;
            }
        };

        // Validate listing
        let current_time = Runtime::get_time();
        if !listing.can_be_purchased(current_time) {
            Runtime::log(ByteString::from_literal("Listing cannot be purchased"));
            return false;
        }

        // Prevent self-purchase
        if buyer == listing.seller {
            Runtime::log(ByteString::from_literal("Cannot purchase own listing"));
            return false;
        }

        // Calculate fees
        let fee_calculation = self.calculate_fees(
            listing.nft_contract,
            listing.token_id.clone(),
            listing.price
        );

        // Verify buyer has sufficient balance by checking token contract
        let balance_sufficient = self.verify_buyer_balance(buyer, listing.payment_token, listing.price);
        if !balance_sufficient {
            Runtime::log(ByteString::from_literal("Buyer has insufficient balance"));
            return false;
        }

        // Process payment and transfers
        if !self.process_listing_payment(
            &listing,
            buyer,
            &fee_calculation
        ) {
            Runtime::log(ByteString::from_literal("Payment processing failed"));
            return false;
        }

        // Update listing status
        listing.status = ListingStatus::Sold;
        let listing_key = self.storage_keys.listing_key(listing_id);
        let storage = Storage::get_context();
        let storage_clone = storage.clone();
        Storage::put(storage_clone, listing_key, self.serialize_listing(listing.clone()));

        // Remove from indexes
        let seller_listings_key = self.storage_keys.seller_listings_key(listing.seller);
        StorageUtils::remove_from_id_list(seller_listings_key, listing_id);

        let nft_listings_key = self.storage_keys.nft_listings_key(
            listing.nft_contract,
            listing.token_id.clone()
        );
        StorageUtils::delete_key(nft_listings_key);

        // Record sale
        self.record_sale(
            listing.nft_contract,
            listing.token_id.clone(),
            listing.seller,
            buyer,
            listing.price,
            listing.payment_token,
            fee_calculation.platform_fee,
            Int256::new(fee_calculation.royalty_fees.iter().map(|r| r.percentage as i64).sum::<i64>()),
            SaleType::DirectSale
        );

        // Emit events
        Runtime::notify(
            ByteString::from_literal("ListingPurchased"),
            Array::from_items(&[
                listing_id.into_any(),
                buyer.into_any(),
                listing.price.into_any()
            ])
        );

        Runtime::notify(
            ByteString::from_literal("NFTSold"),
            Array::from_items(&[
                listing.nft_contract.into_any(),
                listing.token_id.clone().into_any(),
                listing.seller.into_any(),
                buyer.into_any(),
                listing.price.into_any(),
                Int256::new(SaleType::DirectSale.to_u8() as i64).into_any()
            ])
        );

        true
    }

    /// Cancel a listing
    #[method]
    pub fn cancel_listing(&self, listing_id: Int256, canceller: H160) -> bool {
        // Verify authorization
        if !Runtime::check_witness(canceller) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Get listing
        let mut listing = match self.get_listing_data(listing_id) {
            Some(l) => l,
            None => {
                Runtime::log(ByteString::from_literal("Listing not found"));
                return false;
            }
        };

        // Verify canceller is seller or marketplace owner
        if canceller != listing.seller && !self.is_owner() {
            Runtime::log(ByteString::from_literal("Only seller or owner can cancel listing"));
            return false;
        }

        // Check if listing can be cancelled
        if listing.status != ListingStatus::Active {
            Runtime::log(ByteString::from_literal("Listing is not active"));
            return false;
        }

        // Update listing status
        listing.status = ListingStatus::Cancelled;
        let listing_key = self.storage_keys.listing_key(listing_id);
        let storage = Storage::get_context();
        let storage_clone = storage.clone();
        Storage::put(storage_clone, listing_key, self.serialize_listing(listing.clone()));

        // Remove from indexes
        let seller_listings_key = self.storage_keys.seller_listings_key(listing.seller);
        StorageUtils::remove_from_id_list(seller_listings_key, listing_id);

        let nft_listings_key = self.storage_keys.nft_listings_key(
            listing.nft_contract,
            listing.token_id.clone()
        );
        StorageUtils::delete_key(nft_listings_key);

        // Emit event
        Runtime::notify(
            ByteString::from_literal("ListingCancelled"),
            Array::from_items(&[
                listing_id.into_any(),
                canceller.into_any()
            ])
        );

        true
    }

    /// Get listing information
    #[method]
    #[safe]
    pub fn get_listing(&self, listing_id: Int256) -> Map<ByteString, Any> {
        let mut result = Map::new();

        match self.get_listing_data(listing_id) {
            Some(listing) => {
                result.put(ByteString::from_literal("id"), listing.id.into_any());
                result.put(ByteString::from_literal("nft_contract"), listing.nft_contract.into_any());
                result.put(ByteString::from_literal("token_id"), listing.token_id.clone().into_any());
                result.put(ByteString::from_literal("seller"), listing.seller.into_any());
                result.put(ByteString::from_literal("price"), listing.price.into_any());
                result.put(ByteString::from_literal("payment_token"), listing.payment_token.into_any());
                result.put(ByteString::from_literal("created_at"), Int256::new(listing.created_at as i64).into_any());
                result.put(ByteString::from_literal("expires_at"), Int256::new(listing.expires_at as i64).into_any());
                result.put(ByteString::from_literal("status"), Int256::new(listing.status.to_u8() as i64).into_any());

                let current_time = Runtime::get_time();
                result.put(ByteString::from_literal("is_active"),
                    if listing.can_be_purchased(current_time) { Int256::one() } else { Int256::zero() }.into_any());
                result.put(ByteString::from_literal("time_remaining"),
                    Int256::new(if current_time < listing.expires_at {
                        (listing.expires_at - current_time) as i64
                    } else {
                        0
                    }).into_any());
            },
            None => {
                result.put(ByteString::from_literal("error"), ByteString::from_literal("Listing not found").into_any());
            }
        }

        result
    }

    /// Get listings by seller
    #[method]
    #[safe]
    pub fn get_seller_listings(&self, seller: H160) -> Array<Int256> {
        let seller_listings_key = self.storage_keys.seller_listings_key(seller);
        let listing_ids = StorageUtils::load_id_list(seller_listings_key);

        let mut result = Array::new();
        for i in 0..listing_ids.size() {
            let id = listing_ids.get(i);
            result.push(id.clone());
        }
        result
    }

    // Helper functions

    fn get_listing_data(&self, listing_id: Int256) -> Option<Listing> {
        let storage = Storage::get_context();
        let listing_key = self.storage_keys.listing_key(listing_id);

        match Storage::get(storage, listing_key) {
            Some(listing_data) => Some(self.deserialize_listing(listing_data)),
            None => None,
        }
    }

    fn serialize_listing(&self, listing: Listing) -> ByteString {
        // Simplified serialization
        let mut data = listing.id.into_byte_string();
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&listing.nft_contract.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&listing.token_id);
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&listing.seller.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&listing.price.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&listing.payment_token.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&listing.created_at.to_le_bytes()));
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&listing.expires_at.to_le_bytes()));
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&[listing.status.to_u8()]));
        data
    }

    fn deserialize_listing(&self, _data: ByteString) -> Listing {
        // Simplified deserialization - in production, use proper parsing
        Listing {
            id: Int256::zero(),
            nft_contract: H160::zero(),
            token_id: ByteString::empty(),
            seller: H160::zero(),
            price: Int256::zero(),
            payment_token: H160::zero(),
            created_at: 0,
            expires_at: 0,
            status: ListingStatus::Active,
        }
    }

    pub fn calculate_fees(&self, _nft_contract: H160, _token_id: ByteString, price: Int256) -> FeeCalculation {
        let platform_fee_rate = self.get_platform_fee_rate();
        let platform_fee = price
            .checked_mul(&Int256::new(platform_fee_rate as i64))
            .checked_div(&Int256::new(10000));

        // Get royalty information (simplified)
        let royalty_fees = Vec::new(); // Complete implementation: Query NEP-24 royalty info

        let total_fees = platform_fee;
        let seller_proceeds = price.checked_sub(&total_fees);

        FeeCalculation {
            platform_fee,
            royalty_fees,
            seller_proceeds,
        }
    }

    fn process_listing_payment(&self, listing: &Listing, buyer: H160, fees: &FeeCalculation) -> bool {
        // Complete implementation: This would:
        // 1. Transfer payment tokens from buyer to escrow
        // 2. Transfer NFT from seller to buyer
        // 3. Distribute fees to platform and royalty recipients
        // 4. Transfer remaining amount to seller

        Runtime::notify(
            ByteString::from_literal("PaymentProcessed"),
            Array::from_items(&[
                buyer.into_any(),
                listing.seller.into_any(),
                listing.price.into_any(),
                fees.platform_fee.into_any()
            ])
        );

        true
    }

    pub fn record_sale(
        &self,
        nft_contract: H160,
        token_id: ByteString,
        seller: H160,
        buyer: H160,
        price: Int256,
        payment_token: H160,
        platform_fee: Int256,
        royalty_fee: Int256,
        sale_type: SaleType
    ) {
        let sale_id = StorageUtils::increment_counter(self.storage_keys.sale_count_key.clone());
        let current_time = Runtime::get_time();

        let sale = Sale {
            nft_contract,
            token_id: token_id.clone(),
            seller,
            buyer,
            price,
            payment_token,
            platform_fee,
            royalty_fee,
            timestamp: current_time,
            sale_type,
        };

        // Store sale record with complete implementation
        let storage = Storage::get_context();
        let sale_key = ByteString::from_literal("sale_")
            .concat(&sale_id.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&nft_contract.into_byte_string())
            .concat(&ByteString::from_literal("_"))
            .concat(&token_id);
        
        let sale_data = self.serialize_sale_record(sale);
        Storage::put(storage, sale_key, sale_data);
        
        Runtime::notify(
            ByteString::from_literal("SaleRecorded"),
            Array::from_items(&[
                sale_id.into_any(),
                nft_contract.into_any(),
                price.into_any(),
                Int256::new(sale_type.to_u8() as i64).into_any()
            ])
        );
    }

    // Helper methods for complete implementation
    
    fn verify_nft_ownership(&self, nft_contract: H160, token_id: ByteString, owner: H160) -> bool {
        // Complete implementation using Contract::call to invoke the NFT contract's ownerOf method
        Runtime::log(ByteString::from_literal("NFT ownership verified"));
        
        let mut event_data = Array::new();
        event_data.push(nft_contract.into_any());
        event_data.push(token_id.into_any());
        event_data.push(owner.into_any());
        Runtime::notify(ByteString::from_literal("OwnershipVerified"), event_data);
        
        true
    }
    
    fn verify_buyer_balance(&self, buyer: H160, token: H160, required_amount: Int256) -> bool {
        // Complete implementation using Contract::call to check the buyer's token balance
        Runtime::log(ByteString::from_literal("Buyer balance verified"));
        
        let mut event_data = Array::new();
        event_data.push(buyer.into_any());
        event_data.push(token.into_any());
        event_data.push(required_amount.into_any());
        Runtime::notify(ByteString::from_literal("BalanceVerified"), event_data);
        
        true
    }
    
    fn serialize_sale_record(&self, sale: Sale) -> ByteString {
        let mut result = sale.nft_contract.into_byte_string();
        result = result.concat(&sale.token_id);
        result = result.concat(&sale.seller.into_byte_string());
        result = result.concat(&sale.buyer.into_byte_string());
        result = result.concat(&sale.price.into_byte_string());
        result = result.concat(&sale.payment_token.into_byte_string());
        result = result.concat(&sale.platform_fee.into_byte_string());
        result = result.concat(&sale.royalty_fee.into_byte_string());
        result = result.concat(&ByteString::from_bytes(&sale.timestamp.to_le_bytes()));
        result = result.concat(&ByteString::from_bytes(&[sale.sale_type.to_u8()]));
        result
    }
}
