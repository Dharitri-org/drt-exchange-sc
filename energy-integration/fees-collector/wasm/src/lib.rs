////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

dharitri_wasm_node::wasm_endpoints! {
    fees_collector
    (
        addKnownContracts
        addKnownTokens
        claimRewards
        depositSwapFees
        getAccumulatedFees
        getAllKnownContracts
        getAllTokens
        getCurrentClaimProgress
        getCurrentWeek
        getEnergyFactoryAddress
        getFirstWeekStartEpoch
        getLastActiveWeekForUser
        getLastGlobalUpdateWeek
        getLastLockedTokensAddWeek
        getLockEpochs
        getLockedTokenId
        getLockedTokensPerBlock
        getLockingScAddress
        getTotalEnergyForWeek
        getTotalLockedTokensForWeek
        getTotalRewardsForWeek
        getUserEnergyForWeek
        isPaused
        pause
        removeKnownContracts
        removeKnownTokens
        setEnergyFactoryAddress
        setLockEpochs
        setLockedTokensPerBlock
        setLockingScAddress
        unpause
        updateEnergyForUser
    )
}

dharitri_wasm_node::wasm_empty_callback! {}
