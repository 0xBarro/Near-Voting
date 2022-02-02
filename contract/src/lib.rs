use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::{LookupMap, UnorderedMap};

near_sdk::setup_alloc!();

mod gift;
use gift::Gift;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Voting { // TODO: Rename this class
    gifts: LookupMap<String, UnorderedMap<String, Gift>>,
    contract_owner: String,
}

impl Default for Voting {
    fn default() -> Self {
        Voting {gifts: LookupMap::new(b"s"), contract_owner: env::current_account_id()}
    }
}

#[near_bindgen]
impl Voting {
    pub fn add_gift(&mut self, url: &str, n_tokens_needed: usize) {
        // Check if the address already has a list of gifts
        let method_caller_account = env::current_account_id();
        let new_gift = Gift::new(url, n_tokens_needed);
        match self.get_gifts(&method_caller_account) {
            Some(mut map) => {
                map.insert(&url.to_string(), &new_gift);
                self.gifts.insert(&method_caller_account, &map);
            },
            None =>  {
                // Create Vector
                let mut map: UnorderedMap<String, Gift> = UnorderedMap::new(b"g");
                map.insert(&url.to_string(), &new_gift);
                self.gifts.insert(&method_caller_account, &map);       
            },
        };
    }

    pub fn get_gifts(&self, account_name: &str) -> Option<UnorderedMap<String, Gift>> {
        return self.gifts.get(&account_name.to_string());
    }

    #[payable]
    pub fn contribute_to_gift(&mut self, account_name: &str, gift_url: &str) {

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
    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "jane.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 100,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn add_gift() {
        let caller_account = "alice.testnet";
        let context = get_context(caller_account.to_string(), 0);              
        testing_env!(context);    

        let mut contract = Voting::default();  
        let gift1_url = "test1.com";
        let gift2_url = "test2.com";

        contract.add_gift(gift1_url, 10);
        assert_eq!(contract.gifts.get(&caller_account.to_string()).unwrap().get(&gift1_url.to_string()).unwrap().get_url(), gift1_url);

        contract.add_gift(gift2_url, 20);
        assert_eq!(contract.gifts.get(&caller_account.to_string()).unwrap().get(&gift2_url.to_string()).unwrap().get_url(), gift2_url);

        assert_eq!(contract.gifts.get(&caller_account.to_string()).unwrap().len(), 2);
    }
}