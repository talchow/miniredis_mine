use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

pub async fn execute_get(key: &str, db: Db) -> String {
    let db = db.lock().await;
    match db.get(key) {
        Some(value) => format!("{}\r\n", String::from_utf8_lossy(value)),
        None => format!("No value found for key: {}\r\n", key),
    }
}

pub async fn execute_set(key: &str, value: &str, db: Db) -> String {
    let mut db = db.lock().await;
    db.insert(key.to_string(), value.as_bytes().to_vec().into());
    "OK\r\n".to_string()
}