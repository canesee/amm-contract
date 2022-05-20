use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env, AccountId, Balance, PanicOnDefault,
};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MintableToken {
    pub accounts: LookupMap<AccountId, Balance>,
    pub total_supply: Balance,
    pub pool_id: u64,
    pub outcome_id: u16,
}

impl MintableToken {
    pub fn new(pool_id: u64, outcome_id: u16, initial: Balance) -> Self {
        let mut accounts = LookupMap::new(b"mt".to_vec());
        accounts.insert(&env::current_account_id(), &initial);
        Self {
            total_supply: initial,
            accounts,
            pool_id,
            outcome_id,
        }
    }

    pub fn mint(&mut self, id: &AccountId, amount: Balance) {
        self.total_supply += amount;
        let balance = self.accounts.get(&id).unwrap_or(0);
        let new = balance + amount;
        self.accounts.insert(id, &new);
    }

    pub fn burn(&mut self, id: &AccountId, amount: Balance) {
        let balance = self.accounts.get(&id).unwrap_or(0);
        let new = balance - amount;
        self.accounts.insert(id, &new);
        self.total_supply -= amount;
    }

    pub fn deposit(&mut self, id: &AccountId, amount: Balance) {
        let balance = self.accounts.get(&id).unwrap_or(0);
        let new = balance + amount;
        self.accounts.insert(id, &new);
    }

    pub fn withdraw(&mut self, id: &AccountId, amount: Balance) {
        let balance = self.accounts.get(&id).unwrap_or(0);
        let new = balance - amount;
        self.accounts.insert(id, &new);
    }
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OutComeToken {
    pub token: MintableToken,
}

impl OutComeToken {
    pub fn new(pool_id: u64, outcome_id: u16, initial: Balance) -> Self {
        Self {
            token: MintableToken::new(pool_id, outcome_id, initial),
        }
    }

    pub fn get_balance(&self, id: &AccountId) -> Balance {
        self.token.accounts.get(id).unwrap_or(0)
    }

    pub fn total_supply(&self) -> Balance {
        self.token.total_supply
    }

    pub fn mint(&mut self, id: &AccountId, amount: Balance) {
        self.token.mint(id, amount);
    }

    pub fn burn(&mut self, id: &AccountId, amount: Balance) {
        self.token.burn(id, amount);
    }

    pub fn remove_account(&mut self, id: &AccountId) -> Option<Balance> {
        self.token.accounts.remove(id)
    }

    pub fn transfer(&mut self, id: &AccountId, amount: Balance) {
        self.token.withdraw(&env::predecessor_account_id(), amount);
        self.token.deposit(id, amount);
    }

    pub fn safe_transfer_internal(&mut self, s_id: &AccountId, r_id: &AccountId, amount: Balance) {
        self.token.withdraw(s_id, amount);
        self.token.deposit(r_id, amount);
    }
}
