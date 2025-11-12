use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::http::Response;
use axum::Router;
use axum::routing::post;
use tokio::sync::Mutex;

use bytes::Bytes;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

mod execute_data;
use execute_data::execute_cmd;
type Db = Arc<Mutex<HashMap<String, Bytes>>>;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let db = Arc::new(Mutex::new(HashMap::new()));

   let listener = TcpListener::bind("127.0.0.1:6379").await?;
   println!("TCP listening on 127.0.0.1:6379");
   tokio::spawn(tcp_listener(listener,db.clone()));

   println!("HTTP api listening on 127.0.0.1:8080");
   let app = Router::new()
       .route("/api/redis", post(move |body:String| 
    redis_handler(body,db.clone())));
    let addr = SocketAddr::from(([127,0,0,1],8080));
    println!("HTTP listening on http://{addr}/api/redis");

    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }); 

    tokio::signal::ctrl_c().await?;
    println!("Shutting down");
    Ok(())
}

async fn tcp_listener(listener: TcpListener, db: Db) {
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            let mut buf = vec![0u8;1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let input = String::from_utf8_lossy(&buf[..n]);
                        let rsp = execute_cmd(&input, db.clone()).await;

                        if socket.write_all(rsp.as_bytes()).await.is_err() {break;}
                    }
                    Err(_) => break,
                }
            }
        });
    }
}

async fn redis_handler(body: String, db: Db) -> Response<String> {
    let rsp = execute_cmd(&body, db).await;
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .body(rsp)
        .unwrap()
}



