use crate::coding::{Decode, DecodeError, Encode, EncodeError};

/// Sent by the subscriber to change the gop size.
#[derive(Clone, Debug)]
pub struct SetGopSize {
	// The gop size.
	pub gop_size: String,
}

impl Decode for SetGopSize {
	fn decode<R: bytes::Buf>(r: &mut R) -> Result<Self, DecodeError> {
		let gop_size: String = String::decode(r)?;
		Ok(Self { gop_size })
	}
}

impl Encode for SetGopSize {
	fn encode<W: bytes::BufMut>(&self, w: &mut W) -> Result<(), EncodeError> {
		self.gop_size.encode(w)?;
		Ok(())
	}
}
