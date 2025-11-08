use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
pub async fn execute_cmd(cmd:&str,db: Db) -> String {
    let vec_data:Vec<&str> = cmd.trim().split_whitespace().collect();
    if vec_data.is_empty() {
        return  "-ERR empty command\r\n".to_string();
    }
    match vec_data[0] {
      "GET" => get(db,vec_data).await,
        "SET" => set(db,vec_data).await,
        _ => "-ERR unknown command\r\n".to_string(),
    }
}
async fn get(db:Db,vec_data:Vec<&str>) ->String {
    if vec_data.len() != 2 {
        return "-ERR wrong number of arguments for 'get' command\r\n".to_string();
    }
    let db = db.lock().await;
    match db.get(vec_data[1]) {
        Some(value) => format!("+{}\r\n", String::from_utf8_lossy(value)),
        None => "$-1\r\n".to_string(),
    }
}


async fn set(db:Db,vec_data:Vec<&str>) ->String {
    if vec_data.len() != 3 {
        return "-ERR wrong number of arguments for 'set' command\r\n".to_string();
    }
    let mut db = db.lock().await;
    db.insert(vec_data[1].to_string(), vec_data[2].as_bytes().to_vec().into());
    "+OK\r\n".to_string()
}