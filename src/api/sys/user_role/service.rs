use super::{
    dao::{del_by_role_id, ins_user_role, sel_user_ids_by_role_id},
    stc::EditRoleUsers,
};
use crate::{get_pool, AppResult, Res};
use salvo::prelude::*;
use std::collections::HashSet;

#[handler]
pub async fn edit_role_users(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let param = req.parse_json::<EditRoleUsers>().await.unwrap();
    let origin_ids = sel_user_ids_by_role_id(get_pool(), &param.role_id).await?;
    let user_ids: HashSet<String> = param.user_ids.into_iter().map(|user_id| user_id).collect();

    if origin_ids.len() == (&user_ids).len() {
        let mut origin_ids: HashSet<String> = origin_ids
            .into_iter()
            .map(|user_id| user_id.user_id)
            .collect();

        origin_ids.retain(|item| !user_ids.contains(item));
        tracing::info!("ids: {:?}", user_ids);

        // keep the origin ids as the same
        if origin_ids.len() == 0 {
            Res::suc::<()>().render(res);
            return Ok(());
        }
    }

    let mut tx = get_pool().begin().await?;
    del_by_role_id(&mut tx, &param.role_id).await?;
    // ins_user_role(&mut tx, &param.role_id, user_ids).await?;
    tx.commit().await?;

    // render
    Res::suc::<()>().render(res);
    Ok(())
}
