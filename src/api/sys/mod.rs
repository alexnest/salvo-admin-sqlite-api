pub mod acl;
pub mod acl_module;
pub mod menu;
pub mod role;
pub mod role_acl;
pub mod role_menu;
pub mod user_role;
pub mod user;

use crate::middleware::auth::Auth;
use crate::middleware::size::MaxSize;
use salvo::Router;

pub struct Routes;

impl Routes {
    pub fn new() -> Router {
        let limit_size = MaxSize(2048);
        Router::with_path("sys")
            .hoop(limit_size)
            // .hoop(Auth)
            .push(
                Router::with_path("aclModule")
                    .push(Router::with_path("get").get(acl_module::service::get))
                    .push(Router::with_path("save").post(acl_module::service::save))
                    .push(Router::with_path("edit").post(acl_module::service::edit))
                    .push(Router::with_path("remove/<id>").delete(acl_module::service::remove))
                    .push(
                        Router::with_path("remove_by_ids").post(acl_module::service::remove_by_ids),
                    ),
            )
            .push(
                Router::with_path("acl")
                    .push(Router::with_path("page").get(acl::service::page))
                    .push(Router::with_path("get").get(acl::service::get))
                    .push(Router::with_path("save").post(acl::service::save))
                    .push(Router::with_path("edit").post(acl::service::edit))
                    .push(Router::with_path("remove/<id>").delete(acl::service::remove))
                    .push(Router::with_path("remove_by_ids").post(acl::service::remove_by_ids)),
            )
            .push(
                Router::with_path("user")
                    .push(Router::with_path("page").get(user::service::page))
                    .push(Router::with_path("get").get(user::service::get))
                    .push(Router::with_path("add").post(user::service::add))
                    .push(Router::with_path("edit").post(user::service::edit))
                    .push(Router::with_path("remove/<id>").delete(user::service::remove))
                    .push(Router::with_path("remove_by_ids").post(user::service::remove_by_ids)),
            )
            .push(
                Router::with_path("role")
                    .push(Router::with_path("page").get(role::service::page))
                    .push(Router::with_path("get").get(role::service::get))
                    .push(Router::with_path("add").post(role::service::add))
                    .push(Router::with_path("save").post(role::service::save))
                    .push(Router::with_path("edit").post(role::service::edit))
                    .push(Router::with_path("remove/<id>").delete(role::service::remove))
                    .push(Router::with_path("remove_by_ids").post(role::service::remove_by_ids)),
            )
            .push(
                Router::with_path("roleuser")
                    .push(Router::with_path("edit").post(user_role::service::edit_role_users)),
            )
            .push(
                Router::with_path("rolemenu")
                    .push(Router::with_path("edit").post(role_menu::service::edit_role_menu)),
            )
            .push(
                Router::with_path("menu")
                    .push(Router::with_path("index").get(menu::service::page))
                    .push(Router::with_path("add").post(menu::service::add)),
            )
    }
}
