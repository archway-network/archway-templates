use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Dummy {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))]
#[serde(rename_all = "snake_case")]
#[derive(cosmwasm_schema::QueryResponses)]
pub enum QueryMsg {
    #[returns(HelloResponse)]
    Hello {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HelloResponse {
    pub msg: String,
}
