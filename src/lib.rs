pub mod config;
pub mod util;
pub mod log;
pub mod res;
pub mod db;
pub mod api;
pub mod catcher;
pub mod global;
pub mod middleware;
pub mod cache;

pub use res::result::{AppResult,Res};
pub use db::sqlite::get_pool;
