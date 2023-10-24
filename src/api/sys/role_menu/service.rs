use super::{
    dao::{ins_role_menu, sel_menu_ids_by_role_ids, del_by_role_id},
    stc::EditRoleMenu,
};
use crate::{get_pool, AppResult, Res};
use salvo::prelude::*;
use std::collections::HashSet;

#[handler]
pub async fn edit_role_menu(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<EditRoleMenu>().await.unwrap();
    let origin_ids = sel_menu_ids_by_role_ids(get_pool(), &param.menu_ids).await?;
    let menu_ids: &HashSet<String> = &param.menu_ids.into_iter().map(|menu_id| menu_id).collect();

    if origin_ids.len() == (&menu_ids).len() {
        let mut origin_ids: HashSet<String> = origin_ids
            .into_iter()
            .map(|menu_id| menu_id)
            .collect();

        origin_ids.retain(|id| !menu_ids.contains(id));
        tracing::info!("ids: {:?}", menu_ids);

        // keep the origin ids as the same
        if origin_ids.len() == 0 {
            Res::suc::<()>().render(res);
            return Ok(());
        }
    }

    let mut tx = get_pool().begin().await?;
    del_by_role_id(&mut tx, &param.role_id).await?;
    ins_role_menu(&mut tx, &param.role_id, menu_ids).await?;
    tx.commit().await?;

    // render
    Res::suc::<()>().render(res);
    Ok(())
}
