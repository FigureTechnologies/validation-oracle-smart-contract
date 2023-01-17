use validation_oracle_smart_contract::types::core::msg::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg,
};

use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
        migrate: MigrateMsg,
    }
}
