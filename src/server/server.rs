use std::net::UdpSocket;
use log4rs::Logger;

pub struct Server {

	socket: UdpSocket,

	logger: Logger,

	server_id: u64,

	receive_bytes: usize,
	send_bytes: usize
}

// TODO low priority