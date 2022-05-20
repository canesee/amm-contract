use near_sdk::{ext_contract, json_types::U128, AccountId, Gas, Promise};

#[ext_contract]
pub trait Token {
    fn transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        msg: String,
        memo: Option<String>,
    );
    fn transfer(&mut self, receiver_id: AccountId, amount: U128);
}

const GAS_BASE_TRANSFER: Gas = 35_000_000_000_000;

pub fn transfer_call(
    token_account_id: &AccountId,
    receiver_id: AccountId,
    value: u128,
    msg: String,
    gas: Option<Gas>,
) -> Promise {
    token::transfer_call(
        receiver_id,
        U128(value),
        msg,
        None,
        token_account_id,
        1,
        gas.unwrap_or(GAS_BASE_TRANSFER),
    )
}

pub fn token_transfer(
    token_account_id: &AccountId,
    receiver_id: AccountId,
    value: u128,
) -> Promise {
    token::transfer(
        receiver_id,
        U128(value),
        token_account_id,
        1,
        GAS_BASE_TRANSFER,
    )
}
