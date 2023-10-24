
use salvo::prelude::*;

use super::stc::{Index, SiteConfig, Terminal, Upload, AdminInfo};

use crate::{
    api::{
        sys::{menu::service::get_menu_items_by_user_id, user::stc::User},
    },
    global::cst::CURRENT_USER,
    AppResult, Res,
};


#[handler]
// TODO: 这个分层太乱了，需要重构, 重构方式：前端根据不同模块进行请求，这个方法放到一个叫home的模块中
pub async fn index(_: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let user = depot.get::<User>(CURRENT_USER).unwrap();
    tracing::info!("user:{:?}", user);
    let menus = get_menu_items_by_user_id(&user.id).await?;
    let admin_info = AdminInfo {
        avatar: "http://localhost:8000/static/images/avatar.png".to_string(),
        id: user.id.clone(),
        last_login_time: "2023-06-20 08:50:11".to_string(),
        nickname: user.nick_name.clone().unwrap(),
        super_field: true,
        username: user.name.clone(),
    };
    let site_config = SiteConfig {
        api_url: "https://buildadmin.com".to_string(),
        cdn_url: "http://localhost:8000".to_string(),
        site_name: "BuildAdmin".to_string(),
        version: "1.0.0".to_string(),
        upload: Upload {
            maxsize: 10485760,
            mimetype: "jpg,png,bmp,jpeg,gif,webp,zip,rar,xls,xlsx,doc,docx,wav,mp4,mp3,txt"
                .to_string(),
            mode: "local".to_string(),
            savename: "/storage/{topic}/{year}{mon}{day}/{filesha1}{.suffix}".to_string(),
        },
    };
    let terminal = Terminal {
        install_service_port: "".to_string(),
        npm_package_manager: "".to_string(),
    };
    let home = Index {
        admin_info,
        menus,
        site_config,
        terminal,
    };
    Res::suc::<_>().data(home).render(res);
    Ok(())
}
