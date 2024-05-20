use crate::coding::{Decode, DecodeError, Encode, EncodeError};

/// Sent by the subscriber to introduce packet loss.
#[derive(Clone, Debug)]
pub struct PacketLoss {
	// The packet loss rate.
	pub loss_rate: u64,
}

impl Decode for PacketLoss {
	fn decode<R: bytes::Buf>(r: &mut R) -> Result<Self, DecodeError> {
		let loss_rate: u64 = u64::decode(r)?;
		Ok(Self { loss_rate })
	}
}

impl Encode for PacketLoss {
	fn encode<W: bytes::BufMut>(&self, w: &mut W) -> Result<(), EncodeError> {
		self.loss_rate.encode(w)?;
		Ok(())
	}
}
