use std::time::Duration;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn do_get(key: &str,mut stream:TcpStream) {
    // println!("connecting!");
    // let mut stream = TcpStream::connect("127.0.0.1:6379").await.unwrap();
    // println!("connected!");

    // let bulk_get = format!("$3\r\nget\r\n");
    // let bulk_key = format!("${}\r\n{key}\r\n", key.len());
    // let s = format!("*2\r\n{bulk_get}{bulk_key}");

    // try to design a simple protocol
    let bulk_get = format!("$3\r\nget\r\n");
    let bulk_key = format!("${}\r\n{key}\r\n", key.len());
    let s = format!("*2\r\n{bulk_get}{bulk_key}");

    println!("send get: {s:?}");

    if let Err(err) = stream.write_all(s.as_bytes()).await {
        eprintln!("send error: {err}");
    }

    std::thread::sleep(Duration::from_millis(500));
    // tokio::time::sleep(Duration::from_millis(500));

    let mut buf: Vec<u8> = Vec::with_capacity(2000);
    match stream.read_buf(&mut buf).await {
        Ok(len) => {
            println!("read {len} bytes");
        }
        Err(err) => {
            eprintln!("read error: {err}");
        }
    }

    let data = String::from_utf8(buf);
    let data_unpack = data.unwrap();
    println!("get data: {data_unpack}");

    println!("writing bytes!");
}
