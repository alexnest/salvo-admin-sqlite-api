use mini_moka::sync::Cache;
use once_cell::sync::OnceCell;
use std::{time::Duration};

pub static CACHE: OnceCell<Cache<String, i32>> = OnceCell::new();

pub async fn init_cache() {
    let cache = Cache::builder()
        // Time to live (TTL): 30 minutes
        .time_to_live(Duration::from_secs(30 * 60))
        // Time to idle (TTI):  5 minutes
        .time_to_idle(Duration::from_secs(5 * 60))
        // Create the cache.
        .build();

    CACHE.set(cache).expect("cache global set failed.")
}

pub async fn get(key: &String) -> Option<i32> {
    let temp = CACHE.get().expect("cache global get failed.");
    temp.get(key)
}

pub async fn insert(key: String, val: i32) {
    let temp = CACHE.get().expect("cache global get failed.");
    temp.insert(key, val);
}

// pub async fn insert(key: String, val: u32) {
//     CACHE.ca
// }
