use core::str;
use std::{collections::HashMap, sync::{Arc, Mutex}};

use bytes::{Buf, Bytes, BytesMut};
use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
use tokio::io::AsyncReadExt;

type Db = Arc<Mutex<HashMap<String,Bytes>>>;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening!");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket,_) = listener.accept().await.unwrap();

        let db:Db = db.clone();

        println!("Accepted!");

        tokio::spawn(async move {
            process(socket,db).await;
        });
    }
}

async fn process(mut socket:TcpStream,db:Db) {
    let mut buf = BytesMut::new();
   let _ = socket.read_buf(&mut buf).await;
    loop {
        let frame:Vec<u8>= match buf.get_u8() {
            b'*' => {
                buf.to_vec()
            }
            _ => break,
        };
        
        let err = "the bytes is error".to_string();
        let frame_string = String::from_utf8(frame).unwrap_or(err);
        let vec_data:Vec<&str> = frame_string.split("\r\n").collect();

        let res = match vec_data[0] {
            "2" => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(vec_data[4]) {
                    value.clone()
                }else {
                    break;
                }
            }
            "3" => {
                let mut db = db.lock().unwrap();
                db.insert(vec_data[4].to_string(), vec_data[6].to_owned().into());
                "ok".into()

            }
            _ => {
                println!("frame format is error!");
                break;
            }
            
        };
        socket.write_all(&res).await.unwrap();
    }
}