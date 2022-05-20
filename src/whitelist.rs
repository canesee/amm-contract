use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Token {
    pub id: AccountId,
    pub decimals: u32,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Whitelist(pub UnorderedMap<AccountId, u32>);

impl Whitelist {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut list = UnorderedMap::new(b"wl".to_vec());
        for token in tokens {
            list.insert(&token.id, &token.decimals);
        }
        Self(list)
    }
}

#[near_bindgen]
impl AMMContract {
    pub fn get_whitelist(&self) -> Vec<(AccountId, u32)> {
        self.whitelist.0.to_vec()
    }
    pub fn set_whitelist(&mut self, tokens: Vec<Token>) {
        assert_eq!(env::predecessor_account_id(), self.gov);
        self.whitelist = Whitelist::new(tokens);
    }
    pub fn add_whitelist(&mut self, add: Token) {
        assert_eq!(env::predecessor_account_id(), self.gov);
        self.whitelist.0.insert(&add.id, &add.decimals);
    }
}
