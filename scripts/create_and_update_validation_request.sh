#!/bin/sh

### 1. Create a valid request for validation
provenanced tx wasm execute "$VO_CONTRACT" \
    '{ "request_validation": { "request": { "id": "12345", "scopes": ["scope1qqqtl0d4s2y59t5gwhj0mvsmwgxs20h2jc"], "quote": [] }}}' \
    --amount 3000nhash \
    --fees 382000000nhash \
    --from loan-originator \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --broadcast-mode block \
    --yes \
    --testnet \
    --output json | jq

### 2. Examine the new balance of the originator account
provenanced q bank balances "$ORIGINATOR_ACCOUNT" -t -o json | jq

### 3. Query for the request we just created
provenanced query wasm contract-state smart "$VO_CONTRACT" '{"query_validation_request_by_id":{"id": "12345"}}' -t -o json | jq

provenanced query wasm contract-state smart "$VO_CONTRACT" "{\"query_validation_request_by_owner\":{\"owner\": \"$ORIGINATOR_ACCOUNT\"}}" -t -o json | jq

### 4. Update the request we just made
provenanced tx wasm execute "$VO_CONTRACT" \
    '{ "update_validation_request": { "request": { "current_id": "12345", "new_id": "54321", "new_quote": [ { "amount": "200000000", "denom": "nhash" } ] }}}' \
    --amount 3000nhash \
    --fees 382000000nhash \
    --from loan-originator \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --broadcast-mode block \
    --yes \
    --testnet \
    --output json | jq

### 5. Query for the updated request
provenanced query wasm contract-state smart "$VO_CONTRACT" '{"query_validation_request_by_id":{"id": "54321"}}' -t -o json | jq

provenanced query wasm contract-state smart "$VO_CONTRACT" "{\"query_validation_request_by_owner\":{\"owner\": \"$ORIGINATOR_ACCOUNT\"}}" -t -o json | jq

### 6. Verify that the old request ID no longer exists
provenanced query wasm contract-state smart "$VO_CONTRACT" '{"query_validation_request_by_id":{"id": "12345"}}' -t -o json | jq
