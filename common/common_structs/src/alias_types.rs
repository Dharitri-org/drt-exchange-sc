dharitri_wasm::imports!();

use crate::{LockedAssetTokenAttributes, LockedAssetTokenAttributesEx, UnlockSchedule};

pub type Nonce = u64;
pub type Epoch = u64;
pub type Week = usize;
pub type Percent = u64;
pub type PaymentsVec<M> = ManagedVec<M, DctTokenPayment<M>>;
pub type UnlockPeriod<M> = UnlockSchedule<M>;
pub type OldLockedTokenAttributes<M> = LockedAssetTokenAttributesEx<M>;
pub type InitialOldLockedTokenAttributes<M> = LockedAssetTokenAttributes<M>;
