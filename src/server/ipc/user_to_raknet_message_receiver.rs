use std::sync::Arc;
use std::collections::VecDeque;
use crate::server::ipc::UserToRaknetMessage;
use parking_lot::Mutex;

pub struct UserToRaknetMessageReceiver {
	channel: Arc<Mutex<VecDeque<UserToRaknetMessage>>>
}

impl UserToRaknetMessageReceiver {
	pub fn new(channel: Arc<Mutex<VecDeque<UserToRaknetMessage>>>) -> Self {
		Self {
			channel
		}
	}
	pub fn receive(&self) -> Option<UserToRaknetMessage> {
		self.channel.lock().pop_back()
	}
}