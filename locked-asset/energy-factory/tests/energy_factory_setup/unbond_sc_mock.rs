use dharitri_wasm::io::load_endpoint_args;
use dharitri_wasm::{
    contract_base::{CallableContract, ContractBase},
    types::ManagedAddress,
};
use dharitri_wasm_debug::DebugApi;

static DEPOSIT_USER_TOKENS_FN_NAME: &[u8] = b"depositUserTokens";
static DEPOSIT_FEES_FN_NAME: &[u8] = b"depositFees";

#[derive(Clone)]
pub struct UnbondScMock {}

impl ContractBase for UnbondScMock {
    type Api = DebugApi;
}

impl CallableContract for UnbondScMock {
    fn call(&self, fn_name: &[u8]) -> bool {
        if fn_name == DEPOSIT_USER_TOKENS_FN_NAME {
            self.send_to_user();
            true
        } else {
            fn_name == DEPOSIT_FEES_FN_NAME
        }
    }

    fn clone_obj(&self) -> Box<dyn CallableContract> {
        Box::new(self.clone())
    }
}

impl UnbondScMock {
    pub fn new() -> Self {
        UnbondScMock {}
    }

    // We don't test cancel unbond here, so we simply send to the user
    pub fn send_to_user(&self) {
        let [locked_tokens, unlocked_tokens] = self.call_value().multi_dct();

        let locked_tokens_burn_amount = unlocked_tokens.amount.clone();
        self.send().dct_local_burn(
            &locked_tokens.token_identifier,
            locked_tokens.token_nonce,
            &locked_tokens_burn_amount,
        );

        let (dest_user, ()) =
            load_endpoint_args::<DebugApi, (ManagedAddress<DebugApi>, ())>(("dest_user", ()));

        self.send().direct_dct(
            &dest_user,
            &unlocked_tokens.token_identifier,
            unlocked_tokens.token_nonce,
            &unlocked_tokens.amount,
        );
    }
}
