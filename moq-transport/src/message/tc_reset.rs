use crate::coding::{Decode, DecodeError, Encode, EncodeError};

/// Sent by the subscriber to reset tc/netem settings.
#[derive(Clone, Debug)]
pub struct TcReset {
}

impl Decode for TcReset {
	fn decode<R: bytes::Buf>(_r: &mut R) -> Result<Self, DecodeError> {
		Ok(Self {})
	}
}

impl Encode for TcReset {
	fn encode<W: bytes::BufMut>(&self, _w: &mut W) -> Result<(), EncodeError> {
		Ok(())
	}
}
