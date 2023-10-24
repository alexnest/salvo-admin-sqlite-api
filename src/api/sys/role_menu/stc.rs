use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditRoleMenu {
    pub role_id: String,
    pub menu_ids: Vec<String>,
}

pub struct AddRoleMenu {
    pub id: String,
    pub role_id: String,
    pub menu_id: String,
}
