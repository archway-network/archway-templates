use cosmwasm_schema::write_api;

use e_scrow_template_copy::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        // TODO fix QueryMsg not matching expected
        query: QueryMsg,
    }
}
