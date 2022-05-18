use near_sdk::{env, AccountId, Balance, CryptoHash, Promise};

pub fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

pub fn refund_deposit(storage_uesed: u64) {
    let require_cost = env::storage_byte_cost() * Balance::from(storage_uesed);
    let attached_deposit = env::attached_deposit();

    assert!(
        require_cost <= attached_deposit,
        "Must attach {} yocto near to conver storage",
        require_cost,
    );

    let refund = attached_deposit - require_cost;
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}
