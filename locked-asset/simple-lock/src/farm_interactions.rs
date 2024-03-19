use common_structs::{RawResultWrapper, RawResultsType};

dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

type EnterFarmResultType<BigUint> =
    MultiValue2<DctTokenPayment<BigUint>, DctTokenPayment<BigUint>>;
type ExitFarmResultType<BigUint> =
    MultiValue3<DctTokenPayment<BigUint>, DctTokenPayment<BigUint>, DctTokenPayment<BigUint>>;
type ClaimRewardsResultType<BigUint> =
    MultiValue2<DctTokenPayment<BigUint>, DctTokenPayment<BigUint>>;

const ENTER_FARM_RESULTS_LEN: usize = 2;
const EXIT_FARM_RESULTS_LEN: usize = 3;
const CLAIM_REWARDS_RESULTS_LEN: usize = 2;

pub struct EnterFarmResultWrapper<M: ManagedTypeApi> {
    pub farm_tokens: DctTokenPayment<M>,
    pub reward_tokens: DctTokenPayment<M>,
}

pub struct ExitFarmResultWrapper<M: ManagedTypeApi> {
    pub initial_farming_tokens: DctTokenPayment<M>,
    pub reward_tokens: DctTokenPayment<M>,
    pub remaining_farm_tokens: DctTokenPayment<M>,
}

pub struct FarmClaimRewardsResultWrapper<M: ManagedTypeApi> {
    pub new_farm_tokens: DctTokenPayment<M>,
    pub reward_tokens: DctTokenPayment<M>,
}

pub struct FarmCompoundRewardsResultWrapper<M: ManagedTypeApi> {
    pub new_farm_tokens: DctTokenPayment<M>,
}

mod farm_proxy {
    dharitri_wasm::imports!();
    use super::{ClaimRewardsResultType, EnterFarmResultType, ExitFarmResultType};

    #[dharitri_wasm::proxy]
    pub trait FarmProxy {
        #[payable("*")]
        #[endpoint(enterFarm)]
        fn enter_farm(&self) -> EnterFarmResultType<Self::Api>;

        #[payable("*")]
        #[endpoint(exitFarm)]
        fn exit_farm(&self, exit_amount: BigUint) -> ExitFarmResultType<Self::Api>;

        #[payable("*")]
        #[endpoint(claimRewards)]
        fn claim_rewards(&self) -> ClaimRewardsResultType<Self::Api>;
    }
}

#[dharitri_wasm::module]
pub trait FarmInteractionsModule {
    fn call_farm_enter(
        &self,
        farm_address: ManagedAddress,
        farming_token: TokenIdentifier,
        farming_token_amount: BigUint,
        additional_farm_tokens: ManagedVec<DctTokenPayment<Self::Api>>,
    ) -> EnterFarmResultWrapper<Self::Api> {
        let mut contract_call = self
            .farm_proxy(farm_address)
            .enter_farm()
            .add_dct_token_transfer(farming_token, 0, farming_token_amount);
        for farm_token in &additional_farm_tokens {
            contract_call = contract_call.add_dct_token_transfer(
                farm_token.token_identifier,
                farm_token.token_nonce,
                farm_token.amount,
            );
        }

        let raw_results: RawResultsType<Self::Api> = contract_call.execute_on_dest_context();
        let mut results_wrapper = RawResultWrapper::new(raw_results);
        results_wrapper.trim_results_front(ENTER_FARM_RESULTS_LEN);

        let new_farm_tokens = results_wrapper.decode_next_result();
        let reward_tokens = results_wrapper.decode_next_result();

        EnterFarmResultWrapper {
            farm_tokens: new_farm_tokens,
            reward_tokens,
        }
    }

    fn call_farm_exit(
        &self,
        farm_address: ManagedAddress,
        farm_token: TokenIdentifier,
        farm_token_nonce: u64,
        farm_token_amount: BigUint,
        exit_amount: BigUint,
    ) -> ExitFarmResultWrapper<Self::Api> {
        let raw_results: RawResultsType<Self::Api> = self
            .farm_proxy(farm_address)
            .exit_farm(exit_amount)
            .add_dct_token_transfer(farm_token, farm_token_nonce, farm_token_amount)
            .execute_on_dest_context();

        let mut results_wrapper = RawResultWrapper::new(raw_results);
        results_wrapper.trim_results_front(EXIT_FARM_RESULTS_LEN);

        let initial_farming_tokens = results_wrapper.decode_next_result();
        let reward_tokens = results_wrapper.decode_next_result();
        let remaining_farm_tokens = results_wrapper.decode_next_result();

        ExitFarmResultWrapper {
            initial_farming_tokens,
            reward_tokens,
            remaining_farm_tokens,
        }
    }

    fn call_farm_claim_rewards(
        &self,
        farm_address: ManagedAddress,
        farm_token: TokenIdentifier,
        farm_token_nonce: u64,
        farm_token_amount: BigUint,
    ) -> FarmClaimRewardsResultWrapper<Self::Api> {
        let raw_results: RawResultsType<Self::Api> = self
            .farm_proxy(farm_address)
            .claim_rewards()
            .add_dct_token_transfer(farm_token, farm_token_nonce, farm_token_amount)
            .execute_on_dest_context();

        let mut results_wrapper = RawResultWrapper::new(raw_results);
        results_wrapper.trim_results_front(CLAIM_REWARDS_RESULTS_LEN);

        let new_farm_tokens = results_wrapper.decode_next_result();
        let reward_tokens = results_wrapper.decode_next_result();

        FarmClaimRewardsResultWrapper {
            new_farm_tokens,
            reward_tokens,
        }
    }

    #[proxy]
    fn farm_proxy(&self, sc_address: ManagedAddress) -> farm_proxy::Proxy<Self::Api>;
}
