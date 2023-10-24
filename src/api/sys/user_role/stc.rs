use serde::Deserialize;
use sqlx::FromRow;

#[derive(Deserialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserId {
    pub user_id: String,
}

#[derive(Deserialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoleId {
    pub role_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeUserIds {
    pub ids: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditRoleUsers {
    pub role_id: String,
    pub user_ids: Vec<String>,
}

pub struct AddUserRole {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
}
