use cosmwasm_schema::write_api;

use {{crate_name}}::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        // TODO fix QueryMsg not matching expected
        query: QueryMsg,
    }
}
