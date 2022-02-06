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
        Voting {gifts: LookupMap::new(b"a"), contract_owner: env::current_account_id()}
    }
}

#[near_bindgen]
impl Voting {
    pub fn add_gift(&mut self, url: String, n_tokens_needed: String) -> String {
        // Check if the address already has a list of gifts
        let method_caller_account = env::signer_account_id();
        let new_gift = Gift::new(&url, n_tokens_needed.parse().unwrap());

        match self.gifts.get(&method_caller_account) {
            Some(mut map) => {
                map.insert(&url, &new_gift);
                self.gifts.insert(&method_caller_account, &map);
            },
            None =>  {
                // Create Vector
                let mut map: UnorderedMap<String, Gift> = UnorderedMap::new(url.as_bytes());
                map.insert(&url.to_string(), &new_gift);
                self.gifts.insert(&method_caller_account, &map);       
            },
        };

        method_caller_account
    }

    pub fn get_account_gifts(&self, account_id: String) -> Vec<Gift> {
        self.gifts.get(&account_id).unwrap().values().collect()
    }

    #[payable]
    pub fn contribute_to_gift(&mut self, account_name: String, gift_url: String) {
        let amount_given = env::attached_deposit();
        assert_eq!(amount_given > 0, true);

        let donator_account = env::current_account_id();

        println!("Thank you for giving me {}, {}", amount_given, donator_account);
        let mut gifts: UnorderedMap<String, Gift> = self.gifts.get(&account_name.to_string()).unwrap();
        let mut gift: Gift = gifts.get(&gift_url.to_string()).unwrap();

        println!("BEFORE INC: {:?}", gift);
        gift.send_tokens(amount_given as usize);
        println!("AFTER INC: {:?}", gift);

        gifts.insert(&gift_url.to_string(), &gift);
        self.gifts.insert(&account_name.to_string(), &gifts);
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
    fn get_context(current_account_id: String, predecessor_account_id: String, storage_usage: u64, attached_deposit: u128) -> VMContext {
        VMContext {
            current_account_id,
            signer_account_id: "signer.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 100,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit,
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
        let context = get_context(caller_account.to_string(), "Pred".to_string(), 0, 0);              
        testing_env!(context);    

        let mut contract = Voting::default();  
        let gift1_url = "test1.com";
        let gift2_url = "test2.com";

        contract.add_gift(gift1_url.to_string(), "10".to_string());
        assert_eq!(contract.gifts.get(&"signer.testnet".to_string()).unwrap().get(&gift1_url.to_string()).unwrap().get_url(), gift1_url);

        contract.add_gift(gift2_url.to_string(), "20".to_string());
        assert_eq!(contract.gifts.get(&"signer.testnet".to_string()).unwrap().get(&gift2_url.to_string()).unwrap().get_url(), gift2_url);

        assert_eq!(contract.gifts.get(&"signer.testnet".to_string()).unwrap().len(), 2);

        let gifts: Vec<Gift> = contract.get_account_gifts("signer.testnet".to_string());
        assert_eq!(gifts.len(), 2);
    }

    #[test]
    fn contribute_gift() {
        // Add the gift
        let caller_account = "alice.testnet";
        let context = get_context(caller_account.to_string(), caller_account.to_string(), 0, 0);              
        testing_env!(context);    

        let mut contract = Voting::default();  
        let gift1_url = "test1.com";

        contract.add_gift(gift1_url.to_string(), "50".to_string());
        assert_eq!(contract.gifts.get(&"signer.testnet".to_string()).unwrap().get(&gift1_url.to_string()).unwrap().get_url(), gift1_url);

        let gift = contract.gifts.get(&"signer.testnet".to_string()).unwrap().get(&gift1_url.to_string()).unwrap();
        assert_eq!(gift.n_tokens_needed(), 50);

        // // Pay contract
        // let context = get_context("ben.testnet".to_string(), "ben.testnet".to_string(), 0, 20);              
        // testing_env!(context);    
        // contract.contribute_to_gift(caller_account.to_string(), gift1_url.to_string());
        // let gift = contract.gifts.get(&caller_account.to_string()).unwrap().get(&gift1_url.to_string()).unwrap();
        // assert_eq!(gift.n_tokens_needed(), 30)

    }

}