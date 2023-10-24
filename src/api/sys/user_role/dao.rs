use super::stc::{AddUserRole, UserId};
use crate::{util::uuid::MyUuid, AppResult};
use sqlx::Sqlite;
use std::collections::HashSet;

pub async fn sel_user_ids_by_role_id<'a, C>(conn: C, id: &str) -> AppResult<Vec<UserId>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT user_id FROM sys_role_user WHERE role_id = $1
    "#;
    let info = sqlx::query_as::<Sqlite, UserId>(&qry)
        .bind(id)
        .fetch_all(conn)
        .await?;
    Ok(info)
}

pub async fn sel_role_ids_by_user_id<'a, C>(conn: C, id: &str) -> AppResult<Vec<String>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT role_id FROM sys_role_user WHERE user_id = $1
    "#;
    let info = sqlx::query_scalar(&qry).bind(id).fetch_all(conn).await?;
    Ok(info)
}

pub async fn ins_user_role<'a, C>(
    conn: C,
    user_id: &str,
    role_ids: &HashSet<String>,
) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let vals: Vec<AddUserRole> = role_ids
        .iter()
        .map(|role_id| AddUserRole {
            id: MyUuid::new(),
            user_id: user_id.to_string(),
            role_id: role_id.to_string(),
        })
        .collect();

    let qry = r#"
        INSERT INTO sys_user_role(id, user_id, role_id) 
    "#;

    // it_qry means iterator query
    let bat_ins_qry = vals
        .iter()
        .enumerate()
        .map(|(index, add_user_role)| {
            let query = format!(
                "SELECT '{}' as id, '{}' as  user_id, '{}'as role_id",
                add_user_role.id, add_user_role.user_id, add_user_role.role_id
            );
            if index < vals.len() - 1 {
                format!("{} UNION", query)
            } else {
                query
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    // sta means statement
    let sta = format!("{} {}", qry, bat_ins_qry);

    sqlx::query(&sta).execute(conn).await?;

    Ok(())
}

pub async fn del_by_role_id<'a, C>(conn: C, role_id: &str) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
            DELETE FROM sys_role_user WHERE role_id = $1
        "#;

    sqlx::query(&qry).bind(role_id).execute(conn).await?;
    Ok(())
}
