use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::{LookupMap, LookupSet};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Voting {
    pub i: usize,
    pub contract_owner: String,
}

#[near_bindgen]
impl Voting {
    pub fn inc_value(&mut self) {
        let log_str = format!("Signer Account Id {} Incremented the Value", env::signer_account_id());
        self.i += 1;
        env::log(log_str.as_bytes());
    }

    pub fn reset_value(&mut self) {
        assert!(env::predecessor_account_id() == self.contract_owner, 
        "Only the owner {} can call the method, not {}", self.contract_owner, env::predecessor_account_id()
    );
        env::log(b"Reseting Counter");
        self.i = 0
    }
}

impl Default for Voting {
    fn default() -> Voting {
        Voting {i: 0, contract_owner: env::current_account_id()}
    }
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
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
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

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn increment() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);

        // instantiate a contract variable with the counter at zero
        let mut contract = Voting::default(); 
        assert_eq!(contract.i, 0);   
        assert_eq!(contract.contract_owner, "alice.testnet");   
        contract.inc_value();
        assert_eq!(contract.i, 1);
        contract.inc_value();
        assert_eq!(contract.i, 2);
    }

    #[test]
    #[should_panic]
    fn reset_counter() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Voting::default();

        contract.inc_value();
        contract.reset_value();

    }
}