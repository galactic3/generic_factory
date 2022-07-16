#!/bin/zsh

set -e

if [[ "$#" -ne 2 ]]; then
  echo "usage: $0 <account> <wasm_file>"
  exit 1
fi

ACC_ROOT=$1
WASM_FILE=$2
echo "ACC_ROOT=$ACC_ROOT"
echo "WASM_FILE=$WASM_FILE"

near deploy -f --accountId $ACC_ROOT --wasmFile res/generic_factory.wasm

script=$(cat <<EOF
const main = async () => {
  const accountId = "$ACC_ROOT";
  const contractName = accountId;
  const fs = require('fs');
  const account = await near.account(accountId);
  const code = fs.readFileSync("$WASM_FILE");

  const result = await account.signAndSendTransaction(
    contractName,
    [nearAPI.transactions.functionCall("set_code", code, 300_000_000_000_000, "0")]
  );
  console.log(result);
  console.log(JSON.parse(atob(result.receipts_outcome[0].outcome.status.SuccessValue)));
  console.log('TxHash: ' + result.transaction.hash);
};

main();
EOF
)

echo "$script" | near repl
