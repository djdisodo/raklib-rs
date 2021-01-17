use crate::protocol::{Datagram, EncapsulatedPacket, PacketReliability, SplitPacketInfo, ACK, NACK, PacketImpl};
use crate::generic::ReliableCacheEntry;
use std::collections::{HashMap, VecDeque};
use std::mem::replace;
use std::time::{SystemTime, Duration};

pub struct SendReliabilityLayer<'a> {
	send_datagram_callback: Box<dyn Fn(&mut Datagram) -> () + Send + Sync + 'a>,

	on_ack: Box<dyn Fn(u64) -> () + Send + Sync + 'a>,

	mtu_size: usize,

	send_queue: Vec<Box<EncapsulatedPacket>>,

	split_id: usize,

	send_seq_number: u32,

	message_index: u32,

	send_ordered_index: [u32; PacketReliability::MAX_ORDER_CHANNELS],
	send_sequenced_index: [u32; PacketReliability::MAX_ORDER_CHANNELS],

	resend_queue: VecDeque<Datagram>,

	reliable_cache: HashMap<u32, ReliableCacheEntry>, //TODO replace hashmap

	need_ack: HashMap<u64, HashMap<u32, u32>> //TODO replace hashmap
}

impl<'a> SendReliabilityLayer<'a> {
	pub fn new(
		mtu_size: usize,
		send_datagram: impl Fn(&mut Datagram) -> () + Send + Sync + 'a,
		on_ack: impl Fn(u64) -> () + Send + Sync + 'a
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
			resend_queue: Default::default(),
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
		let resendable: Vec<Box<EncapsulatedPacket>> = datagram.packets.into_iter().filter(| x | x.reliability.is_reliable()).collect();
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

	pub fn add_encapsulated_to_queue(&mut self, mut packet: EncapsulatedPacket, _immediate: bool) {
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
			let count = 0;
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

	pub fn on_ack(&mut self, packet: &ACK) {
		for seq in &packet.packets {
			if let Some(reliable_cache) = self.reliable_cache.get(seq) {
				for pk in &reliable_cache.packets {
					if pk.identifier_ack.is_some() && pk.message_index.is_some() {
						self.need_ack.get_mut(&pk.identifier_ack.unwrap()).unwrap().remove(&pk.message_index.unwrap());
						if self.need_ack.get(&pk.identifier_ack.unwrap()).unwrap().is_empty() {
							self.need_ack.remove(&pk.identifier_ack.unwrap());
							(self.on_ack)(pk.identifier_ack.unwrap())
						}
					}
				}
			}
		}
	}

	pub fn on_nack(&mut self, packet: &NACK) {
		for seq in &packet.packets {
			if let Some(_reliable_cache) = self.reliable_cache.get(seq) {
				//TODO: group resends if the resulting datagram is below the MTU
				let mut resend = Datagram::default();
				resend.packets = self.reliable_cache.remove(&seq).unwrap().packets;
				self.resend_queue.push_back(resend);
			}
		}
	}

	pub fn needs_update(&self) -> bool {
		!self.send_queue.is_empty() &&
		!self.resend_queue.is_empty() &&
		!self.reliable_cache.is_empty()
	}

	pub fn update(&mut self) {
		if !self.resend_queue.is_empty() {
			let mut limit = 16;
			while let Some(pk) = self.resend_queue.pop_front() {
				self.send_datagram(pk);

				limit -= 1;
				if limit <= 0 {
					break;
				}
			}
		}

		let keys: Vec<u32> = self.reliable_cache.keys().map(| x | *x).collect();

		for seq in keys {
			if SystemTime::now().duration_since(self.reliable_cache[&seq].timestamp).unwrap() < Duration::from_secs(8) {
				let mut resend = Datagram::default();
				resend.packets = self.reliable_cache.remove(&seq).unwrap().packets;
				self.resend_queue.push_back(resend);
			} else {
				break;
			}
		}

		self.send_queue();
	}

	pub(crate) fn queue_connected_packet(
		&mut self,
		packet: &impl PacketImpl,
		reliability: PacketReliability,
		order_channel: u8,
		immediate: bool // default false
	) {
		let mut encapsulated = EncapsulatedPacket::default();
		encapsulated.reliability = reliability;
		encapsulated.order_channel = Some(order_channel);
		packet.encode_packet(&mut encapsulated.buffer);

		self.add_encapsulated_to_queue(encapsulated, immediate);
	}
}