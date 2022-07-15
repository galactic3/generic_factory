use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct HelloContract {
    subject: String
}

#[near_bindgen]
impl HelloContract {
    #[init]
    pub fn new(subject: String) -> Self {
        Self { subject }
    }

    pub fn hello(&self) -> String {
        format!("Hello, {}!", &self.subject)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    use std::convert::TryInto;

    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context() -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("alice.near".try_into().unwrap())
            .is_view(true)
            .build()
    }

    #[test]
    fn test_hello() {
        let context = get_context();
        testing_env!(context);
        let contract = HelloContract::new("world".into());
        let res = contract.hello();
        assert_eq!(res, "Hello, world!");
    }
}
