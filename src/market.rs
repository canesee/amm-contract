use crate::pool::Pool;
use crate::token_receiver::*;
use crate::*;
use near_sdk::json_types::{WrappedBalance, U128, U64};

const GAS_BASE_COMPUTE: Gas = 5_000_000_000_000;

#[ext_contract]
pub trait CollateralToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Market {
    pub pool: Pool,
}

#[near_bindgen]
impl AMMContract {
    pub fn get_pool_balances(&self, id: U64) -> Vec<WrappedBalance> {
        let market = self.get_market_expect(id);
        market
            .pool
            .get_pool_balances()
            .into_iter()
            .map(|b| b.into())
            .collect()
    }

    pub fn get_pool_token_balance(&self, market_id: U64, account_id: &AccountId) -> WrappedBalance {
        let market = self.get_market_expect(market_id);
        U128(market.pool.get_pool_token_balance(account_id))
    }

    pub fn calc_buy_amount(
        &self,
        market_id: U64,
        cin: WrappedBalance,
        outcome_target: u16,
    ) -> WrappedBalance {
        let market = self.get_market_expect(market_id);
        U128(market.pool.calc_buy_amount(cin.into(), outcome_target))
    }

    pub fn calc_sell_out(
        &self,
        market_id: U64,
        out: WrappedBalance,
        outcome_target: u16,
    ) -> WrappedBalance {
        let market = self.get_market_expect(market_id);
        U128(market.pool.calc_sell_out(out.into(), outcome_target))
    }

    #[payable]
    pub fn sell(
        &mut self,
        market_id: U64,
        out: WrappedBalance,
        outcome_target: u16,
        max_shares_in: WrappedBalance,
    ) -> Promise {
        let out = out.into();
        let mut market = self.markets.get(market_id.into()).unwrap();
        let e = market.pool.sell(
            &env::predecessor_account_id(),
            out,
            outcome_target,
            max_shares_in.into(),
        );
        self.markets.replace(market_id.into(), &market);
        collateral_token::ft_transfer(
            env::predecessor_account_id(),
            U128(out - e),
            None,
            &market.pool.token_id,
            1,
            GAS_BASE_COMPUTE,
        )
    }
}

impl AMMContract {
    pub fn get_market_expect(&self, market_id: U64) -> Market {
        self.markets.get(market_id.into()).expect("no")
    }

    pub fn buy(&mut self, sender: &AccountId, cin: u128, args: BuyArgs) -> PromiseOrValue<U128> {
        let mut market = self.markets.get(args.market_id.into()).unwrap();
        market.pool.buy(
            &sender,
            cin,
            args.outcome_target,
            args.min_shares_out.into(),
        );
        self.markets.replace(args.market_id.into(), &market);
        PromiseOrValue::Value(0.into())
    }
}
