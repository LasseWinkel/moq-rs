use crate::coding::{Decode, DecodeError, Encode, EncodeError};

/// Sent by the subscriber to reset tc/netem settings.
#[derive(Clone, Debug)]
pub struct TcReset {
	// The network namespace.
	pub network_namespace: String,
}

impl Decode for TcReset {
	fn decode<R: bytes::Buf>(r: &mut R) -> Result<Self, DecodeError> {
		let network_namespace: String = String::decode(r)?;
		Ok(Self { network_namespace })
	}
}

impl Encode for TcReset {
	fn encode<W: bytes::BufMut>(&self, w: &mut W) -> Result<(), EncodeError> {
		self.network_namespace.encode(w)?;
		Ok(())
	}
}
