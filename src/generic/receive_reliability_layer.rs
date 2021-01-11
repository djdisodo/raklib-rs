use std::collections::VecDeque;
use crate::protocol::{EncapsulatedPacket, PacketReliability};

pub struct ReceiveReliabilityLayer {
	on_recv: Box<dyn Fn(&mut EncapsulatedPacket) -> ()>,

	send_packet: Box<dyn Fn() -> ()>,

	window_start: usize,
	window_end: usize,

	highest_seq_number: isize,

	ack_queue: VecDeque<usize>,
	nack_queue: VecDeque<usize>,

	reliable_window_start: usize,
	reliable_window_end: usize,

	reliable_window: Vec<bool>,

	receive_ordered_index: [usize; PacketReliability::MAX_ORDER_CHANNELS],
	receive_sequenced_highest_index: [usize; PacketReliability::MAX_ORDER_CHANNELS],
	receive_ordered_packets: [Vec<Box<EncapsulatedPacket>>; PacketReliability::MAX_ORDER_CHANNELS],

	split_packets: Vec<Vec<Option<Box<EncapsulatedPacket>>>>,

	max_split_packet_part_count: usize,

	max_concurrent_split_packets: usize,
}

impl ReceiveReliabilityLayer {

	pub const WINDOW_SIZE: usize = 2048; //todo put rwlock

	pub fn new(
		on_recv: impl Fn(&mut EncapsulatedPacket) -> () + 'static,
		send_packet: impl Fn() -> () + 'static
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
		send_packet: impl Fn() -> () + 'static,
		max_split_packet_part_count: usize,
		max_concurrent_split_packets: usize
	) -> Self {
		Self {
			on_recv: Box::new(on_recv) as _,
			send_packet: Box::new(send_packet) as _,
			window_start: 0,
			window_end: Self::WINDOW_SIZE,
			highest_seq_number: -1,
			ack_queue: Default::default(),
			nack_queue: Default::default(),
			reliable_window_start: 0,
			reliable_window_end: Self::WINDOW_SIZE,
			reliable_window: vec![],
			receive_ordered_index: Default::default(),
			receive_sequenced_highest_index: Default::default(),
			receive_ordered_packets: Default::default(),
			split_packets: vec![],
			max_split_packet_part_count,
			max_concurrent_split_packets
		}
	}

	fn handle_encapsulated_packet_route(&mut self, packet: &mut EncapsulatedPacket) {
		(self.on_recv)(packet);
	}


}