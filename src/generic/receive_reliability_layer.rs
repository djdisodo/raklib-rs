use std::collections::{VecDeque, HashMap};
use crate::protocol::{EncapsulatedPacket, PacketReliability, Datagram, Packet, Payload, PayloadExt};
use log::debug;
use std::ops::{RangeInclusive, RangeBounds};
use std::iter::repeat;
use crate::protocol::payload::{ACK, NACK};

pub struct ReceiveReliabilityLayer {
	on_recv: Box<dyn Fn(&mut EncapsulatedPacket) -> ()>,

	send_packet: Box<dyn Fn(Packet<dyn PayloadExt>) -> ()>,

	window_start: usize,
	window_end: usize,

	ack_queue: Vec<Option<u32>>,
	nack_queue: Vec<Option<u32>>,

	reliable_window_start: usize,
	reliable_window_end: usize,

	reliable_window: Vec<bool>, //originally hashmap; false = not set / true = set

	receive_ordered_index: [usize; PacketReliability::MAX_ORDER_CHANNELS],
	receive_sequenced_highest_index: [usize; PacketReliability::MAX_ORDER_CHANNELS],
	receive_ordered_packets: [VecDeque<Option<Box<EncapsulatedPacket>>>; PacketReliability::MAX_ORDER_CHANNELS],

	split_packets: HashMap<u16, Vec<Option<Box<EncapsulatedPacket>>>>,

	max_split_packet_part_count: usize,

	max_concurrent_split_packets: usize,
}

impl ReceiveReliabilityLayer {

	pub const WINDOW_SIZE: usize = 2048;

	pub fn new(
		on_recv: impl Fn(&mut EncapsulatedPacket) -> () + 'static,
		send_packet: impl Fn(Packet<dyn PayloadExt>) -> () + 'static
	) -> Self {
		Self::with_split_limit(
			on_recv,
			send_packet,
			usize::MAX,
			usize::MAX
		)
	}

	pub fn with_split_limit(
		on_recv: impl Fn(&mut EncapsulatedPacket) -> () + 'static,
		send_packet: impl Fn(Packet<dyn PayloadExt>) -> () + 'static,
		max_split_packet_part_count: usize,
		max_concurrent_split_packets: usize
	) -> Self {
		Self {
			on_recv: Box::new(on_recv) as _,
			send_packet: Box::new(send_packet),
			window_start: 0,
			window_end: Self::WINDOW_SIZE,
			ack_queue: Default::default(),
			nack_queue: Default::default(),
			reliable_window_start: 0,
			reliable_window_end: Self::WINDOW_SIZE,
			reliable_window: vec![false; Self::WINDOW_SIZE],
			receive_ordered_index: Default::default(),
			receive_sequenced_highest_index: Default::default(),
			receive_ordered_packets: Default::default(),
			split_packets: Default::default(),
			max_split_packet_part_count,
			max_concurrent_split_packets
		}
	}

	fn window_range(&self) -> RangeInclusive<usize> {
		self.window_start..=self.window_end
	}

	fn reliable_window_range(&self) -> RangeInclusive<usize> {
		self.reliable_window_start..=self.reliable_window_end
	}

	fn handle_encapsulated_packet_route(&mut self, packet: &mut EncapsulatedPacket) {
		(self.on_recv)(packet);
	}

	fn handle_split(&mut self, packet: EncapsulatedPacket) -> Option<EncapsulatedPacket> {
		let split_info = packet.split_info.as_ref().unwrap();
		let total_parts = split_info.get_total_part_count() as usize;
		let part_index = split_info.get_part_index() as usize;

		if
			total_parts >= self.max_split_packet_part_count ||
			part_index >= total_parts
		{
			debug!(
				"Invalid split packet part, too many parts or invalid split index (part index {}, part count {})",
				part_index,
				total_parts
			);
			return None;
		}

		let split_id = split_info.get_id();
		if self.split_packets.get(&split_id).is_none() {
			if self.split_packets.len() >= self.max_concurrent_split_packets {
				debug!("Ignored split packet part because reached concurrent split packet limit of self.max_concurrent_split_packets");
				return None;
			}
			self.split_packets.insert(split_id, vec![None; total_parts]);
		} else if self.split_packets[&split_id].len() != total_parts {
			debug!("Wrong split count {} for split packet $splitId, expected {}", total_parts, self.split_packets[&split_id].len());
			return None;
		}

		self.split_packets.get_mut(&split_id).unwrap()[part_index] = Some(Box::new(packet)); //wtf rust

		let mut total_len = 0;
		for packet in &self.split_packets[&split_id] {
			if let Some(packet) = packet {
				total_len += packet.buffer.len();
			} else {
				return None;
			}
		}

		let split_packets = self.split_packets.remove(&split_id).unwrap();

		let packet = split_packets[part_index].as_ref().unwrap();

		let mut pk = EncapsulatedPacket::default();
		pk.buffer = Vec::with_capacity(total_len);

		pk.reliability = packet.reliability;
		pk.message_index = packet.message_index;
		pk.sequence_index = packet.sequence_index;
		pk.order_index = packet.order_index;
		pk.order_channel = packet.order_channel;

		for packet in split_packets {
			pk.buffer.extend_from_slice(&packet.unwrap().buffer);
		}

		Some(pk)
	}

	fn handle_encapsulated_packet(&mut self, mut packet: EncapsulatedPacket) {
		if let Some(message_index) = packet.message_index.map(| i | i as usize) {
			if
				!self.reliable_window_range().contains(&message_index) ||
				self.reliable_window[message_index % Self::WINDOW_SIZE]
			{
				return;
			}

			self.reliable_window[message_index % Self::WINDOW_SIZE] = true;

			if message_index == self.reliable_window_start {
				while self.reliable_window[self.reliable_window_start % Self::WINDOW_SIZE] {
					self.reliable_window[self.reliable_window_start % Self::WINDOW_SIZE] = false;
					self.reliable_window_start += 1;
					self.reliable_window_end += 1;
				}
			}
		}

		packet = match self.handle_split(packet) {
			Some(packet) => packet,
			None => return
		};

		if
			(packet.reliability.is_sequenced() || packet.reliability.is_ordered()) &&
			packet.order_channel.unwrap() as usize >= PacketReliability::MAX_ORDER_CHANNELS
		{
			//TODO: this should result in peer banning
			debug!("Invalid packet, bad order channel (packet.order_channel)");
			return;
		}

		if packet.reliability.is_sequenced() {
			let sequence_index = packet.sequence_index.unwrap() as usize;
			let order_channel = packet.order_channel.unwrap() as usize;
			let order_index = packet.order_index.unwrap() as usize;
			if
				sequence_index < self.receive_sequenced_highest_index[order_channel] ||
				order_index < self.receive_ordered_index[order_channel]
			{
				//too old sequenced packet, discard it
				return;
			}

			self.receive_sequenced_highest_index[order_channel] = sequence_index + 1;
			self.handle_encapsulated_packet_route(&mut packet);
		} else if packet.reliability.is_ordered() {
			let order_channel = packet.order_channel.unwrap() as usize;
			let order_index = packet.order_index.unwrap() as usize;
			if order_index == self.receive_ordered_index[order_channel] {
				//this is the packet we expected to get next
				//Any ordered packet resets the sequence index to zero, so that sequenced packets older than this ordered
				//one get discarded. Sequenced packets also include (but don't increment) the order index, so a sequenced
				//packet with an order index less than this will get discarded
				self.receive_sequenced_highest_index[order_index] = 0;
				self.receive_ordered_index[order_channel] = order_index + 1;

				self.handle_encapsulated_packet_route(&mut packet);
				while let Some(Some(_)) = self.receive_ordered_packets[order_channel].front_mut() {
					let mut pk = self.receive_ordered_packets[order_channel].pop_front().unwrap().unwrap();
					self.handle_encapsulated_packet_route(pk.as_mut());
					self.receive_ordered_index[order_channel] += 1;
				}
			} else if order_index > self.receive_ordered_index[order_channel] {
				self.receive_ordered_packets[order_channel].extend(repeat(None).take(order_index - order_channel - 1));
				self.receive_ordered_packets[order_channel].push_back(Some(Box::new(packet)));
			} else {
				//duplicate/already received packet
			}
		} else {
			//not ordered or sequenced
			self.handle_encapsulated_packet_route(&mut packet);
		}
	}

	pub fn on_datagram(&mut self, packet: &mut Datagram) {
		let sequence_number = packet.sequence_number as usize;
		if
			self.window_range().contains(&sequence_number) ||
			self.ack_queue.get(sequence_number).map(| x | x.is_some()).unwrap_or(false)
		{
			debug!("Received duplicate or out-of-window packet (sequence number $packet->seqNumber, window {}-{})", self.window_start, self.window_end);
			return;
		}

		self.nack_queue.get_mut(sequence_number).map(| x | *x = None);
		if self.ack_queue.len() - 1 < sequence_number {
			let diff = sequence_number - self.ack_queue.len() - 1;
			self.ack_queue.extend(repeat(None).take(diff));
		}
		self.ack_queue[sequence_number] = Some(sequence_number as u32);

		if sequence_number == self.window_start {
			//got a contiguous packet, shift the receive window
			//this packet might complete a sequence of out-of-order packets, so we incrementally check the indexes
			//to see how far to shift the window, and stop as soon as we either find a gap or have an empty window
			while self.ack_queue.get(self.window_start).is_some() {
				self.window_start += 1;
				self.window_end += 1;
			}
		} else if sequence_number > self.window_start {
			//we got a gap - a later packet arrived before earlier ones did
			//we add the earlier ones to the NACK queue
			//if the missing packets arrive before the end of tick, they'll be removed from the NACK queue
			self.nack_queue = vec![None; sequence_number];
			for i in self.window_start..sequence_number {
				if self.ack_queue[i].is_none() {
					self.nack_queue[i] = Some(i as u32);
				}
			}
		} else {
			panic!("received packet before window start");
		}

		for mut pk in &mut packet.packets {
			self.handle_encapsulated_packet_route(&mut pk);
		}
	}

	pub fn update(&mut self) {
		let diff = self.ack_queue.len() - self.window_start;
		assert!(diff >= 0);
		if diff > 0 {
			//Move the receive window to account for packets we either received or are about to NACK
			//we ignore any sequence numbers that we sent NACKs for, because we expect the client to resend them
			//when it gets a NACK for it

			self.window_start += diff;
			self.window_end += diff;
		}

		if !self.ack_queue.is_empty() {
			let mut pk = Packet::<ACK>::default();
			pk.packets = self.ack_queue.iter().filter_map(| x | x.to_owned()).collect();
			(self.send_packet)(pk.into_dyn());
			self.ack_queue.clear();
		}

		if !self.nack_queue.is_empty() {
			let mut pk = Packet::<NACK>::default();
			pk.packets = self.nack_queue.iter().filter_map(| x | x.to_owned()).collect();
			(self.send_packet)(pk.into_dyn());
			self.nack_queue.clear();
		}
	}

}