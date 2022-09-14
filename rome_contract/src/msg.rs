use cosmwasm_std::{Uint64, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use desmos_bindings::types::PageRequest;
use desmos_bindings::posts::models::{Entities, RawPostAttachment, ReplySetting, PostReference};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePost{
        post_id: u64,
        external_id: Option<String>,
        tags: Vec<String>,
        text: Option<String>,
        author: Addr,
    },
    EditPost{
        post_id: u64,
        text: String,
        editor: Addr,
    },
    DeletePost{
        post_id: u64,
        signer: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    SubspacePosts{
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    },
    SectionPosts{
        subspace_id: Uint64,
        section_id: u32,
        pagination: Option<PageRequest>,
    },
    Post{
        subspace_id: Uint64,
        post_id: Uint64,
    },
    PostAttachments{
        subspace_id: Uint64,
        post_id: Uint64,
        pagination: Option<PageRequest>,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomResponse {
    val: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
