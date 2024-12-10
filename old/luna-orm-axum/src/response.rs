use luna_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PostResponse<T>
where
    T: Serialize + Entity + Send + Sync,
{
    #[serde(rename = "create")]
    Create { entity: T },
    #[serde(rename = "insert")]
    Insert,
    #[serde(rename = "upsert")]
    Upsert,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PutResponse {
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "change")]
    Change { count: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum DeleteResponse<SE>
where
    SE: Serialize + SelectedEntity,
{
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "remove")]
    Remove { entity: Option<SE> },
    #[serde(rename = "purify")]
    Purify { count: usize },
}
