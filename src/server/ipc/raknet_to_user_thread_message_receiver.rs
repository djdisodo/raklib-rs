use std::io::Read;
use crate::server::ServerEventListener;

pub struct RaknetToUserThreadMessageReceiver<T: Read> {
	channel: T
}

impl<T: Read> RaknetToUserThreadMessageReceiver<T> {
	pub fn new(channel: T) -> Self {
		Self {
			channel
		}
	}
	pub fn handle<L: ServerEventListener>(&mut self, listener: L) {
		unimplemented!()
	}
}