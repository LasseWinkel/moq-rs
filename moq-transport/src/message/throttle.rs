use crate::coding::{Decode, DecodeError, Encode, EncodeError};

/// Sent by the subscriber to throttle the connection.
#[derive(Clone, Debug)]
pub struct Throttle {
}

impl Decode for Throttle {
	fn decode<R: bytes::Buf>(_r: &mut R) -> Result<Self, DecodeError> {
		Ok(Self {})
	}
}

impl Encode for Throttle {
	fn encode<W: bytes::BufMut>(&self, _w: &mut W) -> Result<(), EncodeError> {
		Ok(())
	}
}
