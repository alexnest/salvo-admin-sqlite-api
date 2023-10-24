use crate::config::APPLICATION_CONFIG;
use once_cell::sync::OnceCell;
use sqlx::SqlitePool;

static SQLITE_POOL: OnceCell<SqlitePool> = OnceCell::new();

pub async fn init_database() {
    let host = format!("{}", APPLICATION_CONFIG.database.host); // 相对路径
    let pool = SqlitePool::connect(&host)
        .await
        .expect("Failed to create pool.");
    SQLITE_POOL
        .set(pool)
        .expect("sqlite global pool set failed.")
}

#[inline]
pub fn get_pool() -> &'static SqlitePool{
    // Safety: tt is already set when the program is initialized
    unsafe { SQLITE_POOL.get_unchecked() }
}
