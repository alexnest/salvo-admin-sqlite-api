use crate::{util::uuid::MyUuid, AppResult};
use super::stc::{AddRoleMenu};
use std::collections::HashSet;

pub async fn sel_menu_ids_by_role_ids<'a, C>(
    conn: C,
    role_ids: &Vec<String>,
) -> AppResult<Vec<String>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT menu_id FROM sys_role_menu WHERE
    "#;

    let or_qry = role_ids
        .iter()
        .map(|id| format!(" role_id = '{}'", id))
        .collect::<Vec<String>>()
        .join(" OR ");

    let qry = format!("{}{}", qry, or_qry);

    let ids = sqlx::query_scalar(&qry).fetch_all(conn).await?;
    Ok(ids)
}

pub async fn ins_role_menu<'a, C>(
    conn: C,
    role_id: &str,
    menu_ids: &HashSet<String>,
) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let items: Vec<AddRoleMenu> = menu_ids
        .iter()
        .map(|menu_id| AddRoleMenu {
            id: MyUuid::new(),
            role_id: role_id.to_string(),
            menu_id: menu_id.to_string(),
        })
        .collect();

    let qry = r#"
        INSERT INTO sys_role_menu (id , role_id, menu_id) 
    "#;

    // it_qry means iterator query
    let bat_ins_qry = items
        .iter()
        .enumerate()
        .map(|(_, item)| {
            format!(
                "SELECT '{}' as id, '{}' as role_id, '{}'as menu_id",
                item.id, item.role_id, item.menu_id
            )
        })
        .collect::<Vec<String>>()
        .join(" UNION ");

    // sta means statement
    let qry = format!("{} {}", qry, bat_ins_qry);

    sqlx::query(&qry).execute(conn).await?;

    Ok(())
}

pub async fn del_by_role_id<'a, C>(conn: C, role_id: &str) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
            DELETE FROM sys_role_menu WHERE role_id = $1
        "#;

    sqlx::query(&qry).bind(role_id).execute(conn).await?;
    Ok(())
}