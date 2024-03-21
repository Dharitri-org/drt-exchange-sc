// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           34
// Async Callback:                       1
// Total number of exported functions:  36

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    energy_factory
    (
        init => init
        upgrade => upgrade
        lockTokens => lock_tokens_endpoint
        unlockTokens => unlock_tokens_endpoint
        extendLockPeriod => extend_lock_period
        issueLockedToken => issue_locked_token
        getLockedTokenId => locked_token
        getBaseAssetTokenId => base_asset_token_id
        getLegacyLockedTokenId => legacy_locked_token_id
        getEnergyEntryForUser => get_updated_energy_entry_for_user
        getEnergyAmountForUser => get_energy_amount_for_user
        addLockOptions => add_lock_options
        getLockOptions => get_lock_options_view
        unlockEarly => unlock_early
        reduceLockPeriod => reduce_lock_period
        getPenaltyAmount => calculate_penalty_amount
        setTokenUnstakeAddress => set_token_unstake_address
        revertUnstake => revert_unstake
        getTokenUnstakeScAddress => token_unstake_sc_address
        setEnergyForOldTokens => set_energy_for_old_tokens
        updateEnergyAfterOldTokenUnlock => update_energy_after_old_token_unlock
        migrateOldTokens => migrate_old_tokens
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
        setTransferRoleLockedToken => set_transfer_role
        setBurnRoleLockedToken => set_burn_role
        mergeTokens => merge_tokens_endpoint
        lockVirtual => lock_virtual
        addSCAddressToWhitelist => add_sc_address_to_whitelist
        removeSCAddressFromWhitelist => remove_sc_address_from_whitelist
        isSCAddressWhitelisted => is_sc_address_whitelisted
        addToTokenTransferWhitelist => add_to_token_transfer_whitelist
        removeFromTokenTransferWhitelist => remove_from_token_transfer_whitelist
        setUserEnergyAfterLockedTokenTransfer => set_user_energy_after_locked_token_transfer
    )
}

dharitri_sc_wasm_adapter::async_callback! { energy_factory }
