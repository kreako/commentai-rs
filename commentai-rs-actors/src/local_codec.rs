use actix::Message;
use actix_codec::Encoder;
use byteorder::{BigEndian, ByteOrder};
use bytes::{BufMut, BytesMut};
use commentai_rs_data::Comment;
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::io;
use tokio_util::codec::Decoder;

/// Very simple protocol :
/// 2 bytes of size of the payload
/// payload : AdminRequest or AdminResponse encoded as json

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
/// Requested by the admin interface
pub enum AdminRequest {
    /// List all the comment with an optional filter
    List { filter: Option<String> },
    /// Delete a comment based on id
    Delete { id: i32 },
}

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
/// Answers send back to admin interface
pub enum AdminResponse {
    /// List of comments, with the optional filter used
    List {
        filter: Option<String>,
        comments: Vec<Comment>,
    },
    /// The comment with this id was deleted
    Delete { id: i32 },
}

/// Decode AdminRequest, encode AdminResponse
/// Used by LocalTcpActor
pub struct LocalTcpCodec;

impl Decoder for LocalTcpCodec {
    type Item = AdminRequest;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {
            if src.len() < 2 {
                // not enough input to read the size
                return Ok(None);
            }
            BigEndian::read_u16(src.as_ref()) as usize
        };

        if src.len() >= size + 2 {
            // Discard the 1st 2 bytes (size of payload)
            let _ = src.split_to(2);
            let buf = src.split_to(size);
            // Decode payload from buf
            Ok(Some(json::from_slice::<AdminRequest>(&buf)?))
        } else {
            // not enough input to read the whole payload
            Ok(None)
        }
    }
}

impl Encoder for LocalTcpCodec {
    type Item = AdminResponse;
    type Error = io::Error;

    fn encode(&mut self, msg: AdminResponse, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // msg to json
        let msg = json::to_string(&msg).unwrap();
        // json as &[u8]
        let msg_ref: &[u8] = msg.as_ref();

        // Reserve 2 bytes (size) + payload
        dst.reserve(msg_ref.len() + 2);
        // Write size
        dst.put_u16(msg_ref.len() as u16);
        // Write payload
        dst.put(msg_ref);

        Ok(())
    }
}

/// Decode AdminResponse, encode AdminRequest
/// Used by command line
pub struct CliCodec;

impl Decoder for CliCodec {
    type Item = AdminResponse;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {
            if src.len() < 2 {
                return Ok(None);
            }
            BigEndian::read_u16(src.as_ref()) as usize
        };

        if src.len() >= size + 2 {
            let _ = src.split_to(2);
            let buf = src.split_to(size);
            Ok(Some(json::from_slice::<AdminResponse>(&buf)?))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for CliCodec {
    type Item = AdminRequest;
    type Error = io::Error;

    fn encode(&mut self, msg: AdminRequest, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg = json::to_string(&msg).unwrap();
        let msg_ref: &[u8] = msg.as_ref();

        dst.reserve(msg_ref.len() + 2);
        dst.put_u16(msg_ref.len() as u16);
        dst.put(msg_ref);

        Ok(())
    }
}
