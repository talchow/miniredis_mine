use axum::Router;
use axum::http::Response;
use axum::routing::{post, options};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use bytes::Bytes;
use std::error::Error;

mod execute_data;
use execute_data::*;
mod types;
use types::{GetRequest, SetRequest};
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化数据库,使用 Arc 和 Mutex 包装 HashMap,实现线程安全的并发访问
    let db = Arc::new(Mutex::new(HashMap::new()));

    // 定义 HTTP 路由,监听 127.0.0.1:8080 端口,并暴露api为/api/redis,调用 redis_handler 函数处理请求
    let app = Router::new()
        .route(
            "/api/get",
            post({
                let db = db.clone();
                move |body: axum::extract::Json<GetRequest>| get_handler(body, db.clone())
            }),
        )
        .route(
            "/api/get",
            options(cors_preflight_response),
        )
        .route(
            "/api/set",
            post({
                let db = db.clone();
                move |body: axum::extract::Json<SetRequest>| set_handler(body, db.clone())
            }),
        )
        .route(
            "/api/set",
            options(cors_preflight_response),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("HTTP listening on http://{addr}/api/redis");

    // 启动 HTTP 服务器,并等待 Ctrl+C 信号,触发关闭
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    tokio::signal::ctrl_c().await?;
    println!("Shutting down");
    Ok(())
}


async fn get_handler(body: axum::extract::Json<GetRequest>, db: Db) -> Response<String> {
    let rsp = execute_get(&body.key, db).await;
    Response::builder()
        .header("Access-Control-Allow-Origin", "http://127.0.0.1:3000")
        .header("Access-Control-Allow-Credentials", "true")
        .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        .header("Access-Control-Allow-Methods", "POST, OPTIONS, GET")
        .header("Content-Type", "text/plain")
        .body(rsp)
        .unwrap()
}

async fn set_handler(body: axum::extract::Json<SetRequest>, db: Db) -> Response<String> {
    let rsp = execute_set(&body.key, &body.value, db).await;
    Response::builder()
        .header("Access-Control-Allow-Origin", "http://127.0.0.1:3000")
        .header("Access-Control-Allow-Credentials", "true")
        .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        .header("Access-Control-Allow-Methods", "POST, OPTIONS, GET")
        .header("Content-Type", "text/plain")
        .body(rsp)
        .unwrap()
}

async fn cors_preflight_response() -> Response<String> {
    Response::builder()
        .header("Access-Control-Allow-Origin", "http://127.0.0.1:3000")
        .header("Access-Control-Allow-Methods", "POST, OPTIONS, GET")
        .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        .header("Access-Control-Allow-Credentials", "true")
        .header("Content-Control-Max-Age", "86400")
        .body("".to_string())
        .unwrap()
}