dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

use dharitri_codec::TopEncode;

const INITIAL_SFT_AMOUNT: u32 = 1;

#[dharitri_wasm::module]
pub trait TokenAttributesModule {
    fn get_or_create_nonce_for_attributes<T: TopEncode + NestedEncode>(
        &self,
        nft_mapper: &NonFungibleTokenMapper<Self::Api>,
        token_name: &ManagedBuffer,
        attributes: &T,
    ) -> u64 {
        let nft_token_id = nft_mapper.get_token_id();
        let mut encoded_attributes = ManagedBuffer::new();
        attributes
            .dep_encode(&mut encoded_attributes)
            .unwrap_or_else(|err| sc_panic!(err.message_str()));

        let attributes_to_nonce_mapper =
            self.attributes_to_nonce_mapping(&nft_token_id, &encoded_attributes);
        let existing_nonce = attributes_to_nonce_mapper.get();
        if existing_nonce != 0 {
            return existing_nonce;
        }

        let new_nonce = self.send().dct_nft_create(
            &nft_token_id,
            &INITIAL_SFT_AMOUNT.into(),
            token_name,
            &BigUint::zero(),
            &ManagedBuffer::new(),
            attributes,
            &ManagedVec::new(),
        );
        attributes_to_nonce_mapper.set(new_nonce);

        new_nonce
    }

    #[storage_mapper("attributesToNonceMapping")]
    fn attributes_to_nonce_mapping(
        &self,
        token_id: &TokenIdentifier,
        attributes: &ManagedBuffer,
    ) -> SingleValueMapper<u64>;
}
