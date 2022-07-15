use std::convert::TryInto;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, Promise, env};
use near_sdk::json_types::ValidAccountId;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct FactoryContract {
}

#[no_mangle]
pub extern "C" fn set_code() {
    env::setup_panic_hook();
    unsafe {
        // save input to internal register
        near_sys::input(0);
        // set key
        let key = "code".as_bytes();
        near_sys::write_register(1, key.len() as u64, key.as_ptr() as u64);
        // save code to the state
        near_sys::storage_write(u64::MAX as _, 1 as _, u64::MAX as _, 0 as _, 2);
        // return true
        let result = near_sdk::serde_json::to_string(&true).unwrap().into_bytes();
        near_sys::value_return(result.len() as _, result.as_ptr() as _);
    }
}

#[near_bindgen]
impl FactoryContract {
    #[payable]
    pub fn create(&mut self, subaccount: String) -> Promise {
        let account_id: ValidAccountId = format!("{}.{}", subaccount, env::current_account_id()).try_into().unwrap();

        let code = env::storage_read(&"code".as_bytes()).unwrap();

        Promise::new(account_id.into())
            .create_account()
            .transfer(env::attached_deposit())
            .deploy_contract(code)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

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
        FactoryContract {};
    }
}
