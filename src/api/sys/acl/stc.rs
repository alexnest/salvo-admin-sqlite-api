use crate::util::page::Page;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Acl {
    pub id: String,
    pub name: String,
    pub acl_module_id: String,
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_time: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAcl {
    pub name: String,
    pub acl_module_id: String,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdAcl {
    pub id: String,
    pub name: String,
    pub acl_module_id: String,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelAcl {
    pub ids: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageAcl {
    pub acl_module_id: String,
    pub page: Page,
}

