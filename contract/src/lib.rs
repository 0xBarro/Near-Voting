use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::{LookupMap, LookupSet};

near_sdk::setup_alloc!();

mod gift;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Voting { // TODO: Rename this class
    pub accounts: LookupSet<String>,
    pub gifts: LookupMap<String, gift::Gift>,
    pub contract_owner: String,
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    
    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool, current_account_id: String, signer_account_id: String, predecessor_account_id: String) -> VMContext {
        VMContext {
            current_account_id,
            signer_account_id,
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
}