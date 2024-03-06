// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           25
// Async Callback:                       1
// Total number of exported functions:  27

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    factory
    (
        init => init
        upgrade => upgrade
        whitelist => whitelist
        removeWhitelist => remove_whitelist
        createAndForwardCustomPeriod => create_and_forward_custom_period
        createAndForward => create_and_forward
        unlockAssets => unlock_assets
        lockAssets => lock_assets
        setUnlockPeriod => set_unlock_period
        registerLockedAssetToken => register_locked_asset_token
        setInitEpoch => set_init_epoch
        getInitEpoch => init_epoch
        getWhitelistedContracts => whitelisted_contracts
        getDefaultUnlockPeriod => default_unlock_period
        getLockedAssetTokenId => locked_asset_token
        getAssetTokenId => asset_token_id
        getUnlockScheduleForSFTNonce => get_unlock_schedule_for_sft_nonce
        getCacheSize => get_cache_size
        mergeTokens => merge_tokens
        getExtendedAttributesActivationNonce => extended_attributes_activation_nonce
        setLockedTokenBurnRoleForAddress => set_locked_token_burn_role_for_address
        setTransferRoleOldLockedToken => set_transfer_role_old_locked_token
        setNewFactoryAddress => set_new_factory_address
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
    )
}

dharitri_sc_wasm_adapter::async_callback! { factory }
