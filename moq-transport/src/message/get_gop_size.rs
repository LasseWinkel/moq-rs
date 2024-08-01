use crate::coding::{Decode, DecodeError, Encode, EncodeError};

/// Sent by the publisher to calculate the latency.
#[derive(Clone, Debug)]
pub struct GetGopSize {}

impl Decode for GetGopSize {
	fn decode<R: bytes::Buf>(r: &mut R) -> Result<Self, DecodeError> {
		Ok(Self {})
	}
}

impl Encode for GetGopSize {
	fn encode<W: bytes::BufMut>(&self, w: &mut W) -> Result<(), EncodeError> {
		Ok(())
	}
}
