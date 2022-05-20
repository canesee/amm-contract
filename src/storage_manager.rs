use crate::*;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::serde::Serialize;
use near_sdk::{Balance, Promise};

pub const STORAGE_PRICE_PER_BYTE: Balance = 10_000_000_000_000_000_000;

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageBalance {
    pub total: U128,
    pub available: U128,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AccountStorageBalance {
    pub total: u128,
    pub available: u128,
}

pub trait StorageManager {
    fn storage_deposit(&mut self, id: Option<ValidAccountId>) -> StorageBalance;
    fn storage_withdraw(&mut self, amount: U128) -> StorageBalance;
    fn storage_balance_of(&self, id: ValidAccountId) -> Option<StorageBalance>;
}

#[near_bindgen]
impl StorageManager for AMMContract {
    #[payable]
    fn storage_deposit(&mut self, id: Option<ValidAccountId>) -> StorageBalance {
        let amount = env::attached_deposit();
        let id = id
            .map(|a| a.into())
            .unwrap_or_else(env::predecessor_account_id);

        let mut account = self.get_storage_account(&id);
        account.available += amount;
        account.total += amount;
        self.accounts.insert(&id, &account);
        StorageBalance {
            total: U128(account.total),
            available: U128(account.available),
        }
    }

    #[payable]
    fn storage_withdraw(&mut self, amount: U128) -> StorageBalance {
        let amount = amount.into();
        let id = env::predecessor_account_id();
        let mut account = self.get_storage_account(&id);
        account.available -= amount;
        account.total -= amount;
        self.accounts.insert(&id, &account);
        Promise::new(id).transfer(amount);
        StorageBalance {
            total: U128(account.total),
            available: U128(account.available),
        }
    }

    fn storage_balance_of(&self, id: ValidAccountId) -> Option<StorageBalance> {
        self.accounts.get(id.as_ref()).map(|a| StorageBalance {
            total: U128(a.total),
            available: U128(a.available),
        })
    }
}

impl AMMContract {
    pub fn get_storage_account(&self, id: &AccountId) -> AccountStorageBalance {
        self.accounts.get(id).unwrap_or(AccountStorageBalance {
            total: 0,
            available: 0,
        })
    }
    pub fn use_storage(&mut self, sender_id: &AccountId, usage: u64, balance: u128) {
        let d = u128::from(usage - env::storage_usage());
        let mut account = self.get_storage_account(sender_id);
        account.available = balance + d * STORAGE_PRICE_PER_BYTE;
        self.accounts.insert(sender_id, &account);
    }
}
