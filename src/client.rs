use bytes::Bytes;
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    stream:TcpStream,
    buf:Bytes,
}

impl Client {
    pub async fn new(socket:Tcpstream) -> Client {
        Client {
            stream:TcpStream,
            buf:Bytes,
        }
    }
    pub async fn connect<A:ToSocketAddrs>(addr:A) -> Result<Client> {
        let socket = TcpStream::connect("127.0.0.1:6379").await?;
        let client = Client {
            stream:socket,
            buf:Bytes,
        };
        Ok(Client)

    }
}