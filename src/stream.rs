use std::error::Error;

use tokio::net::{TcpStream, ToSocketAddrs};
pub async fn connect<T: ToSocketAddrs>(addr: T) ->TcpStream {
    if let Ok(stream) = TcpStream::connect(addr).await {
        stream
    }else {
        println!("couldn't connect to the server");
    }
    
}
