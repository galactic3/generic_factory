use near_sdk::json_types::Base58CryptoHash;
use near_sdk::serde_json::json;
use near_sdk::{Balance, Gas, env};
use near_sdk_sim::{init_simulator, to_yocto};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    FACTORY_BYTES => "res/generic_factory.wasm",
    HELLO_BYTES => "res/hello_contract.wasm",
}

const SET_CODE_GAS: Gas = 300 * 10u64.pow(12);
const CREATE_GAS: Gas = 300 * 10u64.pow(12);
const NO_DEPOSIT: Balance = 0;

#[test]
fn test_deploy_set_code_create_call() {
    let root = init_simulator(None);
    // deploy works
    let factory = root.deploy(&FACTORY_BYTES, "factory".parse().unwrap(), to_yocto("10"));
    let user = root.create_user("user".into(), to_yocto("10"));

    // code not set, code_hash is none
    let res = root.view(factory.account_id(), "get_code_hash", &vec![]);
    assert!(res.is_ok());
    let res: Option<Base58CryptoHash> = res.unwrap_json();
    assert!(res.is_none());

    // create should fail before set_code
    let res = user.call(
        factory.account_id(),
        "create",
        &json!({
            "name": "subaccount",
            "init_function": "new",
            "init_args": json!({ "subject": "world" }).to_string(),
        }).to_string().into_bytes(),
        CREATE_GAS,
        to_yocto("5"),
    );
    assert!(!res.is_ok());
    assert!(format!("{:?}", res.status()).contains("code not set, call set_code first"));

    // set_code by not current_account_id fails
    let res = user.call(factory.account_id(), "set_code", &HELLO_BYTES, SET_CODE_GAS, NO_DEPOSIT);
    assert!(!res.is_ok());

    // set_code by current_account_id succeeds
    let res = factory.call(factory.account_id(), "set_code", &HELLO_BYTES, SET_CODE_GAS, NO_DEPOSIT);
    assert!(res.is_ok());

    // set_code by current_account_id fails second time
    let res = factory.call(factory.account_id(), "set_code", &HELLO_BYTES, SET_CODE_GAS, NO_DEPOSIT);
    assert!(!res.is_ok());

    // code not set, code_hash is none
    let res = root.view(factory.account_id(), "get_code_hash", &vec![]);
    assert!(res.is_ok());
    let res: Option<Base58CryptoHash> = res.unwrap_json();
    assert!(res.is_some());

    // create by random user works
    let res = user.call(
        factory.account_id(),
        "create",
        &json!({
            "name": "subaccount",
            "init_function": "new",
            "init_args": json!({ "subject": "world" }).to_string(),
        }).to_string().into_bytes(),
        CREATE_GAS,
        to_yocto("5"),
    );
    assert!(res.is_ok());

    // init method was called with the correct args
    let res = root.view("subaccount.factory".into(), "hello", &vec![]);
    assert!(res.is_ok());
    let res: String = res.unwrap_json();
    assert_eq!(res, "Hello, world!");

    // create by random user without init params
    let res = user.call(
        factory.account_id(),
        "create",
        &json!({
            "name": "subaccount_without_args",
        }).to_string().into_bytes(),
        CREATE_GAS,
        to_yocto("3"),
    );
    assert!(res.is_ok());

    // init method was not called, subject is empty string
    let res = root.view("subaccount_without_args.factory".into(), "hello", &vec![]);
    assert!(res.is_ok());
    let res: String = res.unwrap_json();
    assert_eq!(res, "Hello, !");
}

// - [DONE] check that create fails before set_code
// - [DONE] check that owner can set_code
// - [DONE] check that non owner cannot set_code
// - [DONE] check that create works after set_code
// - [DONE] check that anyone can create
// - [DONE] check that create calls init with correct attributes
// - [DONE] check that set_code cannot be called again
// - [DONE] check that get_code_hash works
