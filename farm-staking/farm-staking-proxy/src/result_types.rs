dharitri_wasm::imports!();

pub type PaymentsVec<M> = ManagedVec<M, DctTokenPayment<M>>;

// lp farm

pub struct LpFarmClaimRewardsResult<M: ManagedTypeApi> {
    pub new_lp_farm_tokens: DctTokenPayment<M>,
    pub lp_farm_rewards: DctTokenPayment<M>,
}

pub struct LpFarmExitResult<M: ManagedTypeApi> {
    pub lp_tokens: DctTokenPayment<M>,
    pub lp_farm_rewards: DctTokenPayment<M>,
}

// staking farm

pub struct StakingFarmEnterResult<M: ManagedTypeApi> {
    pub received_staking_farm_token: DctTokenPayment<M>,
}

pub struct StakingFarmClaimRewardsResult<M: ManagedTypeApi> {
    pub new_staking_farm_tokens: DctTokenPayment<M>,
    pub staking_farm_rewards: DctTokenPayment<M>,
}

pub struct StakingFarmExitResult<M: ManagedTypeApi> {
    pub unbond_staking_farm_token: DctTokenPayment<M>,
    pub staking_rewards: DctTokenPayment<M>,
}

// pair

pub struct PairRemoveLiquidityResult<M: ManagedTypeApi> {
    pub staking_token_payment: DctTokenPayment<M>,
    pub other_token_payment: DctTokenPayment<M>,
}
