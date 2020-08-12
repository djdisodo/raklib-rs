use std::io::Read;
use crate::server::{ServerEventListener, ServerEvent};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

pub struct RaknetToUserThreadEventReceiver {
	channel: Arc<Mutex<VecDeque<ServerEvent>>>
}

impl RaknetToUserThreadEventReceiver {
	pub fn new(channel: Arc<Mutex<VecDeque<ServerEvent>>>) -> Self {
		Self {
			channel
		}
	}
	pub fn receive(&mut self) -> Option<ServerEvent> {
		self.channel.lock().unwrap().pop_back()
	}
}