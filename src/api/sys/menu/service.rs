use super::{
    dao::{self, ins, upd, sel, sel_items_by_ids},
    stc::{Menu, SaveMenu},
};
use crate::{
    api::sys::{
        role_menu::dao::sel_menu_ids_by_role_ids, user_role::dao::sel_role_ids_by_user_id,
        user::stc::User,
    },
    get_pool,
    global::cst::{CURRENT_USER, ROOT},
    res::error::MyErr,
    util::{level_util::Level, tree::TreeNode},
    AppResult, Res,
};
use salvo::prelude::*;
use std::collections::HashMap;
use time::OffsetDateTime;

#[handler]
pub async fn get(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let id = req.params().get("id").unwrap();
    let info = sel(get_pool(), &id).await?;
    Res::suc::<_>().data(info).render(res);
    Ok(())
}

#[handler]
pub async fn add(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<SaveMenu>().await.unwrap();

    // calculate level
    let mut level = String::from(ROOT);
    if let Some(pid) = &param.pid {
        let parent = sel(get_pool(), &pid).await?;
        parent.map(|parent| {
            level = Level::new(&parent.level, &parent.id).cal();
        });
    }

    // because of the multiple sql statements, we need to use transaction
    let mut tx = get_pool().begin().await?;

    // verify if exists the same name in the same parent id
    let count = dao::cnt_by_name(&mut tx, param.name.clone(), param.pid.clone(), None).await?;
    if count > 0 {
        return MyErr::new().msg("name already exists").build();
    }

    // calculate the pid
    let pid = if let Some(pid) = &param.pid {
        pid.to_string()
    } else {
        String::from(ROOT)
    };

    // calculate the weigh
    let order_num = if let Some(order_num) = &param.order_num {
        *order_num
    } else {
        OffsetDateTime::now_utc().unix_timestamp() as i64
    };

    // insert a data into database
    let id = ins(&mut tx, &param, &level, order_num, &pid).await?;

    // find the record just insert into
    let info = sel(&mut tx, &id).await?;
    tx.commit().await?;

    // rend
    Res::suc::<_>().data(info).render(res);

    Ok(())
}

#[handler]
pub async fn edit(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<SaveMenu>().await.unwrap();

    // get self beofre update
    let before = sel(get_pool(), &param.id.as_ref().unwrap()).await?;
    if let None = before {
        return MyErr::new().msg("data not found").build();
    }

    // get self level before update
    let mut before_level = String::from(ROOT);
    if let Some(before) = &before {
        before_level = Level::new(&before.level, &before.id).cal();
    }

    // get self level length before update
    let before_level_len = &before_level.len();

    // calculate self level and self children level after update
    let mut after_level = String::from(ROOT); // init self level
    let mut children_level = String::from(""); // init self children level
    if let Some(pid) = &param.pid {
        let parent = sel(get_pool(), &pid).await?;
        parent.map(|parent| {
            after_level = Level::new(&parent.level, &parent.id).cal(); // calculate self level after update
            children_level = format!("{}.{}", &after_level, &pid); // calculate self children level after update
        });
    }

    let mut tx = get_pool().begin().await?;

    // verify if exists the same name in the same parent id
    let count = dao::cnt_by_name(
        &mut tx,
        param.name.clone(),
        param.id.clone(),
        param.pid.clone(),
    ).await?;
    if count > 0 {
        return MyErr::new().msg("name already exists").build();
    }

    // update self
    upd(&mut tx, &param, &after_level).await?;

    // get children
    let mut children: Vec<Menu> = dao::sel_children(&mut tx, &before_level).await?;
    // calculate children new_level
    for child in children.iter_mut() {
        child
            .level
            .replace_range(0..*before_level_len, &children_level);
    }

    // update children
    if !&children.is_empty() {
        dao::upd_children(&mut tx, children).await?;
    }

    // commit transaction.note: get self must after commit transaction
    tx.commit().await?;

    // get self after updated
    let info = sel(get_pool(), &param.id.unwrap()).await?;

    // rend
    Res::suc::<_>().data(info).render(res);
    Ok(())
}

#[handler]
pub async fn page(_req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let user = depot.get::<User>(CURRENT_USER).unwrap();
    let menus = get_menu_items_by_user_id(&user.id).await?;
    let map: HashMap<String, Vec<TreeNode<Menu>>> = HashMap::from([("items".to_string(), menus)]);
    Res::suc::<_>().data(map).render(res);
    Ok(())
}

pub async fn get_menu_items_by_user_id(user_id: &str) -> AppResult<Vec<TreeNode<Menu>>> {
    let role_ids = sel_role_ids_by_user_id(get_pool(), user_id).await?;
    let menu_ids = sel_menu_ids_by_role_ids(get_pool(), &role_ids).await?;
    let items = sel_items_by_ids(get_pool(), &menu_ids).await?;
    let tree = TreeNode::build_tree(
        items,
        String::from("0"),
        |i| i.id.clone(),
        |i| i.pid.clone(),
        |i| i.order_num,
    );
    Ok(tree)
}
