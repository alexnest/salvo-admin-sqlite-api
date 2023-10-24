use super::stc::{PageUser, UpdUser, User, UserWithPwd, SaveUser};
use crate::{
    util::{date::DateUtil, uuid::MyUuid},
    AppResult,
};
use bcrypt;
use sqlx::Sqlite;

pub async fn sel<'a, C>(conn: C, id: &str) -> AppResult<Option<User>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT * FROM sys_user WHERE id = $1
    "#;
    let items = sqlx::query_as::<Sqlite, User>(&qry)
        .bind(id)
        .fetch_optional(conn)
        .await?;
    Ok(items)
}

pub async fn sel_by_user_ids<'a, C>(conn: C, ids: Vec<String>) -> AppResult<Vec<User>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!(
        r#"
            SELECT * FROM sys_user WHERE {}
        "#,
        ids.iter()
            .map(|id| format!("id = '{}'", id))
            .collect::<Vec<String>>()
            .join(" OR ")
    );

    let items = sqlx::query_as::<Sqlite, User>(&qry).fetch_all(conn).await?;
    Ok(items)
}

pub async fn sel_page<'a, C>(conn: C, param: &PageUser) -> AppResult<Vec<User>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT * FROM sys_user ORDER BY update_time DESC 
    "#;
    let page = sqlx::query_as::<Sqlite, User>(&qry)
        .bind(param.page.page_size)
        .bind(param.page.offset())
        .fetch_all(conn)
        .await?;
    Ok(page)
}

pub async fn cnt_by_page<'a, C>(conn: C, _param: &PageUser) -> AppResult<u32>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = format!("SELECT COUNT(1) FROM sys_user",);

    let count = sqlx::query_scalar(&qry).fetch_one(conn).await?;

    Ok(count)
}

pub async fn ins<'a, C>(conn: C, param: &SaveUser) -> AppResult<String>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let id = MyUuid::new();
    let now = DateUtil::get_current_common_datetime();
    let password = bcrypt::hash(&param.clone().password.as_ref().unwrap(), bcrypt::DEFAULT_COST).unwrap();

    let qry = r#"
        INSERT INTO sys_user (
          id, name, nick_name, type, dept_id 
          ,email, phonenumber, password,  create_by, create_time
          ,update_by, update_time
        ) 
        VALUES(
            $1, $2, $3, $4, $5
            ,$6, $7, $8, $9, $10
            ,$11, $12
        ) returning id
        "#;
    let id = sqlx::query_scalar(&qry)
        .bind(id)
        .bind(&param.name)
        .bind(&param.nick_name)
        .bind(0)
        .bind(&param.dept_id)
        .bind(&param.email)
        .bind(&param.phonenumber)
        .bind(password)
        .bind("createPerson")
        .bind(&now)
        .bind("updatePerson")
        .bind(&now)
        .fetch_one(conn)
        .await?;

    Ok(id)
}

pub async fn upd<'a, C>(conn: C, param: &UpdUser) -> AppResult<()>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let now = DateUtil::get_current_datetime();
    let password = bcrypt::hash(&param.password, bcrypt::DEFAULT_COST).unwrap();

    let qry = r#"
        UPDATE sys_user SET 
            name = $1, 
            password = $2,
            update_time = $3 
        WHERE id = $4
        "#;
    sqlx::query(&qry)
        .bind(&param.user_name)
        .bind(password)
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
            DELETE FROM sys_user WHERE {}
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
            FROM sys_user 
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

pub async fn sel_by_name<'a, C>(conn: C, name: &str) -> AppResult<Option<UserWithPwd>>
where
    C: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let qry = r#"
        SELECT * FROM sys_user WHERE name = $1
    "#;
    let info = sqlx::query_as::<Sqlite, UserWithPwd>(&qry)
        .bind(name)
        .fetch_optional(conn)
        .await?;
    Ok(info)
}
