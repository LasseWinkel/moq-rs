use crate::coding::{Decode, DecodeError, Encode, EncodeError};

/// Sent by the subscriber to throttle the connection.
#[derive(Clone, Debug)]
pub struct Throttle {
	// The packet loss rate.
	pub loss_rate: u64,

	// The packet delay.
	pub delay: u64,

	// The bandwidth limit.
	pub bandwidth_limit: String,

	// The network namespace.
	pub network_namespace: String,
}

impl Decode for Throttle {
	fn decode<R: bytes::Buf>(r: &mut R) -> Result<Self, DecodeError> {
		let loss_rate: u64 = u64::decode(r)?;
		let delay: u64 = u64::decode(r)?;
		let bandwidth_limit: String = String::decode(r)?;
		let network_namespace: String = String::decode(r)?;
		Ok(Self { loss_rate, delay, bandwidth_limit, network_namespace })
	}
}

impl Encode for Throttle {
	fn encode<W: bytes::BufMut>(&self, w: &mut W) -> Result<(), EncodeError> {
		self.loss_rate.encode(w)?;
		self.delay.encode(w)?;
		self.bandwidth_limit.encode(w)?;
		self.network_namespace.encode(w)?;
		Ok(())
	}
}
