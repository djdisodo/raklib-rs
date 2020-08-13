pub enum UserToRaknetMessage {
	Encapsulated(Encapsulated),
	CloseSession(CloseSession),
	Raw(Raw),
	BlockAddress(BlockAddress),
	UnblockAddress(SocketAddr),
	RawFilter(Regex),
	SetName(String),
	EnablePortCheck,
	DisablePortCheck
	SetPacketsPerTickLimit(usize),
	Shutdown
}
