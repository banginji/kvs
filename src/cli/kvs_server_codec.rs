use std::io;

use bytes::{BufMut, Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder, LengthDelimitedCodec};

use super::kvs_command::KvsCliCommand;

pub struct KvsServerCodec {
    length_delimited: LengthDelimitedCodec
}

impl KvsServerCodec {
    pub fn new() -> Self {
        KvsServerCodec {
            length_delimited: LengthDelimitedCodec::new()
        }
    }
}

// read
impl Decoder for KvsServerCodec {
    type Item = KvsCliCommand;
    type Error = io::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(frame) = self.length_delimited.decode(src)? {
            Ok(Some(KvsCliCommand::from_bytes(&frame)))
        } else {
            Ok(None)
        }
    }
}

// write
impl Encoder<Bytes> for KvsServerCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Bytes, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let mut frame = BytesMut::new();
        frame.put_slice(&item);
        self.length_delimited.encode(frame.freeze(), dst)?;
        Ok(())
    }
}
