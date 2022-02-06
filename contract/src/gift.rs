use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};


#[derive(std::fmt::Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct Gift {
    url: String,
    n_tokens_required: usize,
    current_tokens: usize,  // WIll be incremented
}

impl Gift {

    // Initialized a new gift
    pub fn new(url: &str, n_tokens_required: usize) -> Gift {
        Gift{url: url.to_string(), n_tokens_required, current_tokens: 0}
    } 

    // Number of tokens needed for the gift
    pub fn n_tokens_needed(&self) -> usize {self.n_tokens_required - self.current_tokens}

    pub fn send_tokens(&mut self, n_tokens: usize) {
        // Validation for this is made on the frontend
        if n_tokens > self.n_tokens_needed() {
            panic!("Number of tokens sent are higher than the required amount") 
        } else {
            self.current_tokens += n_tokens
        }
    }

    pub fn get_url(&self) -> String {
        self.url.to_string()
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {

    use super::Gift;

    #[test]
    fn send_tokens() {
        let mut g = Gift::new("some_url", 100);

        assert_eq!(g.n_tokens_needed(), 100);
        g.send_tokens(50);
        assert_eq!(g.n_tokens_needed(), 50);
    }

    #[test]
    #[should_panic]
    fn panic_send_tokens() {
        let mut g = Gift::new("some_url", 100);
        g.send_tokens(500);
    }
}