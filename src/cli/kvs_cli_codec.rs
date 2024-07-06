use std::io;

use bytes::{BufMut, Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder, LengthDelimitedCodec};

use super::kvs_command::KvsCliCommand;

pub struct KvsCliCodec {
    length_delimited: LengthDelimitedCodec
}

impl KvsCliCodec {
    pub fn new() -> Self {
        KvsCliCodec {
            length_delimited: LengthDelimitedCodec::new()
        }
    }
}

// read
impl Decoder for KvsCliCodec {
    type Item = Bytes;
    type Error = io::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(frame) = self.length_delimited.decode(src)? {
            Ok(Some(frame.into()))
        } else {
            Ok(None)
        }
    }
}

// send
impl Encoder<KvsCliCommand> for KvsCliCodec {
    type Error = io::Error;

    fn encode(&mut self, item: KvsCliCommand, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let bytes = item.to_bytes();
        let mut frame = BytesMut::new();
        frame.put_slice(&bytes);
        self.length_delimited.encode(frame.freeze(), dst)?;
        Ok(())
    }
}
