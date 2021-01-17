#![feature(thread_spawn_unchecked)]
#[macro_use] extern crate derive_deref;
#[macro_use] extern crate num_enum;

use std::time::Duration;

pub mod generic;
pub mod protocol;
pub mod server;

pub const DEFAULT_PROTOCOL_VERSION: u8 = 6;
pub static SYSTEM_ADDRESS_COUNT: usize = 20;

pub type RaknetTime = Duration;

#[cfg(test)]
mod tests {
    use crate::protocol::{IncompatibleProtocolVersion, EncodePacket};
    use crate::server::{Server, ProtocolAcceptor, ServerEventListener, ServerInterface};
    use crate::server::ipc::{UserToRaknetMessageSender, UserToRaknetMessage, UserToRaknetMessageReceiver};
    use std::collections::VecDeque;
    use std::sync::Arc;
    use parking_lot::Mutex;
    use std::net::SocketAddr;
    use std::time::Duration;
    #[test]
    fn server() {
        env_logger::init();
        let chan: Arc<Mutex<VecDeque<UserToRaknetMessage>>> = Default::default();
        let socket = std::net::UdpSocket::bind("0.0.0.0:8000").unwrap();
        socket.set_nonblocking(false).unwrap();
        let mut buf = [0; 100];
        socket.recv_from(&mut buf).unwrap();
        println!("recv");
        let mut server = Server::new(
            0,
            socket,
            1500,
            PA {},
            UserToRaknetMessageReceiver::new(chan),
            EL {}
        );

        server.internal.lock().set_name("ddddd".to_owned());
        server.run();

    }
    
    struct EL;
    
    impl ServerEventListener for EL {
        fn on_client_connect(&mut self, session_id: usize, address: SocketAddr, client_id: u64) {
            
        }

        fn on_client_disconnect(&mut self, session_id: usize, reason: &str) {
            
        }

        fn on_packet_receive(&mut self, session_id: usize, packet: &[u8]) {
            
        }

        fn on_raw_packet_receive(&mut self, address: SocketAddr, payload: &[u8]) {
            
        }

        fn on_packet_ack(&mut self, session_id: usize, identifier_ack: u64) {
            
        }

        fn on_bandwidth_stats_update(&mut self, bytes_sent_diff: usize, bytes_received_diff: usize) {
            
        }

        fn on_ping_measure(&mut self, session_id: usize, latency: Duration) {
            
        }
    }

    struct PA;

    impl ProtocolAcceptor for PA {
        fn accepts(&self, version: u8) -> bool {
            true
        }

        fn get_primary_version(&self) -> u8 {
            1
        }
    }
}