use std::convert::TryInto;
use std::str;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, Promise, env, Gas};
use near_sdk::json_types::{ValidAccountId, Base58CryptoHash};

const CREATE_GAS: Gas = 20 * 10u64.pow(12);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct FactoryContract {
}

#[no_mangle]
pub extern "C" fn set_code() {
    env::setup_panic_hook();

    unsafe {
        near_sys::current_account_id(0);
        let mut current_account_id: Vec<u8> = vec![];
        current_account_id.resize(near_sys::register_len(0) as _, 0);
        near_sys::read_register(0, current_account_id.as_ptr() as _);
        let current_account_id: &str = str::from_utf8(&current_account_id).unwrap();

        near_sys::predecessor_account_id(0);
        let mut predecessor_account_id: Vec<u8> = vec![];
        predecessor_account_id.resize(near_sys::register_len(0) as _, 0);
        near_sys::read_register(0, predecessor_account_id.as_ptr() as _);
        let predecessor_account_id: &str = str::from_utf8(&predecessor_account_id).unwrap();

        assert_eq!(
            current_account_id,
            predecessor_account_id,
            "expected current_account_id as caller",
        );

        // save input to internal register
        near_sys::input(0);
        // set key
        let key = "code".as_bytes();
        near_sys::write_register(1, key.len() as u64, key.as_ptr() as u64);

        assert_eq!(
            near_sys::storage_has_key(u64::MAX as _, 1 as _),
            0,
            "set_code has already been called"
        );

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
    pub fn create(&mut self, name: String, init_function: String, init_args: String) -> Promise {
        let account_id: ValidAccountId = format!("{}.{}", name, env::current_account_id()).try_into().unwrap();
        let code = env::storage_read(&"code".as_bytes()).expect("code not set, call set_code first");
        Promise::new(account_id.into())
            .create_account()
            .transfer(env::attached_deposit())
            .deploy_contract(code)
            .function_call(
                init_function.into_bytes(),
                init_args.into_bytes(),
                0,
                env::prepaid_gas() - CREATE_GAS,
            )
    }

    pub fn get_code_hash(&self) -> Option<Base58CryptoHash> {
        let code = env::storage_read(&"code".as_bytes());
        if let Some(code) = code {
            let result: [u8; 32] = env::sha256(&code).try_into().unwrap();
            Some(result.into())
        } else {
            None
        }
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
