use std::fs::create_dir_all;
use std::path::PathBuf;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use {{crate_name}}::msg::{ExecuteMsg, HelloResponse, InstantiateMsg, QueryMsg};
use {{crate_name}}::state::State;

fn main() {
    let mut out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
    export_schema(&schema_for!(HelloResponse), &out_dir);
}
