use std::convert::TryInto;
use std::str;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base58CryptoHash, ValidAccountId, WrappedBalance};
use near_sdk::{
    env, ext_contract, is_promise_success, log, near_bindgen, AccountId, Balance, Gas, Promise,
};

const AFTER_CREATE_GAS: Gas = 10 * 10u64.pow(12);
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct FactoryContract {}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn after_create(&mut self, account_id: AccountId, amount: WrappedBalance);
}

unsafe fn read_register_as_string(register_id: u64) -> String {
    let len = near_sys::register_len(register_id);
    assert_ne!(len, u64::MAX, "register is not used");
    let res = vec![0u8; len as usize];
    near_sys::read_register(register_id, res.as_ptr() as _);
    str::from_utf8(&res).unwrap().to_string()
}

#[no_mangle]
pub extern "C" fn set_code() {
    env::setup_panic_hook();

    unsafe {
        near_sys::current_account_id(0);
        let current_account_id: String = read_register_as_string(0);
        near_sys::predecessor_account_id(0);
        let predecessor_account_id: String = read_register_as_string(0);
        // FIXME: Using env::current_account_id(), env::predecessor_account_id() fails with
        // WebAssembly trap: An `unreachable` opcode was executed.
        assert_eq!(
            current_account_id, predecessor_account_id,
            "expected current_account_id as caller",
        );

        // set key
        let key = "code".as_bytes();
        near_sys::write_register(1, key.len() as u64, key.as_ptr() as u64);

        assert_eq!(
            near_sys::storage_has_key(u64::MAX as _, 1 as _),
            0,
            "set_code has already been called",
        );

        // save code to the internal register
        near_sys::input(0);
        // save code to the state
        near_sys::storage_write(u64::MAX as _, 1 as _, u64::MAX as _, 0 as _, 2);
        // return true
        let result = near_sdk::serde_json::to_string(&true).unwrap().into_bytes();
        near_sys::value_return(result.len() as _, result.as_ptr() as _);
    }
}

#[near_bindgen]
impl FactoryContract {
    pub fn get_code_hash(&self) -> Option<Base58CryptoHash> {
        env::storage_read(&"code".as_bytes()).map(|code| {
            let result: [u8; 32] = env::sha256(&code).try_into().unwrap();
            result.into()
        })
    }

    #[payable]
    pub fn create(
        &mut self,
        name: String,
        init_function: Option<String>,
        init_args: Option<String>,
    ) -> Promise {
        let account_id: ValidAccountId = format!("{}.{}", name, env::current_account_id())
            .try_into()
            .unwrap();
        let code =
            env::storage_read(&"code".as_bytes()).expect("code not set, call set_code first");
        let promise = Promise::new(account_id.into())
            .create_account()
            .deploy_contract(code)
            .transfer(env::attached_deposit());

        assert!(env::prepaid_gas() >= AFTER_CREATE_GAS, "not enough gas");
        let promise = if init_function.is_some() && init_args.is_some() {
            promise.function_call(
                init_function.unwrap().into_bytes(),
                init_args.unwrap().into_bytes(),
                NO_DEPOSIT,
                // chosen empirically as the best value
                (env::prepaid_gas() - AFTER_CREATE_GAS) / 2,
            )
        } else {
            if init_function.is_some() || init_args.is_some() {
                panic!("expected both init_function and init_args")
            }
            promise
        };

        promise.then(ext_self::after_create(
            env::predecessor_account_id(),
            env::attached_deposit().into(),
            &env::current_account_id(),
            NO_DEPOSIT,
            AFTER_CREATE_GAS,
        ))
    }

    #[private]
    pub fn after_create(account_id: AccountId, amount: WrappedBalance) -> bool {
        let promise_success = is_promise_success();
        if promise_success {
            log!("Subcontract successfully created!");
            true
        } else {
            log!(
                "Subcontract creation failed, refunding {} to {}!",
                amount.0,
                account_id,
            );
            Promise::new(account_id).transfer(amount.0);
            false
        }
    }
}
