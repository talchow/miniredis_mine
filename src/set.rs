use std::error::Error;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn do_set(key: &str, value: &str) -> Result<(), Box<dyn Error>> {
    // println!("connecting!");
    // let mut stream = TcpStream::connect("127.0.0.1:6379").await;
    let mut stream = TcpStream::connect("127.0.0.1:6379").await?;
    // println!("connected!");

    let bulk_set = format!("$3\r\nset\r\n");
    let bulk_key = format!("${}\r\n{key}\r\n", key.len());

    let bulk_value = format!("${}\r\n{value}\r\n", value.len());
    let s = format!("*3\r\n{bulk_set}{bulk_key}{bulk_value}");

    println!("send set: {s:?}");
    if let Err(err) = stream.write_all(s.as_bytes()).await {
        eprintln!("send error: {err}");
    }

    println!("writing bytes!");
    Ok(())
}
