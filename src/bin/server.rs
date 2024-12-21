use core::str;
use std::{collections::HashMap,sync::{Arc, Mutex}};

use bytes::{Buf, Bytes, BytesMut};
use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
use tokio::io::AsyncReadExt;
use std::error::Error;

type Db = Arc<Mutex<HashMap<String,Bytes>>>;
#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    println!("Listening!");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket,_) = listener.accept().await?;
       
        let db:Db = db.clone();

        println!("Accepted!");

        tokio::spawn(async move {
            let _ = process(socket,db).await;
        });
    }
}

async fn process(mut socket:TcpStream,db:Db) -> Result<(),Box<dyn Error>>{
    let mut buf = BytesMut::new();
   let _ = socket.read_buf(&mut buf).await;
   println!("reach here");
    loop {
        let frame:Vec<u8>= match buf.get_u8() {
            b'*' => {
                buf.to_vec()
            }
            _ => break Ok(()),
        };
        
        let frame_string = String::from_utf8(frame)?;
        let vec_data:Vec<&str> = frame_string.split("\r\n").collect();

        let res = match vec_data[0] {
            "2" => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(vec_data[4]) {
                    value.clone()
                }else {
                    break Ok(());
                }
            }
            "3" => {
                let mut db = db.lock().unwrap();
                db.insert(vec_data[4].to_string(), vec_data[6].to_owned().into());
                "ok".into()

            }
            _ => {
                println!("frame format is error!");
                break Ok(());
            }
            
        };
        socket.write_all(&res).await?;
    }
}