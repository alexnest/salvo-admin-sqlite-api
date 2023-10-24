use crate::{
    api::sys::{menu, role, role_menu, user::{self, stc::UserWithRole}, user_role, acl::stc},
    cache, get_pool,
    global::cst::CURRENT_USER,
    util::jwt::Claims,
    AppResult, Res,
};
use bcrypt;
use salvo::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LoginUser {
    name: String,
    password: String,
    code: String,
    uuid: String,
}

struct UserInfo {
    permissions: Vec<String>,
    roles: Vec<String>,
    user: UserWithRole,
}


#[handler]
pub async fn login(
    req: &mut Request,
    _depot: &mut Depot,
    res: &mut Response,
    _ctrl: &mut FlowCtrl,
) -> AppResult<()> {
    let param = req.parse_json::<LoginUser>().await?;
    let cache = cache::get(&String::from(param.uuid)).await;
    match cache {
        Some(code) => {
            if code.to_string() != param.code {
                Res::fail::<()>().msg("code error").render(res);
                return Ok(());
            }
        }
        None => {
            Res::fail::<()>().msg("code invalid").render(res);
            return Ok(());
        }
    }
    // tracing::info!("user: {:?}", param);
    let info = user::dao::sel_by_name(get_pool(), &param.name).await?;
    if let None = info {
        Res::fail::<()>().msg("user dose not exist").render(res);
        return Ok(());
    }

    if let Some(info) = info {
        let is_ok = bcrypt::verify(&param.password, &info.password)?;
        if !is_ok {
            Res::fail::<()>()
                .msg("user name or password error")
                .render(res);
            return Ok(());
        }

        let jwt = Claims::new(&info.id.as_str()).encode();
        Res::suc::<String>().data(jwt).render(res);
    }

    Ok(())
}

// get user info
/*
{
    "msg": "操作成功",
    "code": 200,
    "permissions": [
        "*:*:*"
    ],
    "roles": [
        "admin"
    ],
    "user": {
        "createBy": "admin",
        "createTime": "2023-07-17 11:39:50",
        "updateBy": null,
        "updateTime": null,
        "remark": "管理员",
        "userId": 1,
        "deptId": 103,
        "userName": "admin",
        "nickName": "若依",
        "email": "ry@163.com",
        "phonenumber": "15888888888",
        "sex": "1",
        "avatar": "",
        "password": "$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE8ByOhJIrdAu2",
        "status": "0",
        "delFlag": "0",
        "loginIp": "127.0.0.1",
        "loginDate": "2023-09-21T10:26:55.000+08:00",
        "dept": {
            "createBy": null,
            "createTime": null,
            "updateBy": null,
            "updateTime": null,
            "remark": null,
            "deptId": 103,
            "parentId": 101,
            "ancestors": "0,100,101",
            "deptName": "研发部门",
            "orderNum": 1,
            "leader": "若依",
            "phone": null,
            "email": null,
            "status": "0",
            "delFlag": null,
            "parentName": null,
            "children": []
        },
        "roles": [
            {
                "createBy": null,
                "createTime": null,
                "updateBy": null,
                "updateTime": null,
                "remark": null,
                "roleId": 1,
                "roleName": "超级管理员",
                "roleKey": "admin",
                "roleSort": 1,
                "dataScope": "1",
                "menuCheckStrictly": false,
                "deptCheckStrictly": false,
                "status": "0",
                "delFlag": null,
                "flag": false,
                "menuIds": null,
                "deptIds": null,
                "permissions": null,
                "admin": true
            }
        ],
        "roleIds": null,
        "postIds": null,
        "roleId": null,
        "admin": true
    }
}
 */
#[handler]
pub async fn get_info(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    _ctrl: &mut FlowCtrl,
) -> AppResult<()> {
    // get user base info
    let user = depot.get::<user::stc::User>(CURRENT_USER).unwrap();

    // get role ids according to the user id
    let role_ids = user_role::dao::sel_role_ids_by_user_id(get_pool(), &user.id).await?;

    // get roles according to the role ids
    let roles = role::dao::sel_by_ids(get_pool(), &role_ids).await?;

    // get role key
    let role_keys: Vec<String> = roles.iter().map(|r| r.key.as_ref().unwrap().clone()).collect();

    // init permissions
    let mut permissions = vec!["*:*:*".to_string()];

    // verify the user is admin or not
    let mut isAdmin = false;
    for role in roles.iter() {
        if role.key.as_ref().unwrap() == "admin" {
            isAdmin = true;
            break;
        }
    }

    // if the user is not admin , get permissions according to the role ids
    if (!isAdmin) {
        permissions = role_menu::dao::sel_menu_ids_by_role_ids(get_pool(), &role_ids).await?;
    }

    // build user with role
    let mut user_with_role = UserWithRole::from(user);
    user_with_role.roles = roles;

    // build user info
    // let UserInfo = {
    //     permissions: permissions ,
    //     roles: role_keys,
    //     user: 
    // }

    Ok(())
}

// get user routers
/**
 *{
    "msg": "操作成功",
    "code": 200,
    "data": [
        {
            "name": "System",
            "path": "/system",
            "hidden": false,
            "redirect": "noRedirect",
            "component": "Layout",
            "alwaysShow": true,
            "meta": {
                "title": "系统管理",
                "icon": "system",
                "noCache": false,
                "link": null
            },
            "children": [
                {
                    "name": "User",
                    "path": "user",
                    "hidden": false,
                    "component": "system/user/index",
                    "meta": {
                        "title": "用户管理",
                        "icon": "user",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Role",
                    "path": "role",
                    "hidden": false,
                    "component": "system/role/index",
                    "meta": {
                        "title": "角色管理",
                        "icon": "peoples",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Menu",
                    "path": "menu",
                    "hidden": false,
                    "component": "system/menu/index",
                    "meta": {
                        "title": "菜单管理",
                        "icon": "tree-table",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Dept",
                    "path": "dept",
                    "hidden": false,
                    "component": "system/dept/index",
                    "meta": {
                        "title": "部门管理",
                        "icon": "tree",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Post",
                    "path": "post",
                    "hidden": false,
                    "component": "system/post/index",
                    "meta": {
                        "title": "岗位管理",
                        "icon": "post",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Dict",
                    "path": "dict",
                    "hidden": false,
                    "component": "system/dict/index",
                    "meta": {
                        "title": "字典管理",
                        "icon": "dict",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Config",
                    "path": "config",
                    "hidden": false,
                    "component": "system/config/index",
                    "meta": {
                        "title": "参数设置",
                        "icon": "edit",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Notice",
                    "path": "notice",
                    "hidden": false,
                    "component": "system/notice/index",
                    "meta": {
                        "title": "通知公告",
                        "icon": "message",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Log",
                    "path": "log",
                    "hidden": false,
                    "redirect": "noRedirect",
                    "component": "ParentView",
                    "alwaysShow": true,
                    "meta": {
                        "title": "日志管理",
                        "icon": "log",
                        "noCache": false,
                        "link": null
                    },
                    "children": [
                        {
                            "name": "Operlog",
                            "path": "operlog",
                            "hidden": false,
                            "component": "monitor/operlog/index",
                            "meta": {
                                "title": "操作日志",
                                "icon": "form",
                                "noCache": false,
                                "link": null
                            }
                        },
                        {
                            "name": "Logininfor",
                            "path": "logininfor",
                            "hidden": false,
                            "component": "monitor/logininfor/index",
                            "meta": {
                                "title": "登录日志",
                                "icon": "logininfor",
                                "noCache": false,
                                "link": null
                            }
                        }
                    ]
                }
            ]
        },
        {
            "name": "Org",
            "path": "/org",
            "hidden": false,
            "redirect": "noRedirect",
            "component": "Layout",
            "alwaysShow": true,
            "meta": {
                "title": "组织架构",
                "icon": "education",
                "noCache": false,
                "link": null
            },
            "children": [
                {
                    "name": "School",
                    "path": "school",
                    "hidden": false,
                    "component": "org/school/index",
                    "meta": {
                        "title": "学校管理",
                        "icon": "#",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Grade",
                    "path": "grade",
                    "hidden": false,
                    "component": "org/grade/index",
                    "meta": {
                        "title": "班级管理",
                        "icon": "#",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Teacher",
                    "path": "teacher",
                    "hidden": false,
                    "component": "org/teacher/index",
                    "meta": {
                        "title": "教师管理",
                        "icon": "#",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Student",
                    "path": "student",
                    "hidden": false,
                    "component": "org/student/index",
                    "meta": {
                        "title": "学生管理",
                        "icon": "#",
                        "noCache": false,
                        "link": null
                    }
                }
            ]
        },
        {
            "name": "Monitor",
            "path": "/monitor",
            "hidden": false,
            "redirect": "noRedirect",
            "component": "Layout",
            "alwaysShow": true,
            "meta": {
                "title": "系统监控",
                "icon": "monitor",
                "noCache": false,
                "link": null
            },
            "children": [
                {
                    "name": "Online",
                    "path": "online",
                    "hidden": false,
                    "component": "monitor/online/index",
                    "meta": {
                        "title": "在线用户",
                        "icon": "online",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Job",
                    "path": "job",
                    "hidden": false,
                    "component": "monitor/job/index",
                    "meta": {
                        "title": "定时任务",
                        "icon": "job",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Druid",
                    "path": "druid",
                    "hidden": false,
                    "component": "monitor/druid/index",
                    "meta": {
                        "title": "数据监控",
                        "icon": "druid",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Server",
                    "path": "server",
                    "hidden": false,
                    "component": "monitor/server/index",
                    "meta": {
                        "title": "服务监控",
                        "icon": "server",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Cache",
                    "path": "cache",
                    "hidden": false,
                    "component": "monitor/cache/index",
                    "meta": {
                        "title": "缓存监控",
                        "icon": "redis",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "CacheList",
                    "path": "cacheList",
                    "hidden": false,
                    "component": "monitor/cache/list",
                    "meta": {
                        "title": "缓存列表",
                        "icon": "redis-list",
                        "noCache": false,
                        "link": null
                    }
                }
            ]
        },
        {
            "name": "Tool",
            "path": "/tool",
            "hidden": false,
            "redirect": "noRedirect",
            "component": "Layout",
            "alwaysShow": true,
            "meta": {
                "title": "系统工具",
                "icon": "tool",
                "noCache": false,
                "link": null
            },
            "children": [
                {
                    "name": "Build",
                    "path": "build",
                    "hidden": false,
                    "component": "tool/build/index",
                    "meta": {
                        "title": "表单构建",
                        "icon": "build",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Gen",
                    "path": "gen",
                    "hidden": false,
                    "component": "tool/gen/index",
                    "meta": {
                        "title": "代码生成",
                        "icon": "code",
                        "noCache": false,
                        "link": null
                    }
                },
                {
                    "name": "Swagger",
                    "path": "swagger",
                    "hidden": false,
                    "component": "tool/swagger/index",
                    "meta": {
                        "title": "系统接口",
                        "icon": "swagger",
                        "noCache": false,
                        "link": null
                    }
                }
            ]
        },
        {
            "name": "Http://ruoyi.vip",
            "path": "http://ruoyi.vip",
            "hidden": false,
            "component": "Layout",
            "meta": {
                "title": "若依官网",
                "icon": "guide",
                "noCache": false,
                "link": "http://ruoyi.vip"
            }
        }
    ]
}
 */
#[handler]
pub async fn get_routers() {}
