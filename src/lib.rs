use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{near_bindgen, BorshStorageKey, PanicOnDefault};

pub mod metadata;
pub mod token;
pub mod utils;

use metadata::AmmTokenMetadata;
use token::{AmmToken, Token, TokenId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AmmContract {
    token_a: AmmToken,
    metadata_a: LazyOption<AmmTokenMetadata>,
    token_b: AmmToken,
    metadata_b: LazyOption<AmmTokenMetadata>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    AmmToken,
    TokenMetadata,
    Enumeration,
    Approval,
}

pub trait AmmTokenMetadataProvider {
    fn amm_metadata(&self) -> Vec<AmmTokenMetadata>;
}

#[near_bindgen]
impl AmmTokenMetadataProvider for AmmContract {
    fn amm_metadata(&self) -> Vec<AmmTokenMetadata> {
        vec![
            self.metadata_a.get().unwrap(),
            self.metadata_a.get().unwrap(),
        ]
    }
}

#[near_bindgen]
impl AmmContract {
    #[init]
    pub fn new(owner_id: ValidAccountId, a: TokenId, b: TokenId) -> Self {
        let token_metadata_a = AmmTokenMetadata {
            name: "a".to_owned(),
            decimals: 24,
            ratio: 1.0,
            ticker: 1.0,
        };
        let token_metadata_b = AmmTokenMetadata {
            name: "b".to_owned(),
            decimals: 24,
            ratio: 1.0,
            ticker: 1.0,
        };
        let mut token_a = AmmToken::new(
            StorageKey::AmmToken,
            owner_id.clone(),
            Some(StorageKey::TokenMetadata),
            Some(StorageKey::Enumeration),
            Some(StorageKey::Approval),
        );
        token_a
            .token_metadata_by_id
            .as_mut()
            .and_then(|map| map.insert(&a, &token_metadata_a));

        let mut token_b = AmmToken::new(
            StorageKey::AmmToken,
            owner_id.clone(),
            Some(StorageKey::TokenMetadata),
            Some(StorageKey::Enumeration),
            Some(StorageKey::Approval),
        );
        token_b
            .token_metadata_by_id
            .as_mut()
            .and_then(|map| map.insert(&b, &token_metadata_b));

        Self {
            token_a,
            metadata_a: LazyOption::new(StorageKey::TokenMetadata, Some(&token_metadata_a)),
            token_b,
            metadata_b: LazyOption::new(StorageKey::TokenMetadata, Some(&token_metadata_b)),
        }
    }

    #[private]
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: ValidAccountId,
        token_metadata: AmmTokenMetadata,
    ) -> Token {
        self.token_a
            .mint(token_id, receiver_id, Some(token_metadata))
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{borsh::BorshSerialize, near_bindgen, BorshStorageKey};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
