use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::server::ipc::UserToRaknetMessage;

pub struct UserToRaknetMessageReceiver {
	channel: Arc<Mutex<VecDeque<UserToRaknetMessage>>>
}

impl UserToRaknetMessageReceiver {
	pub fn new(channel: Arc<Mutex<VecDeque<UserToRaknetMessage>>>) -> Self {
		Self {
			channel
		}
	}
	pub fn receive(&mut self) -> Option<UserToRaknetMessage> {
		self.channel.lock().unwrap().pop_back()
	}
}