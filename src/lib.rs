use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::serde::{Deserialize, Serialize};

type AccountId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AmmTokenMetadata {
    pub owner_id: AccountId,
    pub name: String,
    pub decimals: u8,
}


#[cfg(test)]
mod tests {
    use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
    use serde::{Serialize, Deserialize};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
