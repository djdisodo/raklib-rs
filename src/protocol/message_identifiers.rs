pub fn read_from(buffer: &[u8]) -> u8 {
	buffer[0]
}

pub enum MessageIdentifiers {
	//From https://github.com/OculusVR/RakNet/blob/master/Source/MessageIdentifiers.h

	//
	// RESERVED TYPES - DO NOT CHANGE THESE
	// All types from RakPeer
	//
	/// These types are never returned to the user.
	/// Ping from a connected system.  Update timestamps (internal use only)
	ConnectedPing = 0x00,
	/// Ping from an unconnected system.  Reply but do not update timestamps. (internal use only)
	UnconnectedPing = 0x01,
	/// Ping from an unconnected system.  Only reply if we have open connections. Do not update timestamps. (internal use only)
	UnconnectedPingOpenConnections = 0x02,
	/// Pong from a connected system.  Update timestamps (internal use only)
	ConnectedPong = 0x03,
	/// A reliable packet to detect lost connections (internal use only)
	DetectLostConnections = 0x04,
	/// C2S: Initial query: Header(1), OfflineMesageID(16), Protocol number(1), Pad(toMTU), sent with no fragment set.
	/// If protocol fails on server, returns IdIncompatibleProtocolVersion to client
	OpenConnectionRequest1 = 0x05,
	/// S2C: Header(1), OfflineMesageID(16), server GUID(8), HasSecurity(1), Cookie(4, if HasSecurity)
	/// , pub key (if do security is true), MTU(2). If pub key fails on client, returns IdPubKeyMismatch
	OpenConnectionReply1 = 0x06,
	/// C2S: Header(1), OfflineMesageID(16), Cookie(4, if HasSecurity is true on the server), clientSupportsSecurity(1 bit),
	/// handshakeChallenge (if has security on both server and client), remoteBindingAddress(6), MTU(2), client GUID(8)
	/// Connection slot allocated if cookie is valid, server is not full, GUID and IP not already in use.
	OpenConnectionRequest2 = 0x07,
	/// S2C: Header(1), OfflineMesageID(16), server GUID(8), mtu(2), doSecurity(1 bit), handshakeAnswer (if do security is true)
	OpenConnectionReply2 = 0x08,
	/// C2S: Header(1), GUID(8), Timestamp, HasSecurity(1), Proof(32)
	ConnectionRequest = 0x09,
	/// RakPeer - Remote system requires secure connections, pass a pub key to RakPeerInterface::Connect()
	RemoteSystemRequiresPubKey = 0x0a,
	/// RakPeer - We passed a pub key to RakPeerInterface::Connect(), but the other system did not have security turned on
	OurSystemRequiresSecurity = 0x0b,
	/// RakPeer - Wrong pub key passed to RakPeerInterface::Connect()
	PubKeyMismatch = 0x0c,
	/// RakPeer - Same as IdAdvertiseSystem, but intended for internal use rather than being passed to the user.
	/// Second byte indicates type. Used currently for NAT punchthrough for receiver port advertisement. See ID_NAT_ADVERTISE_RECIPIENT_PORT
	OutOfBandInternal = 0x0d,
	/// If RakPeerInterface::Send() is called where PacketReliability contains _WITH_ACK_RECEIPT, then on a later call to
	/// RakPeerInterface::Receive() you will get IdSndReceiptAcked or IdSndReceiptLoss. The message will be 5 bytes long,
	/// and bytes 1-4 inclusive will contain a number in native order containing a number that identifies this message.
	/// This number will be returned by RakPeerInterface::Send() or RakPeerInterface::SendList(). IdSndReceiptAcked means that
	/// the message arrived
	SndReceiptAcked = 0x0e,
	/// If RakPeerInterface::Send() is called where PacketReliability contains UnreliableWithAckReceipt, then on a later call to
	/// RakPeerInterface::Receive() you will get IdSndReceiptAcked or IdSndReceiptLoss. The message will be 5 bytes long,
	/// and bytes 1-4 inclusive will contain a number in native order containing a number that identifies this message. This number
	/// will be returned by RakPeerInterface::Send() or RakPeerInterface::SendList(). IdSndReceiptLoss means that an ack for the
	/// message did not arrive (it may or may not have been delivered, probably not). On disconnect or shutdown, you will not get
	/// IdSndReceiptLoss for unsent messages, you should consider those messages as all lost.
	SndReceiptLoss = 0x0f,

	//
	// USER TYPES - DO NOT CHANGE THESE
	//

	/// RakPeer - In a client/server environment, our connection request to the server has been accepted.
	ConnectionRequestAccepted = 0x10,
	/// RakPeer - Sent to the player when a connection request cannot be completed due to inability to connect.
	ConnectionAttemptFailed = 0x11,
	/// RakPeer - Sent a connect request to a system we are currently connected to.
	AlreadyConnected = 0x12,
	/// RakPeer - A remote system has successfully connected.
	NewIncomingConnection = 0x13,
	/// RakPeer - The system we attempted to connect to is not accepting new connections.
	NoFreeIncomingConnections = 0x14,
	/// RakPeer - The system specified in Packet::systemAddress has disconnected from us.  For the client, this would mean the
	/// server has shutdown.
	DisconnectionNotification = 0x15,
	/// RakPeer - Reliable packets cannot be delivered to the system specified in Packet::systemAddress.  The connection to that
	/// system has been closed.
	ConnectionLost = 0x16,
	/// RakPeer - We are banned from the system we attempted to connect to.
	ConnectionBanned = 0x17,
	/// RakPeer - The remote system is using a password and has refused our connection because we did not set the correct password.
	InvalidPassword = 0x18,
	// RAKNET_PROTOCOL_VERSION in RakNetVersion.h does not match on the remote system what we have on our system
	// This means the two systems cannot communicate.
	// The 2nd byte of the message contains the value of RAKNET_PROTOCOL_VERSION for the remote system
	IncompatibleProtocolVersion = 0x19,
	// Means that this IP address connected recently, and can't connect again as a security measure. See
	/// RakPeer::SetLimitIPConnectionFrequency()
	IpRecentlyConnected = 0x1a,
	/// RakPeer - The sizeof(RakNetTime) bytes following this byte represent a value which is automatically modified by the difference
	/// in system times between the sender and the recipient. Requires that you call SetOccasionalPing.
	Timestamp = 0x1b,
	/// RakPeer - Pong from an unconnected system.  First byte is IdUnconnectedPong, second sizeof(RakNet::TimeMS) bytes is the ping,
	/// following bytes is system specific enumeration data.
	/// Read using bitstreams
	UnconnectedPong = 0x1c,
	/// RakPeer - Inform a remote system of our IP/Port. On the recipient, all data past IdAdvertiseSystem is whatever was passed to
	/// the data parameter
	AdvertiseSystem = 0x1d,
	// RakPeer - Downloading a large message. Format is IdDownloadProgress (MessageID), partCount (unsigned int),
	///  partTotal (unsigned int),
	/// partLength (unsigned int), first part data (length <= MAX_MTU_SIZE). See the three parameters partCount, partTotal
	///  and partLength in OnFileProgress in FileListTransferCBInterface.h
	DownloadProgress = 0x1e,

	/// ConnectionGraph2 plugin - In a client/server environment, a client other than ourselves has disconnected gracefully.
	///   Packet::systemAddress is modified to reflect the systemAddress of this client.
	RemoteDisconnectionNotification = 0x1f,
	/// ConnectionGraph2 plugin - In a client/server environment, a client other than ourselves has been forcefully dropped.
	///  Packet::systemAddress is modified to reflect the systemAddress of this client.
	RemoteConnectionLost = 0x20,
	/// ConnectionGraph2 plugin: Bytes 1-4 = count. for (count items) contains {SystemAddress, RakNetGUID, 2 byte ping}
	RemoteNewIncomingConnection = 0x21,

	/// FileListTransfer plugin - Setup data
	FileListTransferHeader = 0x22,
	/// FileListTransfer plugin - A file
	FileListTransferFile = 0x23,
	// Ack for reference push, to send more of the file
	FileListReferencePushAck = 0x24,

	/// DirectoryDeltaTransfer plugin - Request from a remote system for a download of a directory
	DdtDownloadRequest = 0x25,

	/// RakNetTransport plugin - Transport provider message, used for remote console
	TransportString = 0x26,

	/// ReplicaManager plugin - Create an object
	ReplicaManagerConstruction = 0x27,
	/// ReplicaManager plugin - Changed scope of an object
	ReplicaManagerScopeChange = 0x28,
	/// ReplicaManager plugin - Serialized data of an object
	ReplicaManagerSerialize = 0x29,
	/// ReplicaManager plugin - New connection, about to send all world objects
	ReplicaManagerDownloadStarted = 0x2a,
	/// ReplicaManager plugin - Finished downloading all serialized objects
	ReplicaManagerDownloadComplete = 0x2b,

	/// RakVoice plugin - Open a communication channel
	RakvoiceOpenChannelRequest = 0x2c,
	/// RakVoice plugin - Communication channel accepted
	RakvoiceOpenChannelReply = 0x2d,
	/// RakVoice plugin - Close a communication channel
	RakvoiceCloseChannel = 0x2e,
	/// RakVoice plugin - Voice data
	RakvoiceData = 0x2f,

	/// Autopatcher plugin - Get a list of files that have changed since a certain date
	AutopatcherGetChangelistSinceDate = 0x30,
	/// Autopatcher plugin - A list of files to create
	AutopatcherCreationList = 0x31,
	/// Autopatcher plugin - A list of files to delete
	AutopatcherDeletionList = 0x32,
	/// Autopatcher plugin - A list of files to get patches for
	AutopatcherGetPatch = 0x33,
	/// Autopatcher plugin - A list of patches for a list of files
	AutopatcherPatchList = 0x34,
	/// Autopatcher plugin - Returned to the user: An error from the database repository for the autopatcher.
	AutopatcherRepositoryFatalError = 0x35,
	/// Autopatcher plugin - Returned to the user: The server does not allow downloading unmodified game files.
	AutopatcherCannotDownloadOriginalUnmodifiedFiles = 0x36,
	/// Autopatcher plugin - Finished getting all files from the autopatcher
	AutopatcherFinishedInternal = 0x37,
	AutopatcherFinished = 0x38,
	/// Autopatcher plugin - Returned to the user: You must restart the application to finish patching.
	AutopatcherRestartApplication = 0x39,

	/// NATPunchthrough plugin: internal
	NatPunchthroughRequest = 0x3a,
	/// NATPunchthrough plugin: internal
	//ID_NAT_GROUP_PUNCHTHROUGH_REQUEST,
	/// NATPunchthrough plugin: internal
	//ID_NAT_GROUP_PUNCHTHROUGH_REPLY,
	/// NATPunchthrough plugin: internal
	NatConnectAtTime = 0x3b,
	/// NATPunchthrough plugin: internal
	NatGetMostRecentPort = 0x3c,
	/// NATPunchthrough plugin: internal
	NatClientReady = 0x3d,
	/// NATPunchthrough plugin: internal
	//ID_NAT_GROUP_PUNCHTHROUGH_FAILURE_NOTIFICATION,

	/// NATPunchthrough plugin: Destination system is not connected to the server. Bytes starting at offset 1 contains the
	///  RakNetGUID destination field of NatPunchthroughClient::OpenNAT().
	NatTargetNotConnected = 0x3e,
	/// NATPunchthrough plugin: Destination system is not responding to IdNatGetMostRecentPort. Possibly the plugin is not installed.
	///  Bytes starting at offset 1 contains the RakNetGUID  destination field of NatPunchthroughClient::OpenNAT().
	NatTargetUnresponsive = 0x3f,
	/// NATPunchthrough plugin: The server lost the connection to the destination system while setting up punchthrough.
	///  Possibly the plugin is not installed. Bytes starting at offset 1 contains the RakNetGUID  destination
	///  field of NatPunchthroughClient::OpenNAT().
	NatConnectionToTargetLost = 0x40,
	/// NATPunchthrough plugin: This punchthrough is already in progress. Possibly the plugin is not installed.
	///  Bytes starting at offset 1 contains the RakNetGUID destination field of NatPunchthroughClient::OpenNAT().
	NatAlreadyInProgress = 0x41,
	/// NATPunchthrough plugin: This message is generated on the local system, and does not come from the network.
	///  packet::guid contains the destination field of NatPunchthroughClient::OpenNAT(). Byte 1 contains 1 if you are the sender, 0 if not
	NatPunchthroughFailed = 0x42,
	/// NATPunchthrough plugin: Punchthrough succeeded. See packet::systemAddress and packet::guid. Byte 1 contains 1 if you are the sender,
	///  0 if not. You can now use RakPeer::Connect() or other calls to communicate with this system.
	NatPunchthroughSucceeded = 0x43,

	/// ReadyEvent plugin - Set the ready state for a particular system
	/// First 4 bytes after the message contains the id
	ReadyEventSet = 0x44,
	/// ReadyEvent plugin - Unset the ready state for a particular system
	/// First 4 bytes after the message contains the id
	ReadyEventUnset = 0x45,
	/// All systems are in state IdReadyEventSet
	/// First 4 bytes after the message contains the id
	ReadyEventAllSet = 0x46,
	/// \internal, do not process in your game
	/// ReadyEvent plugin - Request of ready event state - used for pulling data when newly connecting
	ReadyEventQuery = 0x47,

	/// Lobby packets. Second byte indicates type.
	LobbyGeneral = 0x48,

	// RPC3, RPC4 error
	RpcRemoteError = 0x49,
	/// Plugin based replacement for RPC system
	RpcPlugin = 0x4a,

	/// FileListTransfer transferring large files in chunks that are read only when needed, to save memory
	FileListReferencePush = 0x4b,
	/// Force the ready event to all set
	ReadyEventForceAllSet = 0x4c,

	/// Rooms function
	RoomsExecuteFunc = 0x4d,
	RoomsLogonStatus = 0x4e,
	RoomsHandleChange = 0x4f,

	/// Lobby2 message
	Lobby2SendMessage = 0x50,
	Lobby2ServerError = 0x51,

	/// Informs user of a new host GUID. Packet::Guid contains this new host RakNetGuid. The old host can be read out using BitStream->Read(RakNetGuid) starting on byte 1
	/// This is not returned until connected to a remote system
	/// If the oldHost is UNASSIGNED_RAKNET_GUID, then this is the first time the host has been determined
	Fcm2NewHost = 0x52,
	/// \internal For FullyConnectedMesh2 plugin
	Fcm2RequestFcmguid = 0x53,
	/// \internal For FullyConnectedMesh2 plugin
	Fcm2RespondConnectionCount = 0x54,
	/// \internal For FullyConnectedMesh2 plugin
	Fcm2InformFcmguid = 0x55,
	/// \internal For FullyConnectedMesh2 plugin
	Fcm2UpdateMinTotalConnectionCount = 0x56,
	/// A remote system (not necessarily the host) called FullyConnectedMesh2::StartVerifiedJoin() with our system as the client
	/// Use FullyConnectedMesh2::GetVerifiedJoinRequiredProcessingList() to read systems
	/// For each system, attempt NatPunchthroughClient::OpenNAT() and/or RakPeerInterface::Connect()
	/// When this has been done for all systems, the remote system will automatically be informed of the results
	/// \note Only the designated client gets this message
	/// \note You won't get this message if you are already connected to all target systems
	/// \note If you fail to connect to a system, this does not automatically mean you will get IdFcm2VerifiedJoinFailed as that system may have been shutting down from the host too
	/// \sa FullyConnectedMesh2::StartVerifiedJoin()
	Fcm2VerifiedJoinStart = 0x57,
	/// \internal The client has completed processing for all systems designated in IdFcm2VerifiedJoinStart
	Fcm2VerifiedJoinCapable = 0x58,
	/// Client failed to connect to a required systems notified via FullyConnectedMesh2::StartVerifiedJoin()
	/// RakPeerInterface::CloseConnection() was automatically called for all systems connected due to IdFcm2VerifiedJoinStart
	/// Programmer should inform the player via the UI that they cannot join this session, and to choose a different session
	/// \note Server normally sends us this message, however if connection to the server was lost, message will be returned locally
	/// \note Only the designated client gets this message
	Fcm2VerifiedJoinFailed = 0x59,
	/// The system that called StartVerifiedJoin() got IdFcm2VerifiedJoinCapable from the client and then called RespondOnVerifiedJoinCapable() with true
	/// AddParticipant() has automatically been called for this system
	/// Use GetVerifiedJoinAcceptedAdditionalData() to read any additional data passed to RespondOnVerifiedJoinCapable()
	/// \note All systems in the mesh get this message
	/// \sa RespondOnVerifiedJoinCapable()
	Fcm2VerifiedJoinAccepted = 0x5a,
	/// The system that called StartVerifiedJoin() got IdFcm2VerifiedJoinCapable from the client and then called RespondOnVerifiedJoinCapable() with false
	/// CloseConnection() has been automatically called for each system connected to since IdFcm2VerifiedJoinStart.
	/// The connection is NOT automatically closed to the original host that sent StartVerifiedJoin()
	/// Use GetVerifiedJoinRejectedAdditionalData() to read any additional data passed to RespondOnVerifiedJoinCapable()
	/// \note Only the designated client gets this message
	/// \sa RespondOnVerifiedJoinCapable()
	Fcm2VerifiedJoinRejected = 0x5b,

	/// UDP proxy messages. Second byte indicates type.
	UdpProxyGeneral = 0x5c,

	/// SQLite3Plugin - execute
	Sqlite3Exec = 0x5d,
	/// SQLite3Plugin - Remote database is unknown
	Sqlite3UnknownDb = 0x5e,
	/// Events happening with SQLiteClientLoggerPlugin
	SqlliteLogger = 0x5f,

	/// Sent to NatTypeDetectionServer
	NatTypeDetectionRequest = 0x60,
	/// Sent to NatTypeDetectionClient. Byte 1 contains the type of NAT detected.
	NatTypeDetectionResult = 0x61,

	/// Used by the router2 plugin
	Router2Internal = 0x62,
	/// No path is available or can be established to the remote system
	/// Packet::guid contains the endpoint guid that we were trying to reach
	Router2ForwardingNoPath = 0x63,
	/// \brief You can now call connect, ping, or other operations to the destination system.
	///
	/// Connect as follows:
	///
	/// RakNet::BitStream bs(packet->data, packet->length, false),
	/// bs.IgnoreBytes(sizeof(MessageID)),
	/// RakNetGUID endpointGuid,
	/// bs.Read(endpointGuid),
	/// unsigned short sourceToDestPort,
	/// bs.Read(sourceToDestPort),
	/// char ipAddressString[32],
	/// packet->systemAddress.ToString(false, ipAddressString),
	/// rakPeerInterface->Connect(ipAddressString, sourceToDestPort, 0,0),
	Router2ForwardingEstablished = 0x64,
	/// The IP address for a forwarded connection has changed
	/// Read endpointGuid and port as per IdRouter2ForwardingEstablished
	Router2Rerouted = 0x65,

	/// \internal Used by the team balancer plugin
	TeamBalancerInternal = 0x66,
	/// Cannot switch to the desired team because it is full. However, if someone on that team leaves, you will
	///  get IdTeamBalancerTeamAssigned later.
	/// For TeamBalancer: Byte 1 contains the team you requested to join. Following bytes contain NetworkID of which member
	TeamBalancerRequestedTeamFull = 0x67,
	/// Cannot switch to the desired team because all teams are locked. However, if someone on that team leaves,
	///  you will get ID_TEAM_BALANCER_SET_TEAM later.
	/// For TeamBalancer: Byte 1 contains the team you requested to join.
	TeamBalancerRequestedTeamLocked = 0x68,
	TeamBalancerTeamRequestedCancelled = 0x69,
	/// Team balancer plugin informing you of your team. Byte 1 contains the team you requested to join. Following bytes contain NetworkID of which member.
	TeamBalancerTeamAssigned = 0x6a,

	/// Gamebryo Lightspeed integration
	LightspeedIntegration = 0x6b,

	/// XBOX integration
	XboxLobby = 0x6c,

	/// The password we used to challenge the other system passed, meaning the other system has called TwoWayAuthentication::AddPassword() with the same password we passed to TwoWayAuthentication::Challenge()
	/// You can read the identifier used to challenge as follows:
	/// RakNet::BitStream bs(packet->data, packet->length, false), bs.IgnoreBytes(sizeof(RakNet::MessageID)), RakNet::RakString password, bs.Read(password),
	TwoWayAuthenticationIncomingChallengeSuccess = 0x6d,
	TwoWayAuthenticationOutgoingChallengeSuccess = 0x6e,
	/// A remote system sent us a challenge using TwoWayAuthentication::Challenge(), and the challenge failed.
	/// If the other system must pass the challenge to stay connected, you should call RakPeer::CloseConnection() to terminate the connection to the other system.
	TwoWayAuthenticationIncomingChallengeFailure = 0x6f,
	/// The other system did not rdd the password we used to TwoWayAuthentication::AddPassword()
	/// You can read the identifier used to challenge as follows:
	/// RakNet::BitStream bs(packet->data, packet->length, false), bs.IgnoreBytes(sizeof(MessageID)), RakNet::RakString password, bs.Read(password),
	TwoWayAuthenticationOutgoingChallengeFailure = 0x70,
	/// The other system did not respond within a timeout threshhold. Either the other system is not running the plugin or the other system was blocking on some operation for a long time.
	/// You can read the identifier used to challenge as follows:
	/// RakNet::BitStream bs(packet->data, packet->length, false), bs.IgnoreBytes(sizeof(MessageID)), RakNet::RakString password, bs.Read(password),
	TwoWayAuthenticationOutgoingChallengeTimeout = 0x71,
	/// \internal
	TwoWayAuthenticationNegotiation = 0x72,

	/// CloudClient / CloudServer
	CloudPostRequest = 0x73,
	CloudReleaseRequest = 0x74,
	CloudGetRequest = 0x75,
	CloudGetResponse = 0x76,
	CloudUnsubscribeRequest = 0x77,
	CloudServerToServerCommand = 0x78,
	CloudSubscriptionNotification = 0x79,

	// LibVoice
	LibVoice = 0x7a,

	RelayPlugin = 0x7b,
	NatRequestBoundAddresses = 0x7c,
	NatRespondBoundAddresses = 0x7d,
	Fcm2UpdateUserContext = 0x7e,
	Reserved3 = 0x7f,
	Reserved4 = 0x80,
	Reserved5 = 0x81,
	Reserved6 = 0x82,
	Reserved7 = 0x83,
	Reserved8 = 0x84,
	Reserved9 = 0x85,

	// For the user to use.  Start your first enumeration at this value.
	UserPacketEnum = 0x86,
	//-------------------------------------------------------------------------------------------------------------
	Nack = 0xa0,
	Ack = 0xc0
}
