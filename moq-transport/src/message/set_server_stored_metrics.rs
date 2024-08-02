use crate::coding::{Decode, DecodeError, Encode, EncodeError};

/// Sent by the subscriber to change the server stored metrics.
#[derive(Clone, Debug)]
pub struct SetServerStoredMetrics {
	// The gop size.
	pub gop_size: String,

	// The bitrate mode.
	pub bitrate_mode: String,

	// The bitrate.
	pub bitrate: u64,
}

impl Decode for SetServerStoredMetrics {
	fn decode<R: bytes::Buf>(r: &mut R) -> Result<Self, DecodeError> {
		let gop_size: String = String::decode(r)?;
		let bitrate_mode: String = String::decode(r)?;
		let bitrate: u64 = u64::decode(r)?;
		Ok(Self {
			gop_size,
			bitrate_mode,
			bitrate,
		})
	}
}

impl Encode for SetServerStoredMetrics {
	fn encode<W: bytes::BufMut>(&self, w: &mut W) -> Result<(), EncodeError> {
		self.gop_size.encode(w)?;
		self.bitrate_mode.encode(w)?;
		self.bitrate.encode(w)?;
		Ok(())
	}
}
