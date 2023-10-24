use super::stc::{Menu, SaveMenu};
use crate::{
    util::{date::DateUtil, uuid::MyUuid},
    AppResult,
};
use sqlx::Sqlite;

pub async fn sel<'a, C>(conn: C, id: &str) -> AppResult<Option<Menu>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT * FROM sys_menu WHERE id = $1
    "#;
    let info = sqlx::query_as::<Sqlite, Menu>(&qry)
        .bind(id)
        .fetch_optional(conn)
        .await?;
    Ok(info)
}

pub async fn sel_items_by_ids<'a, C>(conn: C, ids: &Vec<String>) -> AppResult<Vec<Menu>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT * FROM sys_menu 
    "#;

    let mut qry_or = ids
        .iter()
        .map(|id| format!(" id = '{}'", id))
        .collect::<Vec<String>>()
        .join(" OR ");

    if !qry_or.is_empty() {
        qry_or = format!(" WHERE {}", qry_or)
    }

    let qry = format!("{}{}", qry, qry_or);

    let items = sqlx::query_as::<Sqlite, Menu>(&qry)
        .fetch_all(conn)
        .await?;
    Ok(items)
}

pub async fn ins<'a, C>(conn: C, param: &SaveMenu, level: &str, sort: i64, pid: &str) -> AppResult<String>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let id = MyUuid::new();
    let now = DateUtil::get_current_common_datetime();

    let qry = r#"
        INSERT INTO sys_menu (
          id, pid, level, name, path 
          ,component ,query ,is_frame ,is_cache ,type 
          ,visible ,status ,perms ,icon , remark
          ,order_num ,create_by ,create_time ,update_by ,update_time   
        ) 
        VALUES(
          $1 ,$2 ,$3 ,$4 ,$5
          ,$6 ,$7 ,$8 ,$9 ,$10
          ,$11 ,$12 ,$13 ,$14 ,$15
          ,$16 ,$17 ,$18 ,$19, $20
        ) returning id
    "#;
    let id = sqlx::query_scalar(&qry)
        .bind(id)
        .bind(pid)
        .bind(&level)
        .bind(&param.name)
        .bind(&param.path)
        /*****/
        .bind(&param.component)
        .bind(&param.query)
        .bind(&param.is_frame)
        .bind(&param.is_cache)
        .bind(&param.menu_type)
        /*****/
        .bind(&param.visible)
        .bind(&param.status)
        .bind(&param.perms)
        .bind(&param.icon)
        .bind(&param.remark)
        /*****/
        .bind(&sort)
        .bind("createPerson")
        .bind(&now)
        .bind("updatePerson")
        .bind(&now)
        .fetch_one(conn)
        .await?;

    Ok(id)
}

pub async fn upd<'a, C>(conn: C, param: &SaveMenu, level: &str) -> AppResult<String>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    // let now = DateUtil::get_current_common_datetime();

    let qry = r#"
        UPDATE sys_menu SET 
            pid = $1,
            level = $2,
            name = $3,
            path = $4,
            component = $5,
            query = $6,
            is_frame = $7,
            is_cache = $8,
            type = $9,
            visible = $10,
            status = $11,
            perms = $12,
            icon = $13,
            order_num = $14,
        WHERE id = $15
    "#;
    let id = sqlx::query_scalar(&qry)
        .bind(&param.pid)
        .bind(&level)
        .bind(&param.name)
        .bind(&param.path)
        .bind(&param.component)
        .bind(&param.query)
        .bind(&param.is_frame)
        .bind(&param.is_cache)
        .bind(&param.menu_type)
        .bind(&param.visible)
        .bind(&param.status)
        .bind(&param.perms)
        .bind(&param.icon)
        .bind(&param.order_num)
        .bind(&param.id)
        .fetch_one(conn)
        .await?;
    Ok(id)
}

pub async fn del<'a, C>(conn: C, ids: &Vec<String>) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!(
        r#"DELETE FROM sys_menu WHERE {}"#,
        ids.iter()
            .map(|id| format!("id = '{}'", id))
            .collect::<Vec<String>>()
            .join(" OR ")
    );

    sqlx::query(&qry).execute(conn).await?;
    Ok(())
}

pub async fn sel_children<'a, C>(conn: C, level: &str) -> AppResult<Vec<Menu>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let children: Vec<Menu> = sqlx::query_as::<_, Menu>(
        r#"
            SELECT *
            FROM sys_menu 
            WHERE level = ? or level like ?
        "#,
    )
    .bind(level.to_string())
    .bind(level.to_string() + ".%")
    .fetch_all(conn)
    .await?;
    Ok(children)
}

pub async fn upd_children<'a, C>(conn: C, children: Vec<Menu>) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let mut qry_casewhen = String::from("");
    let mut qry_condition = String::from("");
    let qry = String::from(" UPDATE sys_menu SET level = CASE id \n ");
    for child in &children {
        qry_casewhen = format!(
            "{} {}",
            qry_casewhen,
            format!(" WHEN '{}' THEN '{}' \n ", &child.id, &child.level)
        );
    }
    let qry_end = format!(" END \n WHERE ");
    for (idx, child) in children.iter().enumerate() {
        if idx == 0 {
            qry_condition = format!(" id = '{}' \n ", &child.id);
        } else {
            qry_condition = format!(
                "{}{}",
                qry_condition,
                format!(" OR id = '{}' \n ", &child.id)
            );
        }
    }

    let qry = format!("{}{}{}{}", qry, qry_casewhen, qry_end, qry_condition);
    sqlx::query(qry.as_str()).execute(conn).await?;

    Ok(())
}

pub async fn cnt_by_name<'a, C>(
    conn: C,
    name: String,
    id: Option<String>,
    pid: Option<String>,
) -> AppResult<u32>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!("SELECT COUNT(1) FROM sys_menu WHERE name = '{}' ", name);

    let qry_pid = if let Some(pid) = pid {
        format!(" AND pid = '{}' ", pid)
    } else {
        format!(" AND pid = '0'")
    };

    let qry_id = if let Some(id) = id {
        format!(" AND id != '{}' ", id)
    } else {
        "".to_string()
    };

    let qry = format!("{}{}{}", qry, qry_pid, qry_id);
    let cnt: u32 = sqlx::query_scalar(&qry.as_str())
        .fetch_one(conn)
        .await?;

    Ok(cnt)
}

pub async fn cnt_by_pid<'a, C>(conn: C, pid: &str) -> AppResult<u32>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!("SELECT COUNT(1) FROM sys_menu WHERE pid =  ?",);

    let count = sqlx::query_scalar(&qry).bind(pid).fetch_one(conn).await?;

    Ok(count)
}
