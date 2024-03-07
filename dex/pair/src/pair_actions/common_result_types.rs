dharitri_sc::imports!();

pub type AddLiquidityResultType<M> =
    MultiValue3<DctTokenPayment<M>, DctTokenPayment<M>, DctTokenPayment<M>>;

pub type RemoveLiquidityResultType<M> = MultiValue2<DctTokenPayment<M>, DctTokenPayment<M>>;

pub type SwapTokensFixedInputResultType<M> = DctTokenPayment<M>;

pub type SwapTokensFixedOutputResultType<M> = MultiValue2<DctTokenPayment<M>, DctTokenPayment<M>>;
