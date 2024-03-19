// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           29
// Async Callback (empty):               1
// Total number of exported functions:  31

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    governance_v2
    (
        init => init
        upgrade => upgrade
        propose => propose
        vote => vote
        cancel => cancel
        withdrawDeposit => withdraw_deposit
        changeMinEnergyForProposal => change_min_energy_for_propose
        changeMinFeeForProposal => change_min_fee_for_propose
        changeQuorumPercentage => change_quorum_percentage
        changeWithdrawPercentage => change_withdraw_percentage
        changeVotingDelayInBlocks => change_voting_delay_in_blocks
        changeVotingPeriodInBlocks => change_voting_period_in_blocks
        getMinEnergyForPropose => min_energy_for_propose
        getMinFeeForPropose => min_fee_for_propose
        getQuorum => quorum_percentage
        getVotingDelayInBlocks => voting_delay_in_blocks
        getVotingPeriodInBlocks => voting_period_in_blocks
        getFeeTokenId => fee_token_id
        getWithdrawPercentageDefeated => withdraw_percentage_defeated
        getProposals => proposals
        getUserVotedProposals => user_voted_proposals
        getProposalVotes => proposal_votes
        getProposalStatus => get_proposal_status
        getFeesCollectorAddress => fees_collector_address
        setEnergyFactoryAddress => set_energy_factory_address
        getEnergyFactoryAddress => energy_factory_address
        addAdmin => add_admin_endpoint
        removeAdmin => remove_admin_endpoint
        updateOwnerOrAdmin => update_owner_or_admin_endpoint
        getPermissions => permissions
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
