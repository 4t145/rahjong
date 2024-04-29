use bytes::Bytes;
use futures_util::{Sink, Stream};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite};
fn main() {
    println!("Hello, world!");
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConnectionId(u64);
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SerialNumber(u64);
#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    pld: Payload,
    ser: SerialNumber,
    con: ConnectionId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum Encoding {
    Bincode = 0,
    Json = 1,
}

impl TryFrom<u8> for Encoding {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Encoding::Bincode),
            1 => Ok(Encoding::Json),
            _ => Err("Invalid encoding"),
        }
    }
}

impl From<Encoding> for u8 {
    fn from(val: Encoding) -> Self {
        match val {
            Encoding::Bincode => 0,
            Encoding::Json => 1,
        }
    }
}

pub struct Connection<I, O> {
    pull: I,
    push: O,
    ser: SerialNumber,
    id: ConnectionId,
}

pub struct TcpStreamPull {
    inner: tokio::net::tcp::OwnedReadHalf,
}

impl Stream for TcpStreamPull {
    type Item = Result<Packet, std::io::Error>;
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<Packet, std::io::Error>>> {
        todo!()
    }
}

impl<I, O> Connection<I, O>
where
    I: Stream<Item = Result<Packet, std::io::Error>> + Unpin,
    O: Sink<Packet, Error = std::io::Error> + Unpin,
{
    pub fn new(pull: I, push: O, id: ConnectionId) -> Self {
        Connection {
            pull,
            push,
            ser: SerialNumber(0),
            id,
        }
    }
    pub async fn send(&mut self, pld: Payload) {
        use futures_util::SinkExt;
        let packet = Packet {
            pld,
            ser: self.ser,
            con: self.id,
        };
        self.ser.0 += 1;
        self.push.send(packet).await.unwrap();
    }
    pub async fn recv(&mut self) -> Payload {
        use futures_util::StreamExt;
        let packet = self.pull.next().await.unwrap().unwrap();
        packet.pld
    }
}
