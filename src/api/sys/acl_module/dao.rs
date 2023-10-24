use super::stc::{AclModule, AddAclModule, UpdAclModule};
use crate::{
    util::{date::DateUtil, uuid::MyUuid},
    AppResult,
};
use sqlx::Sqlite;

pub async fn sel<'a, C>(conn: C, id: &str) -> AppResult<Option<AclModule>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT * FROM sys_acl_module WHERE id = $1
    "#;
    let info = sqlx::query_as::<Sqlite, AclModule>(&qry)
        .bind(id)
        .fetch_optional(conn)
        .await?;
    Ok(info)
}

pub async fn ins<'a, C>(conn: C, param: &AddAclModule, level: &str) -> AppResult<String>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let id = MyUuid::new();
    let now = DateUtil::get_current_datetime();

    let qry = r#"
        INSERT INTO sys_acl_module (id,name, parent_id,level, add_time) VALUES($1,$2,$3,$4,$5) returning id
    "#;
    let id = sqlx::query_scalar(&qry)
        .bind(id)
        .bind(&param.name)
        .bind(&param.parent_id)
        .bind(&level)
        .bind(&now)
        .fetch_one(conn)
        .await?;

    Ok(id)
}

pub async fn upd<'a, C>(conn: C, param: &UpdAclModule, level: &str) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let now = DateUtil::get_current_datetime();

    let qry = r#"
        UPDATE sys_acl_module SET name = $1, parent_id = $2,level = $3,update_time = $4 WHERE id = $5
    "#;
    sqlx::query(&qry)
        .bind(&param.name)
        .bind(&param.parent_id)
        .bind(&level)
        .bind(now)
        .bind(&param.id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn del<'a, C>(conn: C, ids: &Vec<String>) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!(
        r#"DELETE FROM sys_acl_module WHERE {}"#,
        ids.iter()
            .map(|id| format!("id = '{}'", id))
            .collect::<Vec<String>>()
            .join(" OR ")
    );

    sqlx::query(&qry).execute(conn).await?;
    Ok(())
}

pub async fn sel_children<'a, C>(conn: C, level: &str) -> AppResult<Vec<AclModule>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let children: Vec<AclModule> = sqlx::query_as::<_, AclModule>(
        r#"
            SELECT *
            FROM sys_acl_module 
            WHERE level = ? or level like ?
        "#,
    )
    .bind(level.to_string())
    .bind(level.to_string() + ".%")
    .fetch_all(conn)
    .await?;
    Ok(children)
}

// here use case..when expression to improve database execution efficiency (use for loop will execute multiple sql statements)
pub async fn upd_children<'a, C>(conn: C, children: Vec<AclModule>) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let mut upd_casewhen = String::from("");
    let mut upd_condition = String::from("");
    let upd_qry = String::from(" UPDATE sys_acl_module SET level = CASE id \n ");
    for child in &children {
        upd_casewhen = format!(
            "{} {}",
            upd_casewhen,
            format!(" WHEN '{}' THEN '{}' \n ", &child.id, &child.level)
        );
    }
    let upd_end = format!(" END \n WHERE ");
    for (idx, child) in children.iter().enumerate() {
        if idx == 0 {
            upd_condition = format!(" id = '{}' \n ", &child.id);
        } else {
            upd_condition = format!(
                "{}{}",
                upd_condition,
                format!(" OR id = '{}' \n ", &child.id)
            );
        }
    }

    let upd_sta = format!("{}{}{}{}", upd_qry, upd_casewhen, upd_end, upd_condition);
    sqlx::query(upd_sta.as_str()).execute(conn).await?;

    Ok(())
}

pub async fn cnt_by_name<'a, C>(
    conn: C,
    name: &str,
    id: Option<&str>,
    parent_id: Option<&str>,
) -> AppResult<u32>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!(
        "SELECT COUNT(1) FROM sys_acl_module WHERE name = '{}' ",
        name
    );

    let cod_parent_id = if let Some(parent_id) = parent_id {
        format!(" AND parent_id = '{}' ", parent_id)
    } else {
        format!(" AND parent_id IS NULL ")
    };

    let cod_id = if let Some(id) = id {
        format!(" AND id != '{}' ", id)
    } else {
        "".to_string()
    };

    let count_sta = format!("{}{}{}", qry, cod_parent_id, cod_id);
    let count: u32 = sqlx::query_scalar(&count_sta.as_str())
        .fetch_one(conn)
        .await?;

    Ok(count)
}

pub async fn cnt_by_parent_id<'a, C>(conn: C, parent_id: &str) -> AppResult<u32>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!("SELECT COUNT(1) FROM sys_acl_module WHERE parent_id =  ?",);

    let count = sqlx::query_scalar(&qry)
        .bind(parent_id)
        .fetch_one(conn)
        .await?;

    Ok(count)
}
