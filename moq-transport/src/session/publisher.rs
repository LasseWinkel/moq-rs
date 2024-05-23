use std::{
	collections::{hash_map, HashMap},
	sync::{Arc, Mutex},
	process::Command,
	path::Path
};

use futures::{stream::FuturesUnordered, StreamExt};

use crate::{
	message::{self, Message},
	serve::{ServeError, TracksReader},
	setup,
};

use crate::watch::Queue;

use super::{Announce, AnnounceRecv, Session, SessionError, Subscribed, SubscribedRecv};

// TODO remove Clone.
#[derive(Clone)]
pub struct Publisher {
	webtransport: web_transport::Session,

	announces: Arc<Mutex<HashMap<String, AnnounceRecv>>>,
	subscribed: Arc<Mutex<HashMap<u64, SubscribedRecv>>>,
	unknown: Queue<Subscribed>,

	outgoing: Queue<Message>,
}

impl Publisher {
	pub(crate) fn new(outgoing: Queue<Message>, webtransport: web_transport::Session) -> Self {
		Self {
			webtransport,
			announces: Default::default(),
			subscribed: Default::default(),
			unknown: Default::default(),
			outgoing,
		}
	}

	pub async fn accept(session: web_transport::Session) -> Result<(Session, Publisher), SessionError> {
		let (session, publisher, _) = Session::accept_role(session, setup::Role::Publisher).await?;
		Ok((session, publisher.unwrap()))
	}

	pub async fn connect(session: web_transport::Session) -> Result<(Session, Publisher), SessionError> {
		let (session, publisher, _) = Session::connect_role(session, setup::Role::Publisher).await?;
		Ok((session, publisher.unwrap()))
	}

	/// Announce a namespace and serve tracks using the provided [serve::TracksReader].
	/// The caller uses [serve::TracksWriter] for static tracks and [serve::TracksRequest] for dynamic tracks.
	pub async fn announce(&mut self, tracks: TracksReader) -> Result<(), SessionError> {
		let mut announce = match self.announces.lock().unwrap().entry(tracks.namespace.clone()) {
			hash_map::Entry::Occupied(_) => return Err(ServeError::Duplicate.into()),
			hash_map::Entry::Vacant(entry) => {
				let (send, recv) = Announce::new(self.clone(), tracks.namespace.clone());
				entry.insert(recv);
				send
			}
		};

		let mut tasks = FuturesUnordered::new();
		let mut done = None;

		loop {
			tokio::select! {
				subscribe = announce.subscribed(), if done.is_none() => {
					let subscribe = match subscribe {
						Ok(Some(subscribe)) => subscribe,
						Ok(None) => { done = Some(Ok(())); continue },
						Err(err) => { done = Some(Err(err)); continue },
					};

					let tracks = tracks.clone();

					tasks.push(async move {
						let info = subscribe.info.clone();
						if let Err(err) = Self::serve_subscribe(subscribe, tracks).await {
							log::warn!("failed serving subscribe: {:?}, error: {}", info, err)
						}
					});
				},
				_ = tasks.next(), if !tasks.is_empty() => {},
				else => return Ok(done.unwrap()?)
			}
		}
	}

	pub async fn serve_subscribe(subscribe: Subscribed, mut tracks: TracksReader) -> Result<(), SessionError> {
		if let Some(track) = tracks.subscribe(&subscribe.name) {
			subscribe.serve(track).await?;
		} else {
			subscribe.close(ServeError::NotFound)?;
		}

		Ok(())
	}

	// Returns subscriptions that do not map to an active announce.
	pub async fn subscribed(&mut self) -> Option<Subscribed> {
		self.unknown.pop().await
	}

	pub(crate) fn recv_message(&mut self, msg: message::Subscriber) -> Result<(), SessionError> {
		let res = match msg {
			message::Subscriber::AnnounceOk(msg) => self.recv_announce_ok(msg),
			message::Subscriber::AnnounceError(msg) => self.recv_announce_error(msg),
			message::Subscriber::AnnounceCancel(msg) => self.recv_announce_cancel(msg),
			message::Subscriber::Subscribe(msg) => self.recv_subscribe(msg),
			message::Subscriber::Unsubscribe(msg) => self.recv_unsubscribe(msg),
			message::Subscriber::Throttle(msg) => self.recv_throttle(msg),
			message::Subscriber::PacketLoss(msg) => self.recv_packet_loss(msg),
			message::Subscriber::TcReset(msg) => self.recv_tc_reset(msg),
		};

		if let Err(err) = res {
			log::warn!("failed to process message: {}", err);
		}

		Ok(())
	}

	fn recv_announce_ok(&mut self, msg: message::AnnounceOk) -> Result<(), SessionError> {
		if let Some(announce) = self.announces.lock().unwrap().get_mut(&msg.namespace) {
			announce.recv_ok()?;
		}

		Ok(())
	}

	fn recv_announce_error(&mut self, msg: message::AnnounceError) -> Result<(), SessionError> {
		if let Some(announce) = self.announces.lock().unwrap().remove(&msg.namespace) {
			announce.recv_error(ServeError::Closed(msg.code))?;
		}

		Ok(())
	}

	fn recv_announce_cancel(&mut self, msg: message::AnnounceCancel) -> Result<(), SessionError> {
		if let Some(announce) = self.announces.lock().unwrap().remove(&msg.namespace) {
			announce.recv_error(ServeError::Cancel)?;
		}

		Ok(())
	}

	fn recv_subscribe(&mut self, msg: message::Subscribe) -> Result<(), SessionError> {
		let namespace = msg.track_namespace.clone();

		let subscribe = {
			let mut subscribes = self.subscribed.lock().unwrap();

			// Insert the abort handle into the lookup table.
			let entry = match subscribes.entry(msg.id) {
				hash_map::Entry::Occupied(_) => return Err(SessionError::Duplicate),
				hash_map::Entry::Vacant(entry) => entry,
			};

			let (send, recv) = Subscribed::new(self.clone(), msg);
			entry.insert(recv);

			send
		};

		// If we have an announce, route the subscribe to it.
		if let Some(announce) = self.announces.lock().unwrap().get_mut(&namespace) {
			return announce.recv_subscribe(subscribe).map_err(Into::into);
		}

		// Otherwise, put it in the unknown queue.
		// TODO Have some way to detect if the application is not reading from the unknown queue.
		if let Err(err) = self.unknown.push(subscribe) {
			// Default to closing with a not found error I guess.
			err.close(ServeError::NotFound)?;
		}

		Ok(())
	}

	fn recv_unsubscribe(&mut self, msg: message::Unsubscribe) -> Result<(), SessionError> {
		if let Some(subscribed) = self.subscribed.lock().unwrap().get_mut(&msg.id) {
			subscribed.recv_unsubscribe()?;
		}

		Ok(())
	}

	fn recv_throttle(&mut self, msg: message::Throttle) -> Result<(), SessionError> {
		// Path to bash script
		let script_path = Path::new("tc_scripts/throttle.sh");

		let loss_rate = msg.loss_rate.to_string();
		let delay = msg.delay.to_string();
		let bandwidth_limit = msg.bandwidth_limit;

		// Run the bash script
		Self::run_script(script_path, &[&loss_rate, &delay, &bandwidth_limit])?;

		Ok(())
	}

	fn recv_packet_loss(&mut self, msg: message::PacketLoss) -> Result<(), SessionError> {
		// Path to bash script
		let script_path = Path::new("tc_scripts/packet_loss.sh");

		let loss_rate = msg.loss_rate.to_string();

		// Run the bash script with the loss rate argument
		Self::run_script(script_path, &[&loss_rate])?;

		Ok(())
	}

	fn recv_tc_reset(&mut self, _msg: message::TcReset) -> Result<(), SessionError> {
		// Path to bash script
		let script_path = Path::new("tc_scripts/reset.sh");

		// Run the bash script
		Self::run_script(script_path, &[])?;

		Ok(())
	}

	fn run_script(script_path: &Path, args: &[&str]) -> Result<(), SessionError> {
		let output = Command::new("bash")
			.arg(script_path)
			.args(args)
			.output();

		match output {
			Ok(output) => {
				if !output.status.success() {
					eprintln!("Script failed with status: {}", output.status);
					eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
					return Err(SessionError::ScriptError);
				}
				println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
			}
			Err(e) => {
				eprintln!("Failed to execute script: {}", e);
				return Err(SessionError::ScriptExecutionError(e.to_string()));
			}
		}

		Ok(())
	}

	pub(super) fn send_message<T: Into<message::Publisher> + Into<Message>>(&mut self, msg: T) {
		let msg = msg.into();
		match &msg {
			message::Publisher::SubscribeDone(msg) => self.drop_subscribe(msg.id),
			message::Publisher::SubscribeError(msg) => self.drop_subscribe(msg.id),
			message::Publisher::Unannounce(msg) => self.drop_announce(msg.namespace.as_str()),
			_ => (),
		};

		self.outgoing.push(msg.into()).ok();
	}

	fn drop_subscribe(&mut self, id: u64) {
		self.subscribed.lock().unwrap().remove(&id);
	}

	fn drop_announce(&mut self, namespace: &str) {
		self.announces.lock().unwrap().remove(namespace);
	}

	pub(super) async fn open_uni(&mut self) -> Result<web_transport::SendStream, SessionError> {
		Ok(self.webtransport.open_uni().await?)
	}

	pub(super) async fn send_datagram(&mut self, data: bytes::Bytes) -> Result<(), SessionError> {
		Ok(self.webtransport.send_datagram(data).await?)
	}
}
