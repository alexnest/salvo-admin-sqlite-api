use crate::{api::sys::role::stc::Role, util::page::Page};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserWithPwd {
    pub id: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub user_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonenumber: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub del_flag: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub roles: Option<Vec<Role>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserWithRole {
    pub id: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub user_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonenumber: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub del_flag: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    pub roles: Vec<Role>,
}

impl From<&User> for UserWithRole {
    fn from(user: &User) -> Self {
        UserWithRole {
            id: user.id.clone(),
            name: user.name.clone(),
            nick_name: user.nick_name.clone(),
            user_type: user.user_type.clone(),
            dept_id: user.dept_id.clone(),
            email: user.email.clone(),
            phonenumber: user.phonenumber.clone(),
            sex: user.sex,
            avatar: user.avatar.clone(),
            status: user.status,
            del_flag: user.del_flag,
            remark: user.remark.clone(),
            login_date: user.login_date.clone(),
            login_ip: user.login_ip.clone(),
            roles: vec![],
        }
    }
}

/**
add
deptId : 103
email : "pigeve@qq.com"
nickName : "alex"
password : "123456"
phonenumber : "13631800845"
postIds : [1, 2]
roleIds : [100, 2]
*/
#[derive(Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SaveUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub user_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonenumber: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub del_flag: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddUser {
    pub user_name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdUser {
    pub id: String,
    pub user_name: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelUser {
    pub ids: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageUser {
    pub page: Page,
}
