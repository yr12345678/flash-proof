use scrypto::prelude::*;

#[derive(ScryptoSbor, Clone)]
pub struct FeeInfo {
    pub resource: ResourceAddress,
    pub amount: Decimal,
}