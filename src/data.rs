use crate::*;
use near_sdk::json_types::U64;
use near_sdk::serde_json::json;

pub struct DataRequestArgs {
    pub tags: Vec<String>,
    pub challenge_period: U64,
}

const GAS_BASE_TRANSFER: Gas = 50_000_000_000_000;

impl AMMContract {
    pub fn create_data_request(
        &self,
        bond_token: &AccountId,
        amount: Balance,
        args: DataRequestArgs,
    ) -> Promise {
        token::transfer_call(
            bond_token,
            self.receiver.to_string(),
            amount,
            json!({
                "NewDataRequest": {
                    "challenge_period": args.challenge_period,
                    "target_contract": env::current_account_id(),
                    "tags": args.tags,
                }
            })
            .to_string(),
            Some(GAS_BASE_TRANSFER),
        )
    }
}
