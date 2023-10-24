use crate::util::page::Page;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    pub sort: i64,
    pub scope: i64,
    menu_strictly: i64,
    dept_strictly: i64,
    status: i64,
    del_flag: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/**
add
roleId: ''
roleName : "角色名称"
roleKey : "权限字符"
roleSort : 0
menuCheckStrictly : true
deptCheckStrictly : true
status : "0"
remark : "备注"
deptIds : []
menuIds : [1, 100]

upd
roleId : 100
roleName : "角色名称"
roleKey : "权限字符"
roleSort : 0
dataScope : "1"
menuCheckStrictly : true
deptCheckStrictly : true
status : "0"
delFlag : "0"
remark : "备注"
admin : false
deptIds : null
flag : false
menuIds : [1, 100]
permissions : null
createBy : null
createTime : "2023-09-25 14:53:17"
updateBy : null
updateTime : null
 */
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SaveRole {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    pub sort: i64,

    /** 数据范围（1：所有数据权限；2：自定义数据权限；3：本部门数据权限；4：本部门及以下数据权限；5：仅本人数据权限） */
    pub scope: i64,

    /** 菜单树选择项是否关联显示（ 0：父子不互相关联显示 1：父子互相关联显示） */
    pub menu_strictly: i64,

    /** 部门树选择项是否关联显示（0：父子不互相关联显示 1：父子互相关联显示 ） */
    pub dept_strictly: i64,

    /** 角色状态（0正常 1停用） */
    pub status: i64,

    /** 删除标志（0代表存在 1代表删除） */
    pub del_flag: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_ids: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRole {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdRole {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelRole {
    pub ids: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageRole {
    pub page: Page,
}
