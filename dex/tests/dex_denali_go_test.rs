#[test]
fn add_liquidity_go() {
    dharitri_wasm_debug::denali_go("denali/add_liquidity.scen.json");
}

#[test]
fn calculate_rewards_for_given_position_go() {
    dharitri_wasm_debug::denali_go("denali/calculate_rewards_for_given_position.scen.json");
}

#[test]
fn calculate_rewards_for_given_position_after_compound_go() {
    dharitri_wasm_debug::denali_go(
        "denali/calculate_rewards_for_given_position_after_compound.scen.json",
    );
}

#[test]
fn check_fee_disabled_after_swap_go() {
    dharitri_wasm_debug::denali_go("denali/check_fee_disabled_after_swap.scen.json");
}

#[test]
fn check_fee_enabled_after_swap_go() {
    dharitri_wasm_debug::denali_go("denali/check_fee_enabled_after_swap.scen.json");
}

#[test]
fn claim_rewards_go() {
    dharitri_wasm_debug::denali_go("denali/claim_rewards.scen.json");
}

#[test]
fn complete_setup_go() {
    dharitri_wasm_debug::denali_go("denali/complete_setup.scen.json");
}

#[test]
fn compound_rewards_go() {
    dharitri_wasm_debug::denali_go("denali/compound_rewards.scen.json");
}

#[test]
fn create_pair_twice_go() {
    dharitri_wasm_debug::denali_go("denali/create_pair_twice.scen.json");
}

#[test]
fn enter_farm_go() {
    dharitri_wasm_debug::denali_go("denali/enter_farm.scen.json");
}

#[test]
fn enter_farm_with_merge_tokens_go() {
    dharitri_wasm_debug::denali_go("denali/enter_farm_with_merge_tokens.scen.json");
}

#[test]
fn enter_mex_farm_go() {
    dharitri_wasm_debug::denali_go("denali/enter_mex_farm.scen.json");
}

#[test]
fn exit_farm_go() {
    dharitri_wasm_debug::denali_go("denali/exit_farm.scen.json");
}

#[test]
fn exit_farm_too_soon_go() {
    dharitri_wasm_debug::denali_go("denali/exit_farm_too_soon.scen.json");
}

#[test]
fn exit_mex_farm_go() {
    dharitri_wasm_debug::denali_go("denali/exit_mex_farm.scen.json");
}

#[test]
fn farm_reward_distr_scen_1_go() {
    dharitri_wasm_debug::denali_go("denali/farm_reward_distr_scen_1.scen.json");
}

#[test]
fn farm_reward_distr_scen_2_go() {
    dharitri_wasm_debug::denali_go("denali/farm_reward_distr_scen_2.scen.json");
}

#[test]
fn farm_reward_distr_scen_3_go() {
    dharitri_wasm_debug::denali_go("denali/farm_reward_distr_scen_3.scen.json");
}

#[test]
fn farm_reward_distr_scen_4_go() {
    dharitri_wasm_debug::denali_go("denali/farm_reward_distr_scen_4.scen.json");
}

#[test]
fn farm_with_moax_token_go() {
    dharitri_wasm_debug::denali_go("denali/farm_with_moax_token.scen.json");
}

#[test]
fn farm_wrong_lp_token_go() {
    dharitri_wasm_debug::denali_go("denali/farm_wrong_lp_token.scen.json");
}

#[test]
fn get_amounts_go() {
    dharitri_wasm_debug::denali_go("denali/get_amounts.scen.json");
}

#[test]
fn get_amounts_no_liquidity_go() {
    dharitri_wasm_debug::denali_go("denali/get_amounts_no_liquidity.scen.json");
}

#[test]
fn get_pair_non_existent_go() {
    dharitri_wasm_debug::denali_go("denali/get_pair_non_existent.scen.json");
}

#[test]
fn get_pair_views_go() {
    dharitri_wasm_debug::denali_go("denali/get_pair_views.scen.json");
}

#[test]
fn merge_tokens_go() {
    dharitri_wasm_debug::denali_go("denali/merge_tokens.scen.json");
}

#[test]
fn owner_pause_farm_go() {
    dharitri_wasm_debug::denali_go("denali/owner_pause_farm.scen.json");
}

#[test]
fn owner_resume_farm_go() {
    dharitri_wasm_debug::denali_go("denali/owner_resume_farm.scen.json");
}

#[test]
fn remove_liquidity_go() {
    dharitri_wasm_debug::denali_go("denali/remove_liquidity.scen.json");
}

#[test]
fn remove_liquidity_and_buyback_and_burn_token_go() {
    dharitri_wasm_debug::denali_go("denali/remove_liquidity_and_buyback_and_burn_token.scen.json");
}

#[test]
fn remove_liquidity_twice_go() {
    dharitri_wasm_debug::denali_go("denali/remove_liquidity_twice.scen.json");
}

#[test]
fn remove_pair_go() {
    dharitri_wasm_debug::denali_go("denali/remove_pair.scen.json");
}

#[test]
fn router_pause_self_go() {
    dharitri_wasm_debug::denali_go("denali/router_pause_self.scen.json");
}

#[test]
fn router_resume_self_go() {
    dharitri_wasm_debug::denali_go("denali/router_resume_self.scen.json");
}

#[test]
fn swap_fixed_input_go() {
    dharitri_wasm_debug::denali_go("denali/swap_fixed_input.scen.json");
}

#[test]
fn swap_fixed_input_after_removed_liquidity_go() {
    dharitri_wasm_debug::denali_go("denali/swap_fixed_input_after_removed_liquidity.scen.json");
}

#[test]
fn swap_fixed_output_go() {
    dharitri_wasm_debug::denali_go("denali/swap_fixed_output.scen.json");
}

#[test]
fn swap_same_token_go() {
    dharitri_wasm_debug::denali_go("denali/swap_same_token.scen.json");
}

#[test]
fn swap_wrong_token_go() {
    dharitri_wasm_debug::denali_go("denali/swap_wrong_token.scen.json");
}

#[test]
fn upgrade_contract_go() {
    dharitri_wasm_debug::denali_go("denali/upgrade_contract.scen.json");
}
