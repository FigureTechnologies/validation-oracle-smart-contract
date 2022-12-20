#!/bin/sh

### 1. Create a valid new validation definition
provenanced tx wasm execute "$VO_CONTRACT" \
    '{ "create_validation_definition": { "request": { "validation_type": "lauramachelocfull", "display_name": "Laura Mac - Full HELOC", "enabled": true, "bind_name": true }}}' \
    --from marketplace-admin \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --broadcast-mode block \
    --gas auto \
    --gas-prices="1905nhash" \
    --gas-adjustment=1.5 \
    --yes \
    --testnet \
    --output json | jq

### 2. Query for the definition we just created
provenanced query wasm contract-state smart "$VO_CONTRACT" '{"query_validation_definition_by_type":{"type": "lauramachelocfull"}}' -t -o json | jq

### 3. Update the validation definition we just created
provenanced tx wasm execute "$VO_CONTRACT" \
    '{ "create_validation_definition": { "request": { "validation_type": "lauramachelocfull", "display_name": "Laura Mac - Full HELOC", "enabled": true, "bind_name": true }}}' \
    --from marketplace-admin \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --broadcast-mode block \
    --gas auto \
    --gas-prices="1905nhash" \
    --gas-adjustment=1.5 \
    --yes \
    --testnet \
    --output json | jq

### 2. Query for the updated definition
provenanced query wasm contract-state smart "$VO_CONTRACT" '{"query_validation_definition_by_type":{"type": "lauramachelocfull"}}' -t -o json | jq
