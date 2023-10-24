use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub id: String,

    pub pid: String,

    pub level: String,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_frame: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cache: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub menu_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub perms: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    pub order_num: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/*

*/

/**
 *
添加主类目：
id: 0
pid : 0
level: '0'
name: "主类目"
path : "路由地址"
isFrame : "1"
isCache : "0"
menuType : "M"
visible : "0"
status : "0"
icon : "404"
orderNum : 999

添加目录
id: 0
pid: 2008
level: '0'
name: "主目录二级菜单名称"
path : "路由地址"
component : "组件路径"
query : "路由参数"
isFrame : "1"
isCache : "0"
menuType : "C"
visible : "0"
status : "0"
perms : "权限字符"
icon : "404"
orderNum : 999

添加按钮
pid : 2009
level: ''   
name : "菜单名称"
isFrame : "1"
isCache : "0"
menuType : "F"
visible : "0"
status : "0"
perms : "权限字符"
orderNum : 999
 */
#[derive(Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SaveMenu {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_frame: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cache: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub menu_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub perms : Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_num: Option<i64>,

}
