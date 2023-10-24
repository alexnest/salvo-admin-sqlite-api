use super::{
    dao::{self, del, ins, sel, upd},
    stc::{DelRole, PageRole, Role, SaveRole, },
};
use crate::{
    api::sys::role_menu, get_pool, res::error::MyErr, util::page::PageResult, AppResult, Res,
};
use salvo::prelude::*;
use std::collections::HashSet;

#[handler]
pub async fn page(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let mut tx = get_pool().begin().await?;
    let param = req.parse_json::<PageRole>().await.unwrap();
    let items = dao::sel_by_page(&mut tx, &param).await?;
    let total_count = dao::cnt_by_page(&mut tx, &param).await?;
    tx.commit().await?;
    Res::suc::<_>()
        .data(PageResult::<Role>::new(&param.page, total_count, &items))
        .render(res);
    Ok(())
}

#[handler]
pub async fn get(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let id = req.params().get("id").unwrap();
    let info = sel(get_pool(), &id).await?;
    Res::suc::<_>().data(info).render(res);
    Ok(())
}

#[handler]
pub async fn add(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<SaveRole>().await?;

    // verify if exists the same name
    let count = dao::cnt_by_name(get_pool(), &param.name, None).await?;
    if count > 0 {
        return MyErr::new().msg("name already exists").build();
    }

    // begin the transaction
    let mut tx = get_pool().begin().await?;

    // insert data into role table
    let role_id = ins(&mut tx, &param).await?;

    /* insert data into role_menu relationship table */
    if let Some(menu_ids) = param.menu_ids.clone() {
        let menu_ids: &HashSet<String> = &menu_ids.into_iter().map(|menu_id| menu_id).collect();
        role_menu::dao::ins_role_menu(&mut tx, &role_id, menu_ids).await?;
    }

    // commit the transaction
    tx.commit().await?;

    // select the record just insert into
    let info = sel(get_pool(), &role_id).await?;

    Res::suc::<_>().data(info).render(res);
    Ok(())
}

#[handler]
pub async fn edit(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<SaveRole>().await.unwrap();

    // verify the edit record exists
    let item = sel(get_pool(), &param.id.as_ref().unwrap()).await?;
    if let None = item {
        return MyErr::new().msg("data not found").build();
    }

    // start the transaction
    let mut tx = get_pool().begin().await?;

    // verify if exists the same name in the same parent id
    let count = dao::cnt_by_name(&mut tx, &param.name, Some(&param.id.as_ref().unwrap())).await?;
    if count > 0 {
        return MyErr::new().msg("name already exists").build();
    }

    // update self
    upd(&mut tx, &param).await?;

    tx.commit().await?;

    // get self just updated
    let info = sel(get_pool(), &param.id.unwrap().clone()).await?;

    // rend
    Res::suc::<_>().data(info).render(res);

    Ok(())
}

#[handler]
pub async fn save(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<SaveRole>().await.unwrap();

    // because of the multiple sql statements, we need to use transaction
    let mut tx = get_pool().begin().await?;

    // verify if exists the same name in the same parent id
    let count = dao::cnt_by_name(&mut tx, &param.name, None).await?;
    if count > 0 {
        return MyErr::new().msg("name already exists").build();
    }

    // insert a data into database
    let id = ins(&mut tx, &param).await?;

    // find the record just insert into
    let info = sel(&mut tx, &id).await?;
    tx.commit().await?;

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

    // get self
    let info = sel(get_pool(), &id).await?;

    // delete self
    del(get_pool(), vec![id]).await?;

    // render
    Res::suc::<_>().data(info).render(res);
    Ok(())
}

// the handler suitbale for the delete records did not needed to be verified
#[handler]
pub async fn remove_by_ids(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<DelRole>().await.unwrap();
    del(get_pool(), param.ids).await?;

    // render
    Res::suc::<()>().render(res);
    Ok(())
}
