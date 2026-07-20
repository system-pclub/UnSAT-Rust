mod client;
mod error;
mod server;

pub mod buffer;
pub mod packet;

#[cfg(feature = "crypto")]
pub mod crypto;

use mio::net::TcpStream;
use std::io::Write;

pub use client::{Client, ClientEvent};
pub use error::{Error, Result};
pub use mio::Token;
pub use server::{Server, ServerEvent};

pub enum PacketRecipient {
    All,
    Single(Token),
    Exclude(Token),
    ExcludeMany(Vec<Token>),
    Include(Vec<Token>),
}

#[cfg(test)]
mod compile_only_tests {
    use crate::packet::{serialize_packet, PacketBody};
    use crate::Error;

    #[derive(Clone)]
    struct EmptyPacket;

    impl PacketBody for EmptyPacket {
        fn box_clone(&self) -> Box<dyn PacketBody> {
            Box::new(self.clone())
        }

        fn serialize(&self) -> Result<Vec<u8>, Error> {
            Ok(Vec::new())
        }

        fn deserialize(_data: &[u8]) -> Result<Self, Error> {
            Ok(Self)
        }

        fn id(&self) -> u8 {
            0
        }
    }

    #[test]
    #[ignore = "compile-only LLVM IR instantiation"]
    fn instantiate_buffer_drain_and_packet_serialization() {
        let mut buffer = crate::buffer::NetworkBuffer::new();
        buffer.drain(0);
        let _ = serialize_packet(Box::new(EmptyPacket));
    }
}

/// Send some bytes to a socket.
/// Returns the number of bytes sent, or an `Error`.
pub fn send_bytes(socket: &mut TcpStream, buffer: &[u8]) -> Result<usize> {
    let mut len = buffer.len();
    if len == 0 {
        return Err(Error::InvalidData);
    }

    // Keep sending until we've sent the entire buffer
    while len > 0 {
        match socket.write(buffer) {
            Ok(sent_bytes) => {
                len -= sent_bytes;
            }
            Err(_) => {
                return Err(Error::FailedToSendBytes);
            }
        }
    }

    Ok(buffer.len())
}
