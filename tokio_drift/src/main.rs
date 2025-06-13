use tokio::net::{UdpSocket,TcpSocket,TcpStream};
use std::{net::SocketAddr ,sync::Arc, error::Error};
use tokio::sync::mpsc;
use tokio::io::AsyncWriteExt;

async fn TcpSpawn(buf:&[u8],mut tcp_stream:&mut TcpStream) -> Result<(), Box<dyn Error>>{
    tcp_stream.write_all(&buf).await?;
    println!("spawned tcp");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap()).await?;
    let sock = Arc::from(sock);
    let s = sock.clone();
    let (_tx,mut _rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);
    /*
    tokio::spawn(async move{
        while let Some((bytes, addr)) = rx.recv().await{
            let len = s.send_to(&bytes, &addr).await.unwrap();
            println!("{:?} bytes sent", len); // use later when relaying tcp conversion
        }
    });
    */
    let tcpaddr = "127.0.0.1:8082".parse::<SocketAddr>()?;
    let tcpsocket = TcpSocket::new_v4()?;
    tcpsocket.bind(tcpaddr)?;
    let listener = tcpsocket.listen(1024)?;
    let (mut tcp_stream, client_addr) = listener.accept().await?;

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("received {} bytes from {}", len, addr);
        TcpSpawn(&buf[..len],&mut tcp_stream).await?;
        //tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    }
}
