#!/bin/sh

### Create an originator address which will make requests for validation to the smart contract
provenanced keys add loan-originator --home build/node0 --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
ORIGINATOR_ACCOUNT="$(provenanced keys show -a loan-originator --home build/node0 --keyring-backend test -t)"

### Create an address which will act as the administrator of the smart contract
provenanced keys add marketplace-admin --home build/node0 --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
ADMIN_ACCOUNT="$(provenanced keys show -a marketplace-admin --home build/node0 --keyring-backend test -t)"

### Create a validator address which will act as a third-party reviewer that services requests for validation in the smart contract
provenanced keys add loan-validator --home build/node0 --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
VALIDATOR_ACCOUNT="$(provenanced keys show -a loan-validator --home build/node0 --keyring-backend test -t)"

### Export the address of a validator node in order to fund addresses
NODE0="$(provenanced keys show -a node0 --home build/node0 --keyring-backend test -t)"

### Create the originator account
provenanced tx bank send \
    "$NODE0" \
    "$ORIGINATOR_ACCOUNT" \
    200000000000nhash \
    --from="$NODE0" \
    --keyring-backend=test \
    --home=build/node0 \
    --chain-id=chain-local \
    --gas=auto \
    --gas-prices="1905nhash" \
    --gas-adjustment=1.5 \
    --broadcast-mode=block \
    --yes \
    --testnet \
    --output json | jq

### Verify that the originator account was created & funded
provenanced q bank balances "$ORIGINATOR_ACCOUNT" -t -o json | jq

### Create the administrator account
provenanced tx bank send \
    "$NODE0" \
    "$ADMIN_ACCOUNT" \
    350000000000nhash \
    --from="$NODE0" \
    --keyring-backend=test \
    --home=build/node0 \
    --chain-id=chain-local \
    --gas=auto \
    --gas-prices="1905nhash" \
    --gas-adjustment=1.5 \
    --broadcast-mode=block \
    --yes \
    --testnet \
    --output json | jq

### Verify that the administrator account was created & funded
provenanced q bank balances "$ADMIN_ACCOUNT" -t -o json | jq

### Create an unrestricted name that we will bind the address of the smart contract to
provenanced tx name bind \
    "sc" \
    "$NODE0" \
    "pb" \
    --restrict=false \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas-prices="1905nhash" \
    --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
    --output json | jq

### Store the optimized contract WASM file to the chain — you'll need to copy the artifact to the provenance directory or change the path specified in the example command below

### Change this line to the path to your output contract WASM
PATH_TO_CONTRACT="validation_oracle_smart_contract.wasm"

### Note: Will need to use --instantiate-anyof-addresses instead of --instantiate-only-address in newer Provenance versions
WASM_STORE=$(provenanced tx wasm store "$PATH_TO_CONTRACT" \
    --instantiate-only-address "$ADMIN_ACCOUNT" \
    --from "$ADMIN_ACCOUNT" \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --gas-prices="1905nhash" \
    --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
    --output json | jq)

echo "$WASM_STORE"

### Verify that the code was stored
provenanced query wasm list-code -o json | jq

### Note the value of the code_id for our contract from the above output
VO_CODE_ID=$(echo "$WASM_STORE" | jq -r '.logs[] | select(.msg_index == 0) | .events[] | select(.type == "store_code") | .attributes[0].value')

### Instantiate the contract
provenanced tx wasm instantiate "$VO_CODE_ID" \
    '{ "contract_name": "Validation Oracle Demo", "bind_name": "vo.sc.pb", "create_request_nhash_fee": "3000" }' \
    --admin "$ADMIN_ACCOUNT" \
    --label validation-oracle-demo \
    --from marketplace-admin \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --gas-prices="1905nhash" \
    --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
    --output json | jq

### Verify that the contract can be queried by code ID
provenanced query wasm list-contract-by-code "$VO_CODE_ID" -t -o json | jq

### Store the address of the contract for convenience
### You'll need to adjust the jq command if you have more than one address returned from the previous command
VO_CONTRACT=$(provenanced query wasm list-contract-by-code "$VO_CODE_ID" -t -o json | jq -r '.contracts[0]')

### Ensure that querying the contract with a valid JSON query works — this should return {"data":null} at this point
provenanced query wasm contract-state smart "$VO_CONTRACT" '{"query_request_order":{"id": ""}}' -t -o json | jq
