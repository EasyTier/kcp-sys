use std::{
    pin::Pin,
    task::ready,
    task::{Context, Poll},
};

use bytes::{Bytes, BytesMut};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use crate::endpoint::{ConnId, KcpEndpoint, KcpStreamReceiver};

pub struct KcpStream {
    sender: tokio_util::sync::PollSender<BytesMut>,
    receiver: KcpStreamReceiver,
    conn_id: ConnId,
    conn_data: Bytes,
}

impl std::fmt::Debug for KcpStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KcpStream")
            .field("conn_id", &self.conn_id)
            .finish()
    }
}

impl KcpStream {
    pub fn new(endpoint: &KcpEndpoint, conn_id: ConnId) -> Option<Self> {
        let (sender, receiver) = endpoint.conn_sender_receiver(conn_id)?;
        let conn_data = endpoint.conn_data(&conn_id)?;
        Some(Self {
            sender: tokio_util::sync::PollSender::new(sender),
            receiver,
            conn_id,
            conn_data,
        })
    }

    pub fn conn_data(&self) -> &Bytes {
        &self.conn_data
    }

    pub fn conn_id(&self) -> ConnId {
        self.conn_id
    }
}

impl AsyncRead for KcpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut ReadBuf,
    ) -> Poll<std::io::Result<()>> {
        let Some(read_buf) = ready!(self.receiver.poll_recv(cx)) else {
            return Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "stream closed",
            )));
        };

        buf.put_slice(&read_buf);
        Poll::Ready(Ok(()))
    }
}

impl AsyncWrite for KcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let mut ret = ready!(self.sender.poll_reserve(cx));
        if ret.is_ok() {
            ret = self.sender.send_item(BytesMut::from(buf));
        }
        match ret {
            Ok(_) => Poll::Ready(Ok(buf.len())),
            Err(_) => Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "stream closed",
            ))),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<std::io::Result<()>> {
        self.sender.close();
        Poll::Ready(Ok(()))
    }
}
