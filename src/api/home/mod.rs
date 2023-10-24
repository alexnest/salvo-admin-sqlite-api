pub mod captcha;
pub mod dashboard;
pub mod index;
pub mod login;

use crate::middleware::auth::Auth;
use crate::middleware::size::MaxSize;
use salvo::Router;

pub struct Routes;

impl Routes {
    pub fn new() -> Router {
        let limit_size = MaxSize(1024);
        Router::new()
            .hoop(limit_size)
            .push(Router::with_path("captcha").get(captcha::get_captcha))
            .push(Router::with_path("login").post(login::login))
            .push(
                Router::with_path("home")
                    .hoop(Auth)
                    .push(Router::with_path("getInfo").get(login::get_info))
                    // .push(Router::with_path("login").post(login::login))
                    .push(Router::with_path("index").get(index::service::index))
                    .push(Router::with_path("dashboard").get(dashboard::service::dashboard)),
            )
    }
}
