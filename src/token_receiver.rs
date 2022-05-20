use crate::*;
use near_sdk::json_types::{WrappedBalance, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, serde_json, AccountId, PromiseOrValue};

#[derive(Serialize, Deserialize)]
pub struct CreateMarketArgs {
    pub outcomes: u16,
    pub challenge_period: U64,
    pub token_id: AccountId,
    pub fee: U128,
}

#[derive(Serialize, Deserialize)]
pub struct BuyArgs {
    pub market_id: U64,
    pub outcome_target: u16,
    pub min_shares_out: WrappedBalance,
}

#[derive(Serialize, Deserialize)]
pub enum Payload {
    BuyArgs(BuyArgs),
    CreateMarketArgs(CreateMarketArgs),
}

pub trait TokenReceiver {
    fn on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: WrappedBalance,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl TokenReceiver for AMMContract {
    #[payable]
    fn on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: WrappedBalance,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let amount: u128 = amount.into();
        assert!(amount > 0);
        let usage = env::storage_usage();
        let account = self.get_storage_account(&sender_id);
        let payload: Payload = serde_json::from_str(&msg).expect("json err");
        let res = match payload {
            Payload::BuyArgs(pay) => self.buy(&sender_id, amount, pay),
            Payload::CreateMarketArgs(pay) => {
                self.create_market_callback(&sender_id, amount, pay).into()
            }
        };
        self.use_storage(&sender_id, usage, account.available);
        res
    }
}
