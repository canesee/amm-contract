use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, Gas, PanicOnDefault, Promise,
    PromiseOrValue,
};

near_sdk::setup_alloc!();

mod data;
mod market;
mod market_creation;
mod math;
mod outcome_token;
mod pool;
mod pool_factory;
mod storage_manager;
mod token;
mod token_receiver;
mod whitelist;

use market::Market;
use storage_manager::AccountStorageBalance;
use whitelist::Whitelist;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AMMContract {
    receiver: AccountId,
    gov: AccountId,
    markets: Vector<Market>,
    whitelist: Whitelist,
    accounts: LookupMap<AccountId, AccountStorageBalance>,
}

#[near_bindgen]
impl AMMContract {
    #[init]
    pub fn init(
        gov: ValidAccountId,
        tokens: Vec<whitelist::Token>,
        receiver: ValidAccountId,
    ) -> Self {
        assert!(!env::state_exists());
        let list = Whitelist::new(tokens);
        Self {
            receiver: receiver.into(),
            gov: gov.into(),
            markets: Vector::new(b"v".to_vec()),
            whitelist: list,
            accounts: LookupMap::new(b"a".to_vec()),
        }
    }
}
