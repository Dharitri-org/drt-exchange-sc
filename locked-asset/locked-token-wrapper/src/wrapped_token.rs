dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

use common_structs::Nonce;

pub static WRAPPED_TOKEN_NAME: &[u8] = b"WrappedLKMEX";

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct WrappedTokenAttributes {
    pub locked_token_nonce: Nonce,
}

#[dharitri_wasm::module]
pub trait WrappedTokenModule:
    dharitri_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + simple_lock::token_attributes::TokenAttributesModule
{
    #[only_owner]
    #[payable("MOAX")]
    #[endpoint(issueWrappedToken)]
    fn issue_wrapped_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        let payment_amount = self.call_value().moax_value();

        self.wrapped_token().issue_and_set_all_roles(
            DctTokenType::Meta,
            payment_amount,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    /// Sets the transfer role for the given address. Defaults to own address.
    #[only_owner]
    #[endpoint(setTransferRoleWrappedToken)]
    fn set_transfer_role(&self, opt_address: OptionalValue<ManagedAddress>) {
        let address = match opt_address {
            OptionalValue::Some(addr) => addr,
            OptionalValue::None => self.blockchain().get_sc_address(),
        };

        self.wrapped_token().set_local_roles_for_address(
            &address,
            &[DctLocalRole::Transfer],
            None,
        );
    }

    fn wrap_locked_token_and_send(
        &self,
        caller: &ManagedAddress,
        token: DctTokenPayment,
    ) -> DctTokenPayment {
        self.locked_token()
            .require_same_token(&token.token_identifier);

        let wrapped_token_mapper = self.wrapped_token();
        let wrapped_token_attributes = WrappedTokenAttributes {
            locked_token_nonce: token.token_nonce,
        };
        let wrapped_token_nonce = self.get_or_create_nonce_for_attributes(
            &wrapped_token_mapper,
            &ManagedBuffer::new_from_bytes(WRAPPED_TOKEN_NAME),
            &wrapped_token_attributes,
        );

        wrapped_token_mapper.nft_add_quantity_and_send(caller, wrapped_token_nonce, token.amount)
    }

    fn unwrap_locked_token(&self, token: DctTokenPayment) -> DctTokenPayment {
        let wrapped_token_mapper = self.wrapped_token();
        wrapped_token_mapper.require_same_token(&token.token_identifier);

        let wrapped_token_attributes: WrappedTokenAttributes =
            wrapped_token_mapper.get_token_attributes(token.token_nonce);

        self.send()
            .dct_local_burn(&token.token_identifier, token.token_nonce, &token.amount);

        let locked_token_id = self.locked_token().get_token_id();
        DctTokenPayment::new(
            locked_token_id,
            wrapped_token_attributes.locked_token_nonce,
            token.amount,
        )
    }

    #[view(getLockedTokenId)]
    #[storage_mapper("lockedTokenId")]
    fn locked_token(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[view(getWrappedTokenId)]
    #[storage_mapper("wrappedTokenId")]
    fn wrapped_token(&self) -> NonFungibleTokenMapper<Self::Api>;
}
