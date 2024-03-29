// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    pause_all
    (
        init => init
        upgrade => upgrade
        addPausableContracts => add_pausable_contracts
        removePausableContracts => remove_pausable_contracts
        pauseSelected => pause_selected
        pauseAll => pause_all
        resumeSelected => resume_selected
        resumeAll => resume_all
        getPausableContracts => pausable_contracts
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
