pub const WRONG_TOKEN_ID: &[u8] = b"Wrong payment token id for fee";
pub const NOT_ENOUGH_FEE: &[u8] = b"Minimum fee required not reached";
pub const NOT_ENOUGH_ENERGY: &[u8] = b"Not enough energy for propose";
pub const TOO_MUCH_GAS: &[u8] = b"Actions require too much gas to be executed";
pub const PROPOSAL_NOT_ACTIVE: &[u8] = b"Proposal is not active";
pub const ERROR_NOT_AN_DCT: &[u8] = b"Not a valid dct id";
pub const ALREADY_VOTED_ERR_MSG: &[u8] = b"Already voted for this proposal";
pub const EXEEDED_MAX_ACTIONS: &[u8] = b"Exceeded max actions per proposal";
pub const ONLY_PROPOSER_CANCEL: &[u8] = b"Only original proposer may cancel a pending proposal";
pub const ONLY_PROPOSER_WITHDRAW: &[u8] = b"Only original proposer may withdraw a pending proposal";
pub const FEE_ALREADY_WITHDRAWN: &[u8] = b"Fee already withdrawn!";
pub const NO_PROPOSAL: &[u8] = b"Proposal does not exist";
pub const WITHDRAW_NOT_ALLOWED: &[u8] = b"You may not withdraw funds from this proposal!";
pub const PROPOSAL_NOT_ALLOWED_FOR_SC: &[u8] = b"Smart Contracts are not allowed to propose!";
