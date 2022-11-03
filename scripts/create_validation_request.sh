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
provenanced query wasm contract-state smart "$VO_CONTRACT" '{"query_request_order":{"id": "12345"}}' -t -o json | jq
