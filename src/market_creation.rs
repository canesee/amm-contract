use crate::data::DataRequestArgs;
use crate::market::*;
use crate::*;
use near_sdk::json_types::{WrappedBalance, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{serde_json, PromiseResult};
use token_receiver::CreateMarketArgs;

#[ext_contract(ext_self)]
trait ProtocolResolver {
    fn proceed_datarequest_creation(
        &mut self,
        sender: AccountId,
        bond_token: AccountId,
        bond_in: WrappedBalance,
        market_id: U64,
        market_args: CreateMarketArgs,
    ) -> Promise;
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub bond_token: AccountId,
    pub validity_bound: U128,
}

#[near_bindgen]
impl AMMContract {
    pub fn proceed_datarequest_creation(
        &mut self,
        sender: AccountId,
        bond_token: AccountId,
        bond_in: WrappedBalance,
        market_id: U64,
        market_args: CreateMarketArgs,
    ) -> Promise {
        let config = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => panic!("no config"),
            PromiseResult::Successful(v) => match serde_json::from_slice::<Config>(&v) {
                Ok(v) => v,
                Err(_) => panic!("json err"),
            },
        };
        let vb = config.validity_bound.into();
        let bin: u128 = bond_in.into();
        let rb = bin - vb;
        let create_promise = self.create_data_request(
            &bond_token,
            vb,
            DataRequestArgs {
                tags: vec![market_id.0.to_string()],
                challenge_period: market_args.challenge_period,
            },
        );
        create_promise.then(token::token_transfer(&bond_token, sender, rb))
    }
}

impl AMMContract {
    pub fn create_market(&mut self, pay: &CreateMarketArgs) -> U64 {
        let fee: u128 = pay.fee.into();
        let id = self.markets.len();
        let decimals = self.whitelist.0.get(&pay.token_id);
        assert!(decimals.is_some());

        let pool = pool_factory::new_pool(
            id,
            pay.outcomes,
            pay.token_id.to_string(),
            decimals.unwrap(),
            fee,
        );

        let market = Market { pool };
        self.markets.push(&market);
        id.into()
    }

    pub fn create_market_callback(
        &mut self,
        sender: &AccountId,
        bound_in: Balance,
        payload: CreateMarketArgs,
    ) -> Promise {
        let id = self.create_market(&payload);
        ext_self::proceed_datarequest_creation(
            sender.to_string(),
            env::predecessor_account_id(),
            U128(bound_in),
            id,
            payload,
            &env::current_account_id(),
            0,
            150_000_000_000_000,
        )
    }
}
