use super::{
    dao::{self, del, ins, sel, upd},
    stc::{Acl, AddAcl, DelAcl, PageAcl, UpdAcl},
};
use crate::{get_pool, res::error::MyErr, util::page::PageResult, AppResult, Res};
use salvo::prelude::*;

#[handler]
pub async fn page(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let mut tx = get_pool().begin().await?;
    let param = req.parse_json::<PageAcl>().await.unwrap();
    let items = dao::sel_page(&mut tx, &param).await?;
    let total_count = dao::cnt_by_page(&mut tx, &param).await?;
    tx.commit().await?;
    Res::suc::<_>()
        .data(PageResult::<Acl>::new(&param.page, total_count, &items))
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
pub async fn save(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<AddAcl>().await.unwrap();

    // because of the multiple sql statements, we need to use transaction
    let mut tx = get_pool().begin().await?;

    // verify if exists the same name in the same parent id
    let count = dao::cnt_by_name(&mut tx, &param.name, None, &param.acl_module_id).await?;
    if count > 0 {
        return MyErr::new().msg("name already exists").build();
    }

    // insert a data into database
    let id = ins(&mut tx, param).await?;

    // find the record just insert into
    let info = sel(&mut tx, &id).await?;
    tx.commit().await?;

    // rend
    Res::suc::<_>().data(info).render(res);

    Ok(())
}

#[handler]
pub async fn edit(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<UpdAcl>().await.unwrap();

    let item = sel(get_pool(), &param.id).await?;
    if let None = item {
        return MyErr::new().msg("data not found").build();
    }

    let mut tx = get_pool().begin().await?;

    // verify if exists the same name in the same parent id
    let count =
        dao::cnt_by_name(&mut tx, &param.name, Some(&param.id), &param.acl_module_id).await?;
    if count > 0 {
        return MyErr::new().msg("name already exists").build();
    }

    // update self
    upd(&mut tx, &param).await?;

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
    let param = req.parse_json::<DelAcl>().await.unwrap();
    del(get_pool(), param.ids).await?;

    // render
    Res::suc::<()>().render(res);
    Ok(())
}
