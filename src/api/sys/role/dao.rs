use super::stc::{PageRole, Role, SaveRole};
use crate::{
    util::{date::DateUtil, uuid::MyUuid},
    AppResult,
};
use sqlx::Sqlite;

pub async fn sel<'a, C>(conn: C, id: &str) -> AppResult<Option<Role>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT * FROM sys_role WHERE id = $1
    "#;
    let info = sqlx::query_as::<Sqlite, Role>(&qry)
        .bind(id)
        .fetch_optional(conn)
        .await?;
    Ok(info)
}

pub async fn sel_by_ids<'a, C>(conn: C, ids: &Vec<String>) -> AppResult<Vec<Role>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!(
        r#"
            SELECT * FROM sys_role WHERE {}
        "#,
        ids.iter()
            .map(|id| format!("id = '{}'", id))
            .collect::<Vec<String>>()
            .join(" OR ")
    );
    let items = sqlx::query_as::<Sqlite, Role>(&qry)
        .fetch_all(conn)
        .await?;
    Ok(items)
}

pub async fn sel_by_page<'a, C>(conn: C, param: &PageRole) -> AppResult<Vec<Role>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT * FROM sys_role ORDER BY update_time DESC 
    "#;
    let page = sqlx::query_as::<Sqlite, Role>(&qry)
        .bind(param.page.page_size)
        .bind(param.page.offset())
        .fetch_all(conn)
        .await?;
    Ok(page)
}

pub async fn cnt_by_page<'a, C>(conn: C, _param: &PageRole) -> AppResult<u32>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!("SELECT COUNT(1) FROM sys_role",);

    let count = sqlx::query_scalar(&qry).fetch_one(conn).await?;

    Ok(count)
}

pub async fn ins<'a, C>(conn: C, param: &SaveRole) -> AppResult<String>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let id = MyUuid::new();
    let now = DateUtil::get_current_common_datetime();

    let qry = r#"
        INSERT INTO sys_role (
          id, name, key, sort,  scope,
          menu_strictly, dept_strictly, status, del_flag, remark,                                  
          create_by, create_time, update_by, update_time                        
        ) 
        VALUES(
          $1,$2,$3,$4,$5,
          $6,$7,$9,$9,$10,
          $11,$12,$13,$14
        ) returning id
        "#;
    let id = sqlx::query_scalar(&qry)
        .bind(id)
        .bind(&param.name)
        .bind(&param.key)
        .bind(&param.sort)
        .bind(&param.scope)
        .bind(&param.menu_strictly)
        .bind(&param.dept_strictly)
        .bind(&param.status)
        .bind(&param.del_flag)
        .bind(&param.remark)
        .bind("createPerson")
        .bind(&now)
        .bind("updatePerson")
        .bind(&now)
        .fetch_one(conn)
        .await?;

    Ok(id)
}

pub async fn upd<'a, C>(conn: C, param: &SaveRole) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let now = DateUtil::get_current_datetime();

    let qry = r#"
        UPDATE sys_role SET 
          name = $1, 
          key = $2, 
          sort = $3,  
          scope = $4,
          menu_strictly = $5, 
          dept_strictly = $6, 
          status = $7, 
          del_flag = $8, 
          remark = $9,                                  
          update_by = $12, 
          update_time = $13                        
        WHERE id = $3
        "#;
    sqlx::query(&qry)
        .bind(&param.name)
        .bind(&param.key)
        .bind(&param.sort)
        .bind(&param.scope)
        .bind(&param.menu_strictly)
        .bind(&param.dept_strictly)
        .bind(&param.status)
        .bind(&param.del_flag)
        .bind(&param.remark)
        .bind("")
        .bind(&now)
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
            DELETE FROM sys_role WHERE {}
        "#,
        ids.iter()
            .map(|id| format!("id = '{}'", id))
            .collect::<Vec<String>>()
            .join(" OR ")
    );

    sqlx::query(&qry).execute(conn).await?;
    Ok(())
}

pub async fn cnt_by_name<'a, C>(conn: C, name: &str, id: Option<&str>) -> AppResult<u32>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!(
        r#"
            SELECT COUNT(1) 
            FROM sys_role 
            WHERE name = '{}'  
        "#,
        name
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

