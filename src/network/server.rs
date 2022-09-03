use crate::types::address::Address;
use super::peer;
use super::message;

use async_dup::Arc as AsyncArc;
use futures::io::{AsyncReadExt, AsyncWriteExt};
use futures::io::{BufReader, BufWriter};
use futures::{channel::oneshot, stream::StreamExt};
use smol::{Async, Executor};
use log::{debug, info, trace};
use std::net;
use std::sync::Arc;
use std::thread;


pub fn new(
    addr: std::net::SocketAddr,
    msg_sink: smol::channel::Sender<(Vec<u8>, peer::Handle)>,
) -> std::io::Result<(Context, Handle)> {
    let (control_signal_sender, control_signal_receiver) = smol::channel::bounded(10000);
    let handle = Handle {
        control_chan: control_signal_sender.clone(),
    };
    let ctx = Context {
        peers: std::collections::HashMap::new(),
        addr,
        control_chan: control_signal_receiver,
        control_sender: control_signal_sender,
        new_msg_chan: msg_sink,
    };
    Ok((ctx, handle))
}

pub struct Context {
    peers: std::collections::HashMap<std::net::SocketAddr, peer::Handle>,
    addr: std::net::SocketAddr,
    control_chan: smol::channel::Receiver<ControlSignal>,
    control_sender: smol::channel::Sender<ControlSignal>,
    new_msg_chan: smol::channel::Sender<(Vec<u8>, peer::Handle)>,
}

impl Context {
    /// Start a new server context.
    pub fn start(self) -> std::io::Result<()> {
        // initialize the server socket
        let listener = Async::<net::TcpListener>::bind(self.addr)?;
        info!("P2P server listening at {}", self.addr);
        let control_chan = self.control_sender.clone();
        let ex = Executor::new();
        let ex = Arc::new(ex);
        let ex_clone = ex.clone();
        ex.spawn(async move {
            self.dispatch_control(ex_clone).await.unwrap();
        })
            .detach();
        ex.spawn(async move {
            Self::listener_loop(listener, control_chan).await.unwrap();
        })
            .detach();
        thread::spawn(move || smol::block_on(ex.run(futures::future::pending::<()>())));
        return Ok(());
    }

    /// the loop that endlessly accept incoming peers
    async fn listener_loop(
        listener: Async<net::TcpListener>,
        control_chan: smol::channel::Sender<ControlSignal>,
    ) -> std::io::Result<()> {
        loop {
            let (stream, addr) = listener.accept().await?;
            control_chan
                .send(ControlSignal::GetNewPeer(stream))
                .await
                .unwrap();
            info!("Incoming peer from {}", addr);
        }
    }

    async fn dispatch_control(mut self, ex: Arc<Executor<'_>>) -> std::io::Result<()> {
        // read the next control signal
        while let Ok(ctrl) = self.control_chan.recv().await {
            match ctrl {
                ControlSignal::ConnectNewPeer(addr, result_chan) => {
                    trace!("Processing ConnectNewPeer command");
                    let handle = self.connect(&addr, ex.clone()).await;
                    result_chan.send(handle).unwrap();
                }
                ControlSignal::BroadcastMessage(msg) => {
                    trace!("Processing BroadcastMessage command");
                    for (_, hd) in self.peers.iter_mut() {
                        hd.write(msg.clone());
                    }
                }
                ControlSignal::GetNewPeer(stream) => {
                    trace!("Processing GetNewPeer command");
                    self.accept(stream, ex.clone()).await?;
                }
                ControlSignal::DroppedPeer(addr) => {
                    trace!("Processing DroppedPeer({})", addr);
                    self.peers.remove(&addr);
                    info!("Peer {} disconnected", addr);
                }
                ControlSignal::SendToPeer((_receiver, _msg)) => {
                    unimplemented!()
                }
            }
        }
        return Ok(());
    }

    /// Connect to a peer, and register this peer
    async fn connect(
        &mut self,
        addr: &std::net::SocketAddr,
        ex: Arc<Executor<'_>>,
    ) -> std::io::Result<peer::Handle> {
        debug!("Establishing connection to peer {}", addr);
        let stream = Async::<std::net::TcpStream>::connect(addr.clone()).await?;

        // register the new peer
        self.register(stream, peer::Direction::Outgoing, ex).await
    }

    async fn accept(
        &mut self,
        stream: Async<net::TcpStream>,
        ex: Arc<Executor<'_>>,
    ) -> std::io::Result<()> {
        self.register(stream, peer::Direction::Incoming, ex).await?;
        Ok(())
    }

    async fn register(
        &mut self,
        stream: Async<net::TcpStream>,
        _direction: peer::Direction,
        ex: Arc<Executor<'_>>,
    ) -> std::io::Result<peer::Handle> {
        let (mut write_queue, handle) = peer::new(&stream)?;

        let stream = AsyncArc::new(stream);
        let new_msg_chan = self.new_msg_chan.clone();
        let handle_copy = handle.clone();
        let control_chan = self.control_sender.clone();
        let addr = stream.get_ref().peer_addr()?;

        // start the reactor for this peer
        // first, start a task that keeps reading from this guy
        let mut reader = BufReader::new(stream.clone());
        ex.spawn(async move {
            // the buffer to store the frame header, which contains the length of the frame
            let mut size_buffer: [u8; 4] = [0; 4];
            // the buffer to store the message content
            let mut msg_buffer: Vec<u8> = vec![];
            loop {
                // first, read exactly 4 bytes to get the frame header
                let msg_size = match reader.read_exact(&mut size_buffer).await {
                    Ok(_) => u32::from_be_bytes(size_buffer),
                    Err(_) => {
                        break;
                    }
                };
                // then, read exactly msg_size bytes to get the whole message
                if msg_buffer.len() < msg_size as usize {
                    msg_buffer.resize(msg_size as usize, 0);
                }
                match reader
                    .read_exact(&mut msg_buffer[0..msg_size as usize])
                    .await
                {
                    Ok(_) => {
                        let new_payload: Vec<u8> = msg_buffer[0..msg_size as usize].to_vec();
                        new_msg_chan
                            .send((new_payload, handle_copy.clone()))
                            .await
                            .unwrap();
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
            // the peer is disconnected
        })
            .detach();

        // second, start a task that keeps writing to this guy
        let mut writer = BufWriter::new(stream.clone());
        ex.spawn(async move {
            loop {
                // first, get a message to write from the queue
                let new_msg = write_queue.next().await.unwrap();

                // second, encode the length of the message
                let size_buffer = (new_msg.len() as u32).to_be_bytes();

                // third, write the frame header and the payload
                match writer.write_all(&size_buffer).await {
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                }
                match writer.write_all(&new_msg).await {
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                }
                match writer.flush().await {
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                }
            }
            // the peer is disconnected
            control_chan
                .send(ControlSignal::DroppedPeer(addr))
                .await
                .unwrap();
        })
            .detach();

        // insert the peer handle so that we can broadcast to this guy later
        self.peers.insert(addr, handle.clone());
        Ok(handle)
    }
}

#[derive(Clone)]
pub struct Handle {
    control_chan: smol::channel::Sender<ControlSignal>,
}
#[cfg(any(test,test_utilities))]
pub struct TestReceiver{
    control_chan: smol::channel::Receiver<ControlSignal>,
}
#[cfg(any(test,test_utilities))]
impl TestReceiver {
    pub fn recv(&self) -> Option<message::Message> {
        let sig = smol::block_on(self.control_chan.recv()).unwrap();
        match sig {
            // in this test, only return broadcast msg
            ControlSignal::BroadcastMessage(msg) => Some(msg),
            _ => None,
        }
    }
}

impl Handle {
    pub fn connect(&self, addr: std::net::SocketAddr) -> std::io::Result<peer::Handle> {
        let (sender, receiver) = oneshot::channel();
        smol::block_on(
            self.control_chan
                .send(ControlSignal::ConnectNewPeer(addr, sender)),
        )
            .unwrap();
        smol::block_on(receiver).unwrap()
    }

    pub fn broadcast(&self, msg: message::Message) {
        smol::block_on(self.control_chan.send(ControlSignal::BroadcastMessage(msg))).unwrap();
    }

    pub fn send(&self, receiver: Address, msg: message::Message) {
        smol::block_on(self.control_chan.send(ControlSignal::SendToPeer((receiver, msg)))).unwrap();
    }

    #[cfg(any(test,test_utilities))]
    pub fn new_for_test() -> (Handle, TestReceiver) {
        let (s,r) = smol::channel::unbounded();
        let h = Handle {control_chan: s};
        let t = TestReceiver {control_chan: r};
        (h,t)
    }
}

enum ControlSignal {
    ConnectNewPeer(
        std::net::SocketAddr,
        oneshot::Sender<std::io::Result<peer::Handle>>,
    ),
    BroadcastMessage(message::Message),
    GetNewPeer(Async<net::TcpStream>),
    DroppedPeer(std::net::SocketAddr),
    SendToPeer((Address,message::Message)),
}
