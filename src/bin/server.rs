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

use cli_for_miniredis::execute_data::execute_cmd;
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



// async fn process(mut socket: TcpStream, db: Db) -> Result<(), ProcessError> {
//     // type Result<T> = core::result::Result<T, ProcessError>;
//     // 创建一个缓冲区，用于存储从客户端读取的数据
//     // 把socket数据读到buf中
//     let mut buf = BytesMut::new();
//     let _ = socket.read_buf(&mut buf).await;
//     println!("reach here");
//     // 循环处理buf中的数据,如果数据开头是*，则认为是一个完整的frame,写入frame中，否则认为是一个不完整的frame
//     loop {
//         let frame: Vec<u8> = match buf.get_u8() {
//             b'*' => buf.to_vec(),
//             _ => break Ok(()),
//         };
//         // 将frame转换为字符串,并根据"\r\n"分割为vec
//         let frame_string = String::from_utf8(frame)?;
//         let vec_data: Vec<&str> = frame_string.split("\r\n").collect();

//         let res = match vec_data[0] {
//             "2" => {
//                 let db_guard = db.lock().await;

//                 if let Some(value) = db_guard.get(vec_data[4]) {
//                     value.clone()
//                 } else {
//                     break Ok(());
//                 }
//             }
//             "3" => {
//                 let mut db_guard = db.lock().await;
//                 db_guard.insert(vec_data[4].to_string(), vec_data[6].to_owned().into());
//                 "ok".into()
//             }
//             _ => {
//                 println!("frame format is error!");
//                 break Ok(());
//             }
//         };
//         let _ = match socket.write_all(&res).await {
//             Ok(_) => {
//                 println!("writed!");
//             }
//             Err(_e) => return Err(ProcessError::Incomplete),
//         };
//     }
// }
