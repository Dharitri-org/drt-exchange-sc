#![no_std]

dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

#[dharitri_wasm::module]
pub trait TokenSendModule {
    fn send_multiple_tokens_if_not_zero(
        &self,
        destination: &ManagedAddress,
        payments: &ManagedVec<DctTokenPayment<Self::Api>>,
    ) {
        let mut non_zero_payments = ManagedVec::new();
        for payment in payments {
            if payment.amount > 0u32 {
                non_zero_payments.push(payment);
            }
        }

        if !non_zero_payments.is_empty() {
            self.send().direct_multi(destination, &non_zero_payments)
        }
    }

    fn send_tokens_non_zero(
        &self,
        to: &ManagedAddress,
        token_id: &TokenIdentifier,
        token_nonce: u64,
        amount: &BigUint,
    ) {
        if amount == &0 {
            return;
        }

        self.send().direct_dct(to, token_id, token_nonce, amount);
    }

    fn send_payment_non_zero(&self, to: &ManagedAddress, payment: &DctTokenPayment<Self::Api>) {
        self.send_tokens_non_zero(
            to,
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
    }
}
