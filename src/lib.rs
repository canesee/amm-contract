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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::whitelist::Token;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    #[test]
    fn test_init() {
        // should be add test case in all rust file
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let ts = vec![Token::default()];
        let _contract = AMMContract::init(accounts(1).into(), ts, accounts(2).into());
    }

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }
}
