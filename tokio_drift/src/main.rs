use tokio::net::{UdpSocket};
use std::{io, net::SocketAddr ,sync::Arc};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap()).await?;
    let sock = Arc::from(sock);
    let s = sock.clone();
    let (_tx,mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);
    
    tokio::spawn(async move{
        while let Some((bytes, addr)) = rx.recv().await{
            let len = s.send_to(&bytes, &addr).await.unwrap();
            println!("{:?} bytes sent", len);
        }
    });
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("received {} bytes from {}", len, addr);
    }
}
