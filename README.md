# Generic factory contract

Generic contract factory that doesn't require factory recompilation for setting
the contract code. Code is loaded via custom method after the factory contract
is deployed. The code can only be loaded once, and only by the
current_account_id. Factory creates accounts without access keys so it's
impossible to upgrade the contract, unless this functionality is build into the
contract code.

# Usage example:

Deploy and set code:

Example:

```
./deploy.sh hello_generic_factory.testnet res/hello_contract.wasm
ACC_ROOT=hello_generic_factory.testnet
WASM_FILE=res/hello_contract.wasm
...
TxHash: BvwgFb6VofmwXr6P7ieTJQDJVzyY3TYHpx3Z6Vu6rCXk
```
See in explorer: https://explorer.testnet.near.org/transactions/BvwgFb6VofmwXr6P7ieTJQDJVzyY3TYHpx3Z6Vu6rCXk

After that, any account_id can call create subaccount with the provided code.

```
near call --accountId generic_factory_user.testnet hello_generic_factory.testnet create '{"name":"subaccount_without_args"}' --deposit 3.0
Scheduling a call: hello_generic_factory.testnet.create({"name":"subaccount_without_args"}) with attached 3 NEAR
Doing account.functionCall()
Transaction Id CakTui7iWspjF6iorvNbmTRf9Yb7d3SX8H4qCL3u2PnL
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/CakTui7iWspjF6iorvNbmTRf9Yb7d3SX8H4qCL3u2PnL
''
```

Optionally the user can provide init arguments to the contract

```
near call --accountId generic_factory_user.testnet hello_generic_factory.testnet create '{"name":"subaccount_with_args","init_function":"new","init_args":"{\"subject\":\"world\"}"}' --deposit 3.0
Scheduling a call: hello_generic_factory.testnet.create({"name":"subaccount_with_args","init_function":"new","init_args":"{\"subject\":\"world\"}"}) with attached 3 NEAR
Doing account.functionCall()
Transaction Id FuEBMVkNvN8NhJw9Hi54R3cGX4rP4TLmzm88ofrBZbJk
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/FuEBMVkNvN8NhJw9Hi54R3cGX4rP4TLmzm88ofrBZbJk
''
```
See in explorer: https://explorer.testnet.near.org/transactions/FuEBMVkNvN8NhJw9Hi54R3cGX4rP4TLmzm88ofrBZbJk

NOTE: currently the contract does NOT return the deposit in case of failure.
This should be fixed before the contract is ready to use.
