use near_sdk::serde_json::json;
use near_sdk::{Balance, Gas};
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
    let factory = root.deploy(&FACTORY_BYTES, "factory".parse().unwrap(), to_yocto("10"));
    let user = root.create_user("user".into(), to_yocto("10"));

    let res = root.call(factory.account_id(), "set_code", &HELLO_BYTES, SET_CODE_GAS, NO_DEPOSIT);
    assert!(res.is_ok());

    let init_args: String = json!({ "subject": "world" }).to_string();
    let res = user.call(
        factory.account_id(),
        "create",
        &json!({
            "name": "subaccount",
            "init_function": "new",
            "init_args": init_args
        }).to_string().into_bytes(),
        CREATE_GAS,
        to_yocto("5"),
    );
    assert!(res.is_ok());

    let res = root.view("subaccount.factory".into(), "hello", &vec![]);
    assert!(res.is_ok());
    let res: String = res.unwrap_json();
    assert_eq!(res, "Hello, world!");
}
