use std::sync::Arc;

use kcp_sys::{endpoint::*, packet_def::BytesMut, stream::KcpStream};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UdpSocket,
};

#[tokio::main]
async fn main() {
    let mut endpoint = KcpEndpoint::new();
    endpoint.run().await;

    let (input, mut output) = (endpoint.input_sender(), endpoint.output_receiver().unwrap());

    let udp_socket = Arc::new(UdpSocket::bind("0.0.0.0:54321").await.unwrap());
    udp_socket.connect("127.0.0.1:54320").await.unwrap();

    let udp = udp_socket.clone();
    tokio::spawn(async move {
        while let Some(data) = output.recv().await {
            udp.send(&data.inner()).await.unwrap();
        }
    });

    let udp = udp_socket.clone();
    tokio::spawn(async move {
        loop {
            let mut buf = vec![0; 1024];
            let (size, _) = udp.recv_from(&mut buf).await.unwrap();
            input
                .send(BytesMut::from(&buf[..size]).into())
                .await
                .unwrap();
        }
    });

    loop {
        let conn_id = endpoint.accept().await.unwrap();
        let mut kcp_stream = KcpStream::new(&endpoint, conn_id).unwrap();

        let mut buf = vec![0; 64 * 1024];
        let size = kcp_stream.read(&mut buf).await.unwrap();
        println!("server recv {}", String::from_utf8_lossy(&buf[..size]));

        kcp_stream.write_all(&buf[..size]).await.unwrap();
        kcp_stream.flush().await.unwrap();
    }
}
