#!/bin/sh

### 1. Create a valid new entity — as the contract admin
provenanced tx wasm execute "$VO_CONTRACT" \
    "{ \"create_entity\": { \"entity\": { \"address\": \"$VALIDATOR_ACCOUNT\", \"name\": \"Some Validator I guess\" }}}" \
    --amount 3000nhash \
    --fees 382000000nhash \
    --from marketplace-admin \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --broadcast-mode block \
    --yes \
    --testnet \
    --output json | jq

### 2. Examine the new balance of the originator account
provenanced q bank balances "$ORIGINATOR_ACCOUNT" -t -o json | jq

### 3. Query for the entity we just created
provenanced query wasm contract-state smart "$VO_CONTRACT" "{\"query_entity_by_address\":{\"address\": \"$VALIDATOR_ACCOUNT\"}}" -t -o json | jq

### 4. Update the entity — as the entity itself
provenanced tx wasm execute "$VO_CONTRACT" \
    "{ \"update_entity\": { \"entity\": { \"address\": \"$VALIDATOR_ACCOUNT\", \"name\": \"ACME Validation\", \"description\": \"The best validators on the market. Validates Home Equity Loans only.\", \"home_url\": \"www.google.com\" }}}" \
    --amount 3000nhash \
    --fees 382000000nhash \
    --from loan-validator \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --broadcast-mode block \
    --yes \
    --testnet \
    --output json | jq

### 5. Query for it again
provenanced query wasm contract-state smart "$VO_CONTRACT" "{\"query_entity_by_address\":{\"address\": \"$VALIDATOR_ACCOUNT\"}}" -t -o json | jq
