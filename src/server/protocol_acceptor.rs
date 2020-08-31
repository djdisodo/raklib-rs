pub trait ProtocolAcceptor {
	fn accepts(&self, version: u8) -> bool;
	fn get_primary_version(&self) -> u8;

}