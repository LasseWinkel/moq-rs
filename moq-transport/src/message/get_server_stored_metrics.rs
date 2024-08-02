use crate::coding::{Decode, DecodeError, Encode, EncodeError};

/// Sent by the publisher to get current server stored metrics.
#[derive(Clone, Debug)]
pub struct GetServerStoredMetrics {}

impl Decode for GetServerStoredMetrics {
	fn decode<R: bytes::Buf>(_r: &mut R) -> Result<Self, DecodeError> {
		Ok(Self {})
	}
}

impl Encode for GetServerStoredMetrics {
	fn encode<W: bytes::BufMut>(&self, _w: &mut W) -> Result<(), EncodeError> {
		Ok(())
	}
}
