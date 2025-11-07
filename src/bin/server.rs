use core::str;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::{Buf, Bytes, BytesMut};
use cli_for_miniredis::ProcessError;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    println!("Listening!");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await?;

        let db: Db = db.clone();

        println!("Accepted!");

        tokio::spawn(async move {
            if let Err(err) = process(socket, db).await {
                eprintln!("processor error: {err}");
                // match err {
                //   ProcessError::FromUtf8Error()  > x 
                //   ProcessError::FromUtf8Error()  > x 
                // }
            }
        });
    }
}

async fn process(mut socket: TcpStream, db: Db) -> Result<(), ProcessError> {
    // type Result<T> = core::result::Result<T, ProcessError>;
    // 创建一个缓冲区，用于存储从客户端读取的数据
    // 把socket数据读到buf中
    let mut buf = BytesMut::new();
    let _ = socket.read_buf(&mut buf).await;
    println!("reach here");
    // 循环处理buf中的数据,如果数据开头是*，则认为是一个完整的frame,写入frame中，否则认为是一个不完整的frame
    loop {
        let frame: Vec<u8> = match buf.get_u8() {
            b'*' => buf.to_vec(),
            _ => break Ok(()),
        };
        // 将frame转换为字符串,并根据"\r\n"分割为vec
        let frame_string = String::from_utf8(frame)?;
        let vec_data: Vec<&str> = frame_string.split("\r\n").collect();

        let res = match vec_data[0] {
            "2" => {
                let db = match db.lock() {
                    Ok(db) => db,
                    Err(_e) => return Err(ProcessError::PoisonError("DB lock".to_string())),
                };

                if let Some(value) = db.get(vec_data[4]) {
                    value.clone()
                } else {
                    break Ok(());
                }
            }
            "3" => {
                let mut db = db
                    .lock()
                    .map_err(|_| ProcessError::PoisonError("DB lock".to_string()))?;
                db.insert(vec_data[4].to_string(), vec_data[6].to_owned().into());
                "ok".into()
            }
            _ => {
                println!("frame format is error!");
                break Ok(());
            }
        };
        let _ = match socket.write_all(&res).await {
            Ok(_) => {
                println!("writed!");
            }
            Err(_e) => return Err(ProcessError::Incomplete),
        };
    }
}
