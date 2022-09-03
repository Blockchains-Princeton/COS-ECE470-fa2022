use super::message::Message;
use futures::{channel::mpsc, sink::SinkExt};
use log::trace;
use smol::Async;

pub fn new(
    stream: &Async<std::net::TcpStream>,
) -> std::io::Result<(mpsc::UnboundedReceiver<Vec<u8>>, Handle)> {
    let (write_sender, write_receiver) = mpsc::unbounded();
    let addr = stream.get_ref().peer_addr()?;
    let handle = Handle {
        write_queue: write_sender,
        addr,
    };
    Ok((write_receiver, handle))
}

#[derive(Copy, Clone)]
pub enum Direction {
    Incoming,
    Outgoing,
}

#[derive(Clone, Debug)]
pub struct Handle {
    addr: std::net::SocketAddr,
    write_queue: mpsc::UnboundedSender<Vec<u8>>,
}

#[cfg(any(test,test_utilities))]
pub struct TestReceiver {
    r: mpsc::UnboundedReceiver<Vec<u8>>
}

impl Handle {
    pub fn write(&mut self, msg: Message) {
        let buffer = bincode::serialize(&msg).unwrap();
        smol::block_on(async move {
            if self.write_queue.send(buffer).await.is_err() {
                trace!("Trying to send to disconnected peer");
            }
        });
    }

    pub fn addr(&self) -> &std::net::SocketAddr {
        &self.addr
    }

    #[cfg(any(test,test_utilities))]
    pub fn test_handle() -> (Handle, TestReceiver) {
        let (s,r) = mpsc::unbounded();
        (Handle {
            addr: std::net::SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), 12321),
            write_queue: s,
        },
        TestReceiver {
            r
        })
    }
}

#[cfg(any(test,test_utilities))]
impl TestReceiver {
    pub fn recv(&mut self) -> Message {
        let bytes = smol::block_on(futures::stream::StreamExt::next(&mut self.r)).unwrap();
        let msg: Message = bincode::deserialize(&bytes).unwrap();
        msg
    }
}