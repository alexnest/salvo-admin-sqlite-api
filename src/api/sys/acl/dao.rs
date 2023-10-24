use super::stc::{Acl, AddAcl, PageAcl, UpdAcl};
use crate::{
    util::{date::DateUtil, uuid::MyUuid},
    AppResult,
};
use sqlx::Sqlite;

pub async fn sel<'a, C>(conn: C, id: &str) -> AppResult<Option<Acl>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT * FROM sys_acl WHERE id = $1
    "#;
    let info = sqlx::query_as::<Sqlite, Acl>(&qry)
        .bind(id)
        .fetch_optional(conn)
        .await?;
    Ok(info)
}

pub async fn sel_page<'a, C>(conn: C, param: &PageAcl) -> AppResult<Vec<Acl>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT * FROM sys_acl WHERE acl_module_id = $1 ORDER BY seq DESC LIMIT $2 OFFSET $3
    "#;
    let page = sqlx::query_as::<Sqlite, Acl>(&qry)
        .bind(&param.acl_module_id)
        .bind(param.page.page_size)
        .bind(param.page.offset())
        .fetch_all(conn)
        .await?;
    Ok(page)
}

pub async fn cnt_by_page<'a, C>(conn: C, param: &PageAcl) -> AppResult<u32>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!("SELECT COUNT(1) FROM sys_acl WHERE acl_module_id = $1",);

    let count = sqlx::query_scalar(&qry)
        .bind(&param.acl_module_id)
        .fetch_one(conn)
        .await?;

    Ok(count)
}

pub async fn ins<'a, C>(conn: C, param: AddAcl) -> AppResult<String>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let id = MyUuid::new();
    let now = DateUtil::get_current_datetime();

    let qry = r#"
        INSERT INTO sys_acl (id , name , acl_module_id , url , add_time) VALUES($1,$2,$3,$4,$5) returning id
        "#;
    // "INSERT INTO sys_acl (id , name , acl_module_id , add_time) VALUES($1,$2,$3,$5) returning id";
    let id = sqlx::query_scalar(&qry)
        .bind(id)
        .bind(&param.name)
        .bind(&param.acl_module_id)
        .bind(&param.url)
        .bind(&now)
        .fetch_one(conn)
        .await?;

    Ok(id)
}

pub async fn upd<'a, C>(conn: C, param: &UpdAcl) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let now = DateUtil::get_current_datetime();

    let qry = r#"
        UPDATE sys_acl SET 
            name = $1, 
            acl_module_id= $2,
            url = $3,
            update_time = $4 
        WHERE id = $5
        "#;
    sqlx::query(&qry)
        .bind(&param.name)
        .bind(&param.acl_module_id)
        .bind(&param.url)
        .bind(now)
        .bind(&param.id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn del<'a, C>(conn: C, ids: Vec<String>) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!(
        r#"
            DELETE FROM sys_acl WHERE {}
        "#,
        ids.iter()
            .map(|id| format!("id = '{}'", id))
            .collect::<Vec<String>>()
            .join(" OR ")
    );

    sqlx::query(&qry).execute(conn).await?;
    Ok(())
}

pub async fn cnt_by_name<'a, C>(
    conn: C,
    name: &str,
    id: Option<&str>,
    acl_module_id: &str,
) -> AppResult<u32>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!(
        r#"
            SELECT COUNT(1) 
            FROM sys_acl 
            WHERE name = '{}'  
            AND  acl_module_id= '{}' 
        "#,
        name, acl_module_id
    );

    let cod_id = if let Some(id) = id {
        format!(" AND id != '{}' ", id)
    } else {
        "".to_string()
    };

    let count_sta = format!("{}{}", qry, cod_id);
    let count: u32 = sqlx::query_scalar(&count_sta.as_str())
        .fetch_one(conn)
        .await?;

    Ok(count)
}
