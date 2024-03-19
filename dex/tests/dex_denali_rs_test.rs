use dharitri_wasm_debug::BlockchainMock;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("dex");

    blockchain.register_contract_builder("file:router/output/router.wasm", router::ContractBuilder);
    blockchain.register_contract_builder("file:pair/output/pair.wasm", pair::ContractBuilder);
    blockchain.register_contract_builder("file:farm/output/farm.wasm", farm::ContractBuilder);

    blockchain
}

#[test]
fn add_liquidity_rs() {
    dharitri_wasm_debug::denali_rs("denali/add_liquidity.scen.json", world());
}

#[test]
fn calculate_rewards_for_given_position_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/calculate_rewards_for_given_position.scen.json",
        world(),
    );
}

#[test]
fn calculate_rewards_for_given_position_after_compound_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/calculate_rewards_for_given_position_after_compound.scen.json",
        world(),
    );
}

#[test]
fn check_fee_disabled_after_swap_rs() {
    dharitri_wasm_debug::denali_rs("denali/check_fee_disabled_after_swap.scen.json", world());
}

#[test]
fn check_fee_enabled_after_swap_rs() {
    dharitri_wasm_debug::denali_rs("denali/check_fee_enabled_after_swap.scen.json", world());
}

#[test]
fn claim_rewards_rs() {
    dharitri_wasm_debug::denali_rs("denali/claim_rewards.scen.json", world());
}

#[test]
fn complete_setup_rs() {
    dharitri_wasm_debug::denali_rs("denali/complete_setup.scen.json", world());
}

#[test]
fn compound_rewards_rs() {
    dharitri_wasm_debug::denali_rs("denali/compound_rewards.scen.json", world());
}

#[test]
fn create_pair_twice_rs() {
    dharitri_wasm_debug::denali_rs("denali/create_pair_twice.scen.json", world());
}

#[test]
fn enter_farm_rs() {
    dharitri_wasm_debug::denali_rs("denali/enter_farm.scen.json", world());
}

#[test]
fn enter_farm_with_merge_tokens_rs() {
    dharitri_wasm_debug::denali_rs("denali/enter_farm_with_merge_tokens.scen.json", world());
}

#[test]
fn enter_mex_farm_rs() {
    dharitri_wasm_debug::denali_rs("denali/enter_mex_farm.scen.json", world());
}

#[test]
fn exit_farm_rs() {
    dharitri_wasm_debug::denali_rs("denali/exit_farm.scen.json", world());
}

#[test]
fn exit_farm_too_soon_rs() {
    dharitri_wasm_debug::denali_rs("denali/exit_farm_too_soon.scen.json", world());
}

#[test]
fn exit_mex_farm_rs() {
    dharitri_wasm_debug::denali_rs("denali/exit_mex_farm.scen.json", world());
}

#[test]
fn farm_reward_distr_scen_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/farm_reward_distr_scen_1.scen.json", world());
}

#[test]
fn farm_reward_distr_scen_2_rs() {
    dharitri_wasm_debug::denali_rs("denali/farm_reward_distr_scen_2.scen.json", world());
}

#[test]
fn farm_reward_distr_scen_3_rs() {
    dharitri_wasm_debug::denali_rs("denali/farm_reward_distr_scen_3.scen.json", world());
}

#[test]
fn farm_reward_distr_scen_4_rs() {
    dharitri_wasm_debug::denali_rs("denali/farm_reward_distr_scen_4.scen.json", world());
}

#[test]
fn farm_with_moax_token_rs() {
    dharitri_wasm_debug::denali_rs("denali/farm_with_moax_token.scen.json", world());
}

#[test]
fn farm_wrong_lp_token_rs() {
    dharitri_wasm_debug::denali_rs("denali/farm_wrong_lp_token.scen.json", world());
}

#[test]
fn get_amounts_rs() {
    dharitri_wasm_debug::denali_rs("denali/get_amounts.scen.json", world());
}

#[test]
fn get_amounts_no_liquidity_rs() {
    dharitri_wasm_debug::denali_rs("denali/get_amounts_no_liquidity.scen.json", world());
}

#[test]
fn get_pair_non_existent_rs() {
    dharitri_wasm_debug::denali_rs("denali/get_pair_non_existent.scen.json", world());
}

#[test]
fn get_pair_views_rs() {
    dharitri_wasm_debug::denali_rs("denali/get_pair_views.scen.json", world());
}

#[test]
fn merge_tokens_rs() {
    dharitri_wasm_debug::denali_rs("denali/merge_tokens.scen.json", world());
}

#[test]
fn owner_pause_farm_rs() {
    dharitri_wasm_debug::denali_rs("denali/owner_pause_farm.scen.json", world());
}

#[test]
fn owner_resume_farm_rs() {
    dharitri_wasm_debug::denali_rs("denali/owner_resume_farm.scen.json", world());
}

#[test]
fn remove_liquidity_rs() {
    dharitri_wasm_debug::denali_rs("denali/remove_liquidity.scen.json", world());
}

#[test]
fn remove_liquidity_and_buyback_and_burn_token_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/remove_liquidity_and_buyback_and_burn_token.scen.json",
        world(),
    );
}

#[test]
fn remove_liquidity_twice_rs() {
    dharitri_wasm_debug::denali_rs("denali/remove_liquidity_twice.scen.json", world());
}

#[test]
fn remove_pair_rs() {
    dharitri_wasm_debug::denali_rs("denali/remove_pair.scen.json", world());
}

#[test]
fn router_pause_self_rs() {
    dharitri_wasm_debug::denali_rs("denali/router_pause_self.scen.json", world());
}

#[test]
fn router_resume_self_rs() {
    dharitri_wasm_debug::denali_rs("denali/router_resume_self.scen.json", world());
}

#[test]
fn swap_fixed_input_rs() {
    dharitri_wasm_debug::denali_rs("denali/swap_fixed_input.scen.json", world());
}

#[test]
fn swap_fixed_input_after_removed_liquidity_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/swap_fixed_input_after_removed_liquidity.scen.json",
        world(),
    );
}

#[test]
fn swap_fixed_output_rs() {
    dharitri_wasm_debug::denali_rs("denali/swap_fixed_output.scen.json", world());
}

#[test]
fn swap_same_token_rs() {
    dharitri_wasm_debug::denali_rs("denali/swap_same_token.scen.json", world());
}

#[test]
fn swap_wrong_token_rs() {
    dharitri_wasm_debug::denali_rs("denali/swap_wrong_token.scen.json", world());
}

#[test]
fn upgrade_contract_rs() {
    dharitri_wasm_debug::denali_rs("denali/upgrade_contract.scen.json", world());
}
