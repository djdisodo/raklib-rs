use crate::protocol::{Datagram, EncapsulatedPacket, PacketReliability, SplitPacketInfo};
use crate::generic::ReliableCacheEntry;
use std::collections::HashMap;
use std::mem::replace;

pub struct SendReliabilityLayer {
	send_datagram_callback: Box<dyn Fn(&mut Datagram) -> ()>,

	on_ack: Box<dyn Fn(u32) -> ()>,

	mtu_size: usize,

	send_queue: Vec<Box<EncapsulatedPacket>>,

	split_id: usize,

	send_seq_number: u32,

	message_index: u32,

	send_ordered_index: [u32; PacketReliability::MAX_ORDER_CHANNELS],
	send_sequenced_index: [u32; PacketReliability::MAX_ORDER_CHANNELS],

	resend_queue: Vec<Datagram>,

	reliable_cache: HashMap<u32, ReliableCacheEntry>, //TODO replace hashmap

	need_ack: HashMap<u64, HashMap<u32, u32>> //TODO replace hashmap
}

impl SendReliabilityLayer {
	pub fn new(
		mtu_size: usize,
		send_datagram: impl Fn(&mut Datagram) -> () + 'static,
		on_ack: impl Fn(u32) -> () + 'static
	) -> Self {
		Self {
			send_datagram_callback: Box::new(send_datagram),
			on_ack: Box::new(on_ack),
			mtu_size,
			send_queue: vec![],
			split_id: 0,
			send_seq_number: 0,
			message_index: 0,
			send_ordered_index: Default::default(),
			send_sequenced_index: Default::default(),
			resend_queue: vec![],
			reliable_cache: Default::default(),
			need_ack: Default::default()
		}
	}

	fn send_datagram(&mut self, mut datagram: Datagram) {
		if let Some(sequence_number) = datagram.sequence_number {
			self.reliable_cache.remove(&sequence_number);
		}
		datagram.sequence_number = Some(self.send_seq_number);
		self.send_seq_number += 1;
		(self.send_datagram_callback)(&mut datagram);

		let seq_number = datagram.sequence_number.unwrap();
		let mut resendable: Vec<Box<EncapsulatedPacket>> = datagram.packets.into_iter().filter(| x | x.reliability.is_reliable()).collect();
		if !resendable.is_empty() {
			self.reliable_cache.insert(seq_number, ReliableCacheEntry::new(resendable));
		}
	}

	pub fn send_queue(&mut self) {
		if !self.send_queue.is_empty() {
			let mut datagram = Datagram::default();
			datagram.packets = replace(&mut self.send_queue, vec![]);
			self.send_datagram(datagram);
		}
	}

	fn add_to_queue(&mut self, pk: EncapsulatedPacket, immediate: bool) {
		if let (Some(identifier_ack), Some(message_index)) = (pk.identifier_ack, pk.message_index) {
			self.need_ack.get_mut(&identifier_ack).unwrap().insert(message_index, message_index);
		}

		let mut length = Datagram::HEADER_SIZE;

		for queued in &self.send_queue {
			length += queued.get_total_length();
		}

		if length + pk.get_total_length() > self.mtu_size - 36 { //IP header (20 bytes) + UDP header (8 bytes) + RakNet weird (8 bytes) = 36 bytes
			self.send_queue()
		}

		if pk.identifier_ack.is_some() {
			self.send_queue.push(Box::new(pk));
			//pk.identifier_ack = None;
		} else {
			self.send_queue.push(Box::new(pk))
		}

		if immediate {
			// Forces pending sends to go out now, rather than waiting to the next update interval
			self.send_queue();
		}
	}

	pub fn add_encapsulated_to_queue(&mut self, mut packet: EncapsulatedPacket, immediate: bool) {
		if let Some(identifier_ack) = packet.identifier_ack {
			self.need_ack.insert(identifier_ack, Default::default());
		}

		if packet.reliability.is_ordered() {
			let order_channel = packet.order_channel.unwrap() as usize;
			packet.order_index = Some(self.send_ordered_index[order_channel]);
			self.send_ordered_index[order_channel] += 1;
		} else if packet.reliability.is_sequenced() {
			let order_channel = packet.order_channel.unwrap() as usize;
			packet.order_index = Some(self.send_ordered_index[order_channel]);
			packet.sequence_index = Some(self.send_sequenced_index[order_channel])
		}

		//IP header size (20 bytes) + UDP header size (8 bytes) + RakNet weird (8 bytes) + datagram header size (4 bytes) + max encapsulated packet header size (20 bytes)
		let max_size = self.mtu_size - 60;

		if packet.buffer.len() > max_size {
			let buffers: Vec<&[u8]> = packet.buffer.chunks(max_size).collect();
			let buffer_count = buffers.len() as u32;

			self.split_id += 1;
			let split_id = (self.split_id % 65536) as u16;
			let mut count = 0;
			for buffer in buffers {
				let mut pk = EncapsulatedPacket::default();
				pk.split_info = Some(SplitPacketInfo::new(split_id, count, buffer_count));
				pk.reliability = packet.reliability;
				pk.buffer.extend(buffer);

				if pk.reliability.is_reliable() {
					pk.message_index = Some(self.message_index);
					self.message_index += 1;
				}

				pk.sequence_index = packet.sequence_index;
				pk.order_channel = packet.order_channel;
				pk.order_index = packet.order_index;

				self.add_to_queue(pk, true);
			}
		} else {
			if packet.reliability.is_reliable() {
				packet.message_index = Some(self.message_index);
				self.message_index += 1;
			}
			self.add_to_queue(packet, false);
		}

	}


}