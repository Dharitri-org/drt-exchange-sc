dharitri_sc::imports!();
dharitri_sc::derive_imports!();

use crate::config;

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug, Clone)]
pub enum VoteType {
    Upvote = 1,
    DownVote = 2,
}

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct VoteNFTAttributes<M: ManagedTypeApi> {
    pub proposal_id: u64,
    pub vote_type: VoteType,
    pub vote_weight: BigUint<M>,
    pub voter: ManagedAddress<M>,
    pub payment: DctTokenPayment<M>,
}

#[dharitri_sc::module]
pub trait VoteHelper: config::Config {
    fn create_vote_nft(
        &self,
        proposal_id: u64,
        vote_type: VoteType,
        vote_weight: BigUint,
        payment: DctTokenPayment<Self::Api>,
    ) -> DctTokenPayment<Self::Api> {
        let big_one = BigUint::from(1u64);
        let vote_nft_id = self.vote_nft_id().get();
        let attr = VoteNFTAttributes {
            proposal_id,
            vote_type,
            vote_weight,
            voter: self.blockchain().get_caller(),
            payment,
        };

        let nonce = self.send().dct_nft_create(
            &vote_nft_id,
            &big_one,
            &ManagedBuffer::new(),
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &attr,
            &ManagedVec::new(),
        );

        DctTokenPayment::new(vote_nft_id, nonce, big_one)
    }

    fn get_vote_attr(&self, payment: &DctTokenPayment<Self::Api>) -> VoteNFTAttributes<Self::Api> {
        self.blockchain()
            .get_dct_token_data(
                &self.blockchain().get_sc_address(),
                &payment.token_identifier,
                payment.token_nonce,
            )
            .decode_attributes()
    }

    fn burn_vote_nft(&self, payment: DctTokenPayment<Self::Api>) {
        self.send().dct_local_burn(
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
    }
}
