use crate::outcome_token::OutComeToken;
use crate::*;
use near_sdk::collections::UnorderedMap;
use near_sdk::Balance;
use std::cmp::Ordering;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Pool {
    pub id: u64,
    pub token_id: AccountId,
    pub deno: u128,
    pub outcomes: u16,
    pub outcome_tokens: UnorderedMap<u16, OutComeToken>,
    pub pool_token: OutComeToken,
    pub fee: Balance,
}

impl Pool {
    pub fn new(id: u64, token_id: AccountId, decimals: u32, outcomes: u16, fee: Balance) -> Self {
        let deno = 10_u128.pow(decimals);
        Self {
            id,
            token_id,
            deno,
            outcomes,
            outcome_tokens: UnorderedMap::new(b"o".to_vec()),
            pool_token: OutComeToken::new(id, outcomes, 0),
            fee,
        }
    }

    pub fn get_pool_token_balance(&self, id: &AccountId) -> Balance {
        self.pool_token.get_balance(id)
    }

    pub fn get_pool_balances(&self) -> Vec<Balance> {
        self.outcome_tokens
            .iter()
            .map(|(_, t)| t.get_balance(&env::current_account_id()))
            .collect()
    }

    pub fn calc_buy_amount(&self, tin: Balance, out: u16) -> Balance {
        let tokens = &self.outcome_tokens;
        let fees = tin - math::mul_u128(self.deno, tin, self.fee);
        let buy = tokens.get(&out).expect("no token");
        let balance = buy.get_balance(&env::current_account_id());
        let mut new = balance;

        for (outcome, token) in tokens.iter() {
            if outcome != out {
                let balance = token.get_balance(&env::current_account_id());
                let dividend = math::mul_u128(self.deno, new, balance);
                let divisor = balance + fees;
                new = math::div_u128(self.deno, dividend, divisor);
            }
        }

        balance + fees - new
    }

    pub fn calc_sell_out(&self, tin: Balance, out: u16) -> Balance {
        let tokens = &self.outcome_tokens;
        let fees = tin - math::div_u128(self.deno, tin, self.fee);
        let sell = tokens.get(&out).expect("no token");
        let balance = sell.get_balance(&env::current_account_id());
        let mut new = balance;

        for (outcome, token) in tokens.iter() {
            if outcome != out {
                let balance = token.get_balance(&env::current_account_id());
                let dividend = math::mul_u128(self.deno, new, balance);
                let divisor = balance + fees;
                new = math::div_u128(self.deno, dividend, divisor);
            }
        }

        balance + new - balance
    }

    pub fn buy(
        &mut self,
        sender: &AccountId,
        amount_in: Balance,
        outcome_target: u16,
        min_shares_out: Balance,
    ) {
        let shares_out = self.calc_buy_amount(amount_in, outcome_target);
        assert!(shares_out >= min_shares_out);
        let fee = math::mul_u128(self.deno, amount_in, self.fee);
        let mint = amount_in - fee;
        self.add_to_pools(mint);
        let mut out = self.outcome_tokens.get(&outcome_target).expect("no");
        out.safe_transfer_internal(&env::current_account_id(), sender, shares_out);
        self.outcome_tokens.insert(&outcome_target, &out);
    }

    pub fn sell(
        &mut self,
        sender: &AccountId,
        amount_out: Balance,
        outcome_target: u16,
        max_shares_in: Balance,
    ) -> Balance {
        let shares_in = self.calc_sell_out(amount_out, outcome_target);
        assert!(shares_in <= max_shares_in);
        let mut token_in = self.outcome_tokens.get(&outcome_target).expect("no");

        let fee = math::mul_u128(self.deno, amount_out, self.fee);
        let avg = math::div_u128(self.deno, 0, token_in.get_balance(sender));
        let sell = math::div_u128(self.deno, amount_out + fee, shares_in);

        token_in.transfer(&env::current_account_id(), shares_in);
        self.outcome_tokens.insert(&outcome_target, &token_in);

        let to = match (sell).cmp(&avg) {
            Ordering::Less => {
                let d = avg - sell;
                let amt = math::mul(self.deno, d, shares_in);
                amt
            }
            Ordering::Greater => {
                let d = sell - avg;
                let amt = math::mul(self.deno, d, shares_in);
                amt
            }
            Ordering::Equal => 0,
        };
        let burn = amount_out + fee;
        self.remove_from_pools(burn);
        to
    }

    fn add_to_pools(&mut self, amount: Balance) {
        for outcome in 0..self.outcomes {
            let mut token = self.outcome_tokens.get(&outcome).expect("out");
            token.mint(&env::current_account_id(), amount);
            self.outcome_tokens.insert(&outcome, &token);
        }
    }

    fn remove_from_pools(&mut self, amount: Balance) {
        for outcome in 0..self.outcomes {
            let mut token = self.outcome_tokens.get(&outcome).expect("out");
            token.burn(&env::current_account_id(), amount);
            self.outcome_tokens.insert(&outcome, &token);
        }
    }
}
