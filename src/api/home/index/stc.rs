
use serde::{Serialize};
use crate::{api::sys::menu::stc::Menu, util::{tree::{TreeNode}}};
use sqlx::FromRow;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub admin_info: AdminInfo,
    pub menus: Vec<TreeNode<Menu>>,
    pub site_config: SiteConfig,
    pub terminal: Terminal,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AdminInfo {
    pub avatar: String,
    pub id: String,
    pub last_login_time: String,
    pub nickname: String,
    #[sqlx(rename = "super")]
    pub super_field: bool,
    pub username: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteConfig {
    pub api_url: String,
    pub cdn_url: String,
    pub site_name: String,
    pub upload: Upload,
    pub version: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub maxsize: u32,
    pub mimetype: String,
    pub mode: String,
    pub savename: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Terminal {
    pub install_service_port: String,
    pub npm_package_manager: String,
}
