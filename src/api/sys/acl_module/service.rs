use super::{
    dao::{self, del, ins, sel, upd},
    stc::{AclModule, AddAclModule, DelAclModule, UpdAclModule},
};
use crate::{
    get_pool, global::cst::ROOT, res::error::MyErr, util::level_util::Level, AppResult, Res,
};
use salvo::prelude::*;

#[handler]
pub async fn get(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let id = req.params().get("id").unwrap();
    let info = sel(get_pool(), &id).await?;
    Res::suc::<_>().data(info).render(res);
    Ok(())
}

#[handler]
pub async fn save(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<AddAclModule>().await.unwrap();

    // calculate level
    let mut level = String::from(ROOT);
    if let Some(parent_id) = &param.parent_id {
        let parent = sel(get_pool(), &parent_id).await?;
        parent.map(|parent| {
            level = Level::new(&parent.level, &parent.id).cal();
        });
    }

    // because of the multiple sql statements, we need to use transaction
    let mut tx = get_pool().begin().await?;

    // verify if exists the same name in the same parent id
    let count = dao::cnt_by_name(&mut tx, &param.name, None, param.parent_id.as_deref()).await?;
    if count > 0 {
        return MyErr::new().msg("name already exists").build();

        // you can also use the following code to return the error
        // Res::fail::<()>().msg("name already exists").render(res);
        // return Ok(());
    }

    // insert a data into database
    let id = ins(&mut tx, &param, &level).await?;

    // find the record just insert into
    let info = sel(&mut tx, &id).await?;

    // rend
    Res::suc::<_>().data(info).render(res);

    tx.commit().await?;
    Ok(())
}

#[handler]
pub async fn edit(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<UpdAclModule>().await.unwrap();

    let before = sel(get_pool(), &param.id).await?;
    if let None = before {
        return MyErr::new().msg("data not found").build();
    }

    // get before level
    let mut before_level = String::from(ROOT);
    before.map(|before| {
        before_level = Level::new(&before.level, &before.id).cal();
    });
    let before_level_len = &before_level.len();

    // get self after level and children after level
    let mut after_level = String::from(ROOT);
    let mut child_level = String::from("");
    if let Some(parent_id) = &param.parent_id {
        let parent = sel(get_pool(), &parent_id).await?;
        parent.map(|parent| {
            after_level = Level::new(&parent.level, &parent.id).cal();
            child_level = format!("{}.{}", &after_level, &param.id);
        });
    }

    let mut tx = get_pool().begin().await?;

    // verify if exists the same name in the same parent id
    let count = dao::cnt_by_name(
        &mut tx,
        &param.name,
        Some(&param.id),
        param.parent_id.as_deref(),
    ).await?;
    if count > 0 {
        return MyErr::new().msg("name already exists").build();
    }

    // update self
    upd(&mut tx, &param, &after_level).await?;

    // get children
    let mut children: Vec<AclModule> = dao::sel_children(&mut tx, &before_level).await?;
    // calculate children new_level
    for child in children.iter_mut() {
        child.level.replace_range(0..*before_level_len, &child_level);
    }

    // update children
    if !&children.is_empty() {
        dao::upd_children(&mut tx, children).await?;
    }

    // commit transaction.note: get self must after commit transaction
    tx.commit().await?;

    // get self just updated
    let info = sel(get_pool(), &param.id).await?;

    // rend
    Res::suc::<_>().data(info).render(res);
    Ok(())
}

// the handler suitable for the delete recrod needed to be verified
#[handler]
pub async fn remove(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let id = req.param::<String>("id").unwrap();

    // the item did not found according to the id
    let item = sel(get_pool(), &id).await?;
    if let None = item {
        return MyErr::new().msg("data not found").build();
    }

    // the item has children and not allow to remove
    let count = dao::cnt_by_parent_id(get_pool(), &id).await?;
    if count > 0 {
        return MyErr::new().msg("has children").build();
    }

    // get self
    let info = sel(get_pool(), &id).await?;

    // delete self
    del(get_pool(), &vec![id]).await?;

    // render
    Res::suc::<_>().data(info).render(res);
    Ok(())
}

// the handler suitbale for the delete records did not needed to be verified
#[handler]
pub async fn remove_by_ids(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<DelAclModule>().await.unwrap();
    del(get_pool(), &param.ids).await?;
    Res::suc::<()>().render(res);
    Ok(())
}
