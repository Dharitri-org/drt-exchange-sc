#![no_std]

dharitri_wasm::imports!();

use common_structs::{PaymentAttributesPair, PaymentsVec};
use dharitri_wasm::dharitri_codec::TopEncode;
use fixed_supply_token::FixedSupplyToken;
use mergeable::Mergeable;

static ERR_EMPTY_PAYMENTS: &[u8] = b"No payments";

#[dharitri_wasm::module]
pub trait UtilsModule {
    fn dest_from_optional(&self, opt_destination: OptionalValue<ManagedAddress>) -> ManagedAddress {
        match opt_destination {
            OptionalValue::Some(dest) => dest,
            OptionalValue::None => self.blockchain().get_caller(),
        }
    }

    fn to_dct_payment(
        &self,
        moax_or_dct_payment: MoaxOrDctTokenPayment<Self::Api>,
    ) -> DctTokenPayment {
        DctTokenPayment::new(
            moax_or_dct_payment.token_identifier.unwrap_dct(),
            moax_or_dct_payment.token_nonce,
            moax_or_dct_payment.amount,
        )
    }

    fn burn_multi_dct(&self, payments: &PaymentsVec<Self::Api>) {
        for payment in payments {
            self.send().dct_local_burn(
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );
        }
    }

    fn get_non_empty_payments(&self) -> PaymentsVec<Self::Api> {
        let payments = self.call_value().all_dct_transfers();
        require!(!payments.is_empty(), ERR_EMPTY_PAYMENTS);

        payments
    }

    fn pop_first_payment(
        &self,
        payments: &mut PaymentsVec<Self::Api>,
    ) -> DctTokenPayment<Self::Api> {
        require!(!payments.is_empty(), ERR_EMPTY_PAYMENTS);

        let first_payment = payments.get(0);
        payments.remove(0);

        first_payment
    }

    fn get_token_attributes<T: TopDecode>(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
    ) -> T {
        let own_sc_address = self.blockchain().get_sc_address();
        let token_data =
            self.blockchain()
                .get_dct_token_data(&own_sc_address, token_id, token_nonce);

        token_data.decode_attributes()
    }

    fn get_attributes_as_part_of_fixed_supply<T: FixedSupplyToken<Self::Api> + TopDecode>(
        &self,
        payment: &DctTokenPayment,
        mapper: &NonFungibleTokenMapper<Self::Api>,
    ) -> T {
        let attr: T = mapper.get_token_attributes(payment.token_nonce);
        attr.into_part(&payment.amount)
    }

    fn merge_from_payments_and_burn<
        T: FixedSupplyToken<Self::Api> + Mergeable<Self::Api> + TopDecode,
    >(
        &self,
        mut payments: PaymentsVec<Self::Api>,
        mapper: &NonFungibleTokenMapper<Self::Api>,
    ) -> T {
        let first_payment = self.pop_first_payment(&mut payments);
        let base_attributes: T =
            self.get_attributes_as_part_of_fixed_supply(&first_payment, mapper);
        mapper.nft_burn(first_payment.token_nonce, &first_payment.amount);

        let output_attributes =
            self.merge_attributes_from_payments(base_attributes, &payments, mapper);
        self.burn_multi_dct(&payments);

        output_attributes
    }

    fn merge_attributes_from_payments<
        T: FixedSupplyToken<Self::Api> + Mergeable<Self::Api> + TopDecode,
    >(
        &self,
        mut base_attributes: T,
        payments: &PaymentsVec<Self::Api>,
        mapper: &NonFungibleTokenMapper<Self::Api>,
    ) -> T {
        for payment in payments {
            let attributes: T = self.get_attributes_as_part_of_fixed_supply(&payment, mapper);
            base_attributes.merge_with(attributes);
        }

        base_attributes
    }

    fn merge_and_create_token<
        T: FixedSupplyToken<Self::Api>
            + Mergeable<Self::Api>
            + Clone
            + TopEncode
            + TopDecode
            + NestedEncode
            + NestedDecode,
    >(
        &self,
        base_attributes: T,
        payments: &PaymentsVec<Self::Api>,
        mapper: &NonFungibleTokenMapper<Self::Api>,
    ) -> PaymentAttributesPair<Self::Api, T> {
        let output_attributes =
            self.merge_attributes_from_payments(base_attributes, payments, mapper);
        let new_token_amount = output_attributes.get_total_supply();
        let new_token_payment = mapper.nft_create(new_token_amount, &output_attributes);

        PaymentAttributesPair {
            payment: new_token_payment,
            attributes: output_attributes,
        }
    }

    fn require_valid_token_id(&self, token_id: &TokenIdentifier) {
        require!(token_id.is_valid_dct_identifier(), "Invalid token ID");
    }

    fn require_sc_address(&self, address: &ManagedAddress) {
        require!(
            !address.is_zero() && self.blockchain().is_smart_contract(address),
            "Invalid SC address"
        );
    }
}
