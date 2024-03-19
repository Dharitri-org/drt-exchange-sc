dharitri_wasm::imports!();

use contexts::storage_cache::StorageCache;

use crate::token_attributes::UnbondSftAttributes;

#[dharitri_wasm::module]
pub trait UnbondFarmModule:
    crate::custom_rewards::CustomRewardsModule
    + rewards::RewardsModule
    + config::ConfigModule
    + events::EventsModule
    + token_send::TokenSendModule
    + farm_token::FarmTokenModule
    + sc_whitelist_module::SCWhitelistModule
    + pausable::PausableModule
    + permissions_module::PermissionsModule
    + dharitri_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + farm_base_impl::base_farm_init::BaseFarmInitModule
    + farm_base_impl::base_farm_validation::BaseFarmValidationModule
    + utils::UtilsModule
{
    #[payable("*")]
    #[endpoint(unbondFarm)]
    fn unbond_farm(&self) -> DctTokenPayment {
        let storage_cache = StorageCache::new(self);
        self.validate_contract_state(storage_cache.contract_state, &storage_cache.farm_token_id);

        let farm_token_mapper = self.farm_token();
        let payment = self.call_value().single_dct();
        farm_token_mapper.require_same_token(&payment.token_identifier);

        let attributes: UnbondSftAttributes =
            farm_token_mapper.get_token_attributes(payment.token_nonce);

        let current_epoch = self.blockchain().get_block_epoch();
        require!(
            current_epoch >= attributes.unlock_epoch,
            "Unbond period not over"
        );

        farm_token_mapper.nft_burn(payment.token_nonce, &payment.amount);

        let caller = self.blockchain().get_caller();
        let farming_tokens =
            DctTokenPayment::new(storage_cache.farming_token_id.clone(), 0, payment.amount);
        self.send_payment_non_zero(&caller, &farming_tokens);

        farming_tokens
    }
}
