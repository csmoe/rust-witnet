use std::io;

use actix::Message;
use bytes::{BytesMut, BigEndian, ByteOrder};
use log::info;
use tokio::codec::{Decoder, Encoder};

const HEADER_SIZE: usize = 2; // bytes

/// Message coming from the network
#[derive(Debug, Message, Eq, PartialEq, Clone)]
pub enum Request {
    /// Request message
    Message(BytesMut),
}

/// Message going to the network
#[derive(Debug, Message, Eq, PartialEq, Clone)]
pub enum Response {
    /// Response message
    Message(BytesMut),
}

/// Codec for client -> server transport
#[derive(Debug, Message, Eq, PartialEq, Clone)]
pub struct P2PCodec;

/// Implement decoder trait for P2P codec
///
impl Decoder for P2PCodec {
    type Item = Request;
    type Error = io::Error;

    /// Method to decode bytes to a request
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut ftb: Option<Self::Item> = None;
        let msg_len = src.len();
        if msg_len > 2 {
            let msg_size = BigEndian::read_u16(&src[..2]) as usize;
            if msg_len >= (msg_size + HEADER_SIZE) {
                // trim the header from given byte stream
                // [000 00100] => [00100]
                //  ^^^
                //  |-- header
                src.split_to(HEADER_SIZE);
                // unwrap message bytes from header-trimmed stream
                // [00100] => [001]
                //  ^^^-- message
                let msg = src.split_to(msg_size);
                ftb = Some(Request::Message(msg));
            }
        }
        Ok(ftb)
    }
}

/// Implement encoder trait for P2P codec
impl Encoder for P2PCodec {
    type Item = Response;
    type Error = io::Error;

    /// Method to encode a response into bytes
    fn encode(&mut self, msg: Response, dst: &mut BytesMut) -> Result<(), Self::Error> {
        info!("Encoding {:?}", msg);

        let Response::Message(bytes) = msg;

        let mut encoded_msg = vec![0; 2];
        let header: u16 = bytes.len() as u16;
        // push header with msg len
        BigEndian::write_u16(&mut encoded_msg, header);
        // push message
        encoded_msg.append(&mut bytes.to_vec());
        // push message to destination
        dst.unsplit(BytesMut::from(encoded_msg));
        Ok(())
    }
}
