// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            5
// Async Callback (empty):               1
// Total number of exported functions:   7

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    proxy_deployer
    (
        init => init
        upgrade => upgrade
        deployFarm => deploy_farm
        callFarmEndpoint => call_farm_endpoint
        getAllDeployedFarms => get_all_deployed_farms
        getDeployerFarmAddresses => deployer_farm_addresses
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
