////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

dharitri_wasm_node::wasm_endpoints! {
    distribution
    (
        calculateLockedAssets
        claimLockedAssets
        clearUnclaimableAssets
        deleteUserDistributedLockedAssets
        endGlobalOperation
        getAssetTokenId
        getCommunityDistributionList
        getUnlockPeriod
        getUsersDistributedLockedAssetsLength
        setCommunityDistribution
        setPerUserDistributedLockedAssets
        setUnlockPeriod
        startGlobalOperation
        undoLastCommunityDistribution
        undoUserDistributedAssetsBetweenEpochs
    )
}

dharitri_wasm_node::wasm_empty_callback! {}
