// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            9
// Async Callback:                       1
// Total number of exported functions:  11

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    locked_token_wrapper
    (
        init => init
        upgrade => upgrade
        wrapLockedToken => wrap_locked_token_endpoint
        unwrapLockedToken => unwrap_locked_token_endpoint
        issueWrappedToken => issue_wrapped_token
        setTransferRoleWrappedToken => set_transfer_role
        unsetTransferRoleWrappedToken => unset_transfer_role
        getWrappedTokenId => wrapped_token
        setEnergyFactoryAddress => set_energy_factory_address
        getEnergyFactoryAddress => energy_factory_address
    )
}

dharitri_sc_wasm_adapter::async_callback! { locked_token_wrapper }
