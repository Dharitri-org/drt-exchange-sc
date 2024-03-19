use common_structs::{
    LockedAssetTokenAttributesEx, UnlockMilestone, UnlockMilestoneEx, UnlockScheduleEx,
};
use dharitri_wasm::{
    storage::mappers::StorageTokenWrapper,
    types::{DctLocalRole, ManagedVec, MultiValueEncoded},
};
use dharitri_wasm_debug::{
    managed_biguint, managed_token_id, rust_biguint, testing_framework::*, DebugApi,
};

const SC_WASM_PATH: &str = "output/factory.wasm";

use dharitri_wasm_modules::pause::PauseModule;
use factory::{locked_asset::LockedAssetModule, LockedAssetFactory};

const ASSET_TOKEN_ID: &[u8] = b"MEX-123456";
const LOCKED_ASSET_TOKEN_ID: &[u8] = b"LKMEX-123456";

#[test]
fn test_lock_assets() {
    let mut blockchain_wrapper = BlockchainStateWrapper::new();

    let owner_addr = blockchain_wrapper.create_user_account(&rust_biguint!(0));
    let factory = blockchain_wrapper.create_sc_account(
        &rust_biguint!(0),
        Option::Some(&owner_addr),
        factory::contract_obj,
        SC_WASM_PATH,
    );

    blockchain_wrapper
        .execute_tx(&owner_addr, &factory, &rust_biguint!(0), |sc| {
            let asset_token_id = managed_token_id!(ASSET_TOKEN_ID);
            let mut unlock_period = MultiValueEncoded::new();
            unlock_period.push(UnlockMilestone {
                unlock_epoch: 1,
                unlock_percent: 100,
            });
            sc.init(asset_token_id, unlock_period);
            sc.locked_asset_token()
                .set_token_id(managed_token_id!(LOCKED_ASSET_TOKEN_ID));
            sc.set_paused(false);
        })
        .assert_ok();

    let asset_token_roles = [DctLocalRole::Burn];
    let locked_asset_token_roles = [
        DctLocalRole::NftCreate,
        DctLocalRole::NftAddQuantity,
        DctLocalRole::NftBurn,
    ];

    blockchain_wrapper.set_dct_local_roles(
        factory.address_ref(),
        ASSET_TOKEN_ID,
        &asset_token_roles[..],
    );
    blockchain_wrapper.set_dct_local_roles(
        factory.address_ref(),
        LOCKED_ASSET_TOKEN_ID,
        &locked_asset_token_roles[..],
    );

    let mut locked_assets_nonce = 0;

    blockchain_wrapper.set_dct_balance(&owner_addr, ASSET_TOKEN_ID, &rust_biguint!(2000));

    blockchain_wrapper
        .execute_dct_transfer(
            &owner_addr,
            &factory,
            ASSET_TOKEN_ID,
            0,
            &rust_biguint!(1000),
            |sc| {
                let locked_assets = sc.lock_assets();
                locked_assets_nonce = locked_assets.token_nonce;
                assert_eq!(locked_assets.amount, managed_biguint!(1000));
            },
        )
        .assert_ok();

    blockchain_wrapper.execute_in_managed_environment(|| {
        let expected_attributes = LockedAssetTokenAttributesEx::<DebugApi> {
            unlock_schedule: UnlockScheduleEx {
                unlock_milestones: ManagedVec::from(vec![UnlockMilestoneEx {
                    unlock_epoch: 1,
                    unlock_percent: 100_000,
                }]),
            },
            is_merged: false,
        };

        blockchain_wrapper.check_nft_balance(
            &owner_addr,
            LOCKED_ASSET_TOKEN_ID,
            locked_assets_nonce,
            &rust_biguint!(1000),
            Some(&expected_attributes),
        );
    });
}
