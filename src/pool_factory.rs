use crate::pool::Pool;
use near_sdk::{AccountId, Balance};

pub fn new_pool(
    pool_id: u64,
    outcomes: u16,
    token_id: AccountId,
    decimals: u32,
    fee: Balance,
) -> Pool {
    Pool::new(pool_id, token_id, decimals, outcomes, fee)
}
