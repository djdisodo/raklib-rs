use std::mem::size_of;

pub const SIZE: usize = size_of::<u8>(); // size of message identifier

pub fn read_from(buffer: &[u8]) -> u8 {
	buffer[0]
}
//From https://github.com/OculusVR/RakNet/blob/master/Source/MessageIdentifiers.h

//
// RESERVED TYPES - DO NOT CHANGE THESE
// All types from RakPeer
//
/// These types are never returned to the user.
/// Ping from a connected system.  Update timestamps (internal use only)
pub const ID_CONNECTED_PING: u8 = 0x00;
/// Ping from an unconnected system.  Reply but do not update timestamps. (internal use only)
pub const ID_UNCONNECTED_PING: u8 = 0x01;
/// Ping from an unconnected system.  Only reply if we have open connections. Do not update timestamps. (internal use only)
pub const ID_UNCONNECTED_PING_OPEN_CONNECTIONS: u8 = 0x02;
/// Pong from a connected system.  Update timestamps (internal use only)
pub const ID_CONNECTED_PONG: u8 = 0x03;
/// A reliable packet to detect lost connections (internal use only)
pub const ID_DETECT_LOST_CONNECTIONS: u8 = 0x04;
/// C2S: Initial query: Header(1), OfflineMesageID(16), Protocol number(1), Pad(toMTU), sent with no fragment set.
/// If protocol fails on server, returns ID_INCOMPATIBLE_PROTOCOL_VERSION to client
pub const ID_OPEN_CONNECTION_REQUEST_1: u8 = 0x05;
/// S2C: Header(1), OfflineMesageID(16), server GUID(8), HasSecurity(1), Cookie(4, if HasSecurity)
/// , pub key (if do security is true), MTU(2). If pub key fails on client, returns ID_PUB_KEY_MISMATCH
pub const ID_OPEN_CONNECTION_REPLY_1: u8 = 0x06;
/// C2S: Header(1), OfflineMesageID(16), Cookie(4, if HasSecurity is true on the server), clientSupportsSecurity(1 bit),
/// handshakeChallenge (if has security on both server and client), remoteBindingAddress(6), MTU(2), client GUID(8)
/// Connection slot allocated if cookie is valid, server is not full, GUID and IP not already in use.
pub const ID_OPEN_CONNECTION_REQUEST_2: u8 = 0x07;
/// S2C: Header(1), OfflineMesageID(16), server GUID(8), mtu(2), doSecurity(1 bit), handshakeAnswer (if do security is true)
pub const ID_OPEN_CONNECTION_REPLY_2: u8 = 0x08;
/// C2S: Header(1), GUID(8), Timestamp, HasSecurity(1), Proof(32)
pub const ID_CONNECTION_REQUEST: u8 = 0x09;
/// RakPeer - Remote system requires secure connections, pass a pub key to RakPeerInterface::Connect()
pub const ID_REMOTE_SYSTEM_REQUIRES_PUB_KEY: u8 = 0x0a;
/// RakPeer - We passed a pub key to RakPeerInterface::Connect(), but the other system did not have security turned on
pub const ID_OUR_SYSTEM_REQUIRES_SECURITY: u8 = 0x0b;
/// RakPeer - Wrong pub key passed to RakPeerInterface::Connect()
pub const ID_PUB_KEY_MISMATCH: u8 = 0x0c;
/// RakPeer - Same as ID_ADVERTISE_SYSTEM, but intended for internal use rather than being passed to the user.
/// Second byte indicates type. Used currently for NAT punchthrough for receiver port advertisement. See ID_NAT_ADVERTISE_RECIPIENT_PORT
pub const ID_OUT_OF_BAND_INTERNAL: u8 = 0x0d;
/// If RakPeerInterface::Send() is called where PacketReliability contains _WITH_ACK_RECEIPT, then on a later call to
/// RakPeerInterface::Receive() you will get ID_SND_RECEIPT_ACKED or ID_SND_RECEIPT_LOSS. The message will be 5 bytes long,
/// and bytes 1-4 inclusive will contain a number in native order containing a number that identifies this message.
/// This number will be returned by RakPeerInterface::Send() or RakPeerInterface::SendList(). ID_SND_RECEIPT_ACKED means that
/// the message arrived
pub const ID_SND_RECEIPT_ACKED: u8 = 0x0e;
/// If RakPeerInterface::Send() is called where PacketReliability contains UNRELIABLE_WITH_ACK_RECEIPT, then on a later call to
/// RakPeerInterface::Receive() you will get ID_SND_RECEIPT_ACKED or ID_SND_RECEIPT_LOSS. The message will be 5 bytes long,
/// and bytes 1-4 inclusive will contain a number in native order containing a number that identifies this message. This number
/// will be returned by RakPeerInterface::Send() or RakPeerInterface::SendList(). ID_SND_RECEIPT_LOSS means that an ack for the
/// message did not arrive (it may or may not have been delivered, probably not). On disconnect or shutdown, you will not get
/// ID_SND_RECEIPT_LOSS for unsent messages, you should consider those messages as all lost.
pub const ID_SND_RECEIPT_LOSS: u8 = 0x0f;

//
// USER TYPES - DO NOT CHANGE THESE
//

/// RakPeer - In a client/server environment, our connection request to the server has been accepted.
pub const ID_CONNECTION_REQUEST_ACCEPTED: u8 = 0x10;
/// RakPeer - Sent to the player when a connection request cannot be completed due to inability to connect.
pub const ID_CONNECTION_ATTEMPT_FAILED: u8 = 0x11;
/// RakPeer - Sent a connect request to a system we are currently connected to.
pub const ID_ALREADY_CONNECTED: u8 = 0x12;
/// RakPeer - A remote system has successfully connected.
pub const ID_NEW_INCOMING_CONNECTION: u8 = 0x13;
/// RakPeer - The system we attempted to connect to is not accepting new connections.
pub const ID_NO_FREE_INCOMING_CONNECTIONS: u8 = 0x14;
/// RakPeer - The system specified in Packet::systemAddress has disconnected from us.  For the client, this would mean the
/// server has shutdown.
pub const ID_DISCONNECTION_NOTIFICATION: u8 = 0x15;
/// RakPeer - Reliable packets cannot be delivered to the system specified in Packet::systemAddress.  The connection to that
/// system has been closed.
pub const ID_CONNECTION_LOST: u8 = 0x16;
/// RakPeer - We are banned from the system we attempted to connect to.
pub const ID_CONNECTION_BANNED: u8 = 0x17;
/// RakPeer - The remote system is using a password and has refused our connection because we did not set the correct password.
pub const ID_INVALID_PASSWORD: u8 = 0x18;
// RAKNET_PROTOCOL_VERSION in RakNetVersion.h does not match on the remote system what we have on our system
// This means the two systems cannot communicate.
// The 2nd byte of the message contains the value of RAKNET_PROTOCOL_VERSION for the remote system
pub const ID_INCOMPATIBLE_PROTOCOL_VERSION: u8 = 0x19;
// Means that this IP address connected recently, and can't connect again as a security measure. See
/// RakPeer::SetLimitIPConnectionFrequency()
pub const ID_IP_RECENTLY_CONNECTED: u8 = 0x1a;
/// RakPeer - The sizeof(RakNetTime) bytes following this byte represent a value which is automatically modified by the difference
/// in system times between the sender and the recipient. Requires that you call SetOccasionalPing.
pub const ID_TIMESTAMP: u8 = 0x1b;
/// RakPeer - Pong from an unconnected system.  First byte is ID_UNCONNECTED_PONG, second sizeof(RakNet::TimeMS) bytes is the ping,
/// following bytes is system specific enumeration data.
/// Read using bitstreams
pub const ID_UNCONNECTED_PONG: u8 = 0x1c;
/// RakPeer - Inform a remote system of our IP/Port. On the recipient, all data past ID_ADVERTISE_SYSTEM is whatever was passed to
/// the data parameter
pub const ID_ADVERTISE_SYSTEM: u8 = 0x1d;
// RakPeer - Downloading a large message. Format is ID_DOWNLOAD_PROGRESS (MessageID), partCount (unsigned int),
///  partTotal (unsigned int),
/// partLength (unsigned int), first part data (length <= MAX_MTU_SIZE). See the three parameters partCount, partTotal
///  and partLength in OnFileProgress in FileListTransferCBInterface.h
pub const ID_DOWNLOAD_PROGRESS: u8 = 0x1e;

/// ConnectionGraph2 plugin - In a client/server environment, a client other than ourselves has disconnected gracefully.
///   Packet::systemAddress is modified to reflect the systemAddress of this client.
pub const ID_REMOTE_DISCONNECTION_NOTIFICATION: u8 = 0x1f;
/// ConnectionGraph2 plugin - In a client/server environment, a client other than ourselves has been forcefully dropped.
///  Packet::systemAddress is modified to reflect the systemAddress of this client.
pub const ID_REMOTE_CONNECTION_LOST: u8 = 0x20;
/// ConnectionGraph2 plugin: Bytes 1-4: u8 = count. for (count items) contains {SystemAddress, RakNetGUID, 2 byte ping}
pub const ID_REMOTE_NEW_INCOMING_CONNECTION: u8 = 0x21;

/// FileListTransfer plugin - Setup data
pub const ID_FILE_LIST_TRANSFER_HEADER: u8 = 0x22;
/// FileListTransfer plugin - A file
pub const ID_FILE_LIST_TRANSFER_FILE: u8 = 0x23;
// Ack for reference push, to send more of the file
pub const ID_FILE_LIST_REFERENCE_PUSH_ACK: u8 = 0x24;

/// DirectoryDeltaTransfer plugin - Request from a remote system for a download of a directory
pub const ID_DDT_DOWNLOAD_REQUEST: u8 = 0x25;

/// RakNetTransport plugin - Transport provider message, used for remote console
pub const ID_TRANSPORT_STRING: u8 = 0x26;

/// ReplicaManager plugin - Create an object
pub const ID_REPLICA_MANAGER_CONSTRUCTION: u8 = 0x27;
/// ReplicaManager plugin - Changed scope of an object
pub const ID_REPLICA_MANAGER_SCOPE_CHANGE: u8 = 0x28;
/// ReplicaManager plugin - Serialized data of an object
pub const ID_REPLICA_MANAGER_SERIALIZE: u8 = 0x29;
/// ReplicaManager plugin - New connection, about to send all world objects
pub const ID_REPLICA_MANAGER_DOWNLOAD_STARTED: u8 = 0x2a;
/// ReplicaManager plugin - Finished downloading all serialized objects
pub const ID_REPLICA_MANAGER_DOWNLOAD_COMPLETE: u8 = 0x2b;

/// RakVoice plugin - Open a communication channel
pub const ID_RAKVOICE_OPEN_CHANNEL_REQUEST: u8 = 0x2c;
/// RakVoice plugin - Communication channel accepted
pub const ID_RAKVOICE_OPEN_CHANNEL_REPLY: u8 = 0x2d;
/// RakVoice plugin - Close a communication channel
pub const ID_RAKVOICE_CLOSE_CHANNEL: u8 = 0x2e;
/// RakVoice plugin - Voice data
pub const ID_RAKVOICE_DATA: u8 = 0x2f;

/// Autopatcher plugin - Get a list of files that have changed since a certain date
pub const ID_AUTOPATCHER_GET_CHANGELIST_SINCE_DATE: u8 = 0x30;
/// Autopatcher plugin - A list of files to create
pub const ID_AUTOPATCHER_CREATION_LIST: u8 = 0x31;
/// Autopatcher plugin - A list of files to delete
pub const ID_AUTOPATCHER_DELETION_LIST: u8 = 0x32;
/// Autopatcher plugin - A list of files to get patches for
pub const ID_AUTOPATCHER_GET_PATCH: u8 = 0x33;
/// Autopatcher plugin - A list of patches for a list of files
pub const ID_AUTOPATCHER_PATCH_LIST: u8 = 0x34;
/// Autopatcher plugin - Returned to the user: An error from the database repository for the autopatcher.
pub const ID_AUTOPATCHER_REPOSITORY_FATAL_ERROR: u8 = 0x35;
/// Autopatcher plugin - Returned to the user: The server does not allow downloading unmodified game files.
pub const ID_AUTOPATCHER_CANNOT_DOWNLOAD_ORIGINAL_UNMODIFIED_FILES: u8 = 0x36;
/// Autopatcher plugin - Finished getting all files from the autopatcher
pub const ID_AUTOPATCHER_FINISHED_INTERNAL: u8 = 0x37;
pub const ID_AUTOPATCHER_FINISHED: u8 = 0x38;
/// Autopatcher plugin - Returned to the user: You must restart the application to finish patching.
pub const ID_AUTOPATCHER_RESTART_APPLICATION: u8 = 0x39;

/// NATPunchthrough plugin: internal
pub const ID_NAT_PUNCHTHROUGH_REQUEST: u8 = 0x3a;
/// NATPunchthrough plugin: internal
//ID_NAT_GROUP_PUNCHTHROUGH_REQUEST,
/// NATPunchthrough plugin: internal
//ID_NAT_GROUP_PUNCHTHROUGH_REPLY,
/// NATPunchthrough plugin: internal
pub const ID_NAT_CONNECT_AT_TIME: u8 = 0x3b;
/// NATPunchthrough plugin: internal
pub const ID_NAT_GET_MOST_RECENT_PORT: u8 = 0x3c;
/// NATPunchthrough plugin: internal
pub const ID_NAT_CLIENT_READY: u8 = 0x3d;
/// NATPunchthrough plugin: internal
//ID_NAT_GROUP_PUNCHTHROUGH_FAILURE_NOTIFICATION,

/// NATPunchthrough plugin: Destination system is not connected to the server. Bytes starting at offset 1 contains the
///  RakNetGUID destination field of NatPunchthroughClient::OpenNAT().
pub const ID_NAT_TARGET_NOT_CONNECTED: u8 = 0x3e;
/// NATPunchthrough plugin: Destination system is not responding to ID_NAT_GET_MOST_RECENT_PORT. Possibly the plugin is not installed.
///  Bytes starting at offset 1 contains the RakNetGUID  destination field of NatPunchthroughClient::OpenNAT().
pub const ID_NAT_TARGET_UNRESPONSIVE: u8 = 0x3f;
/// NATPunchthrough plugin: The server lost the connection to the destination system while setting up punchthrough.
///  Possibly the plugin is not installed. Bytes starting at offset 1 contains the RakNetGUID  destination
///  field of NatPunchthroughClient::OpenNAT().
pub const ID_NAT_CONNECTION_TO_TARGET_LOST: u8 = 0x40;
/// NATPunchthrough plugin: This punchthrough is already in progress. Possibly the plugin is not installed.
///  Bytes starting at offset 1 contains the RakNetGUID destination field of NatPunchthroughClient::OpenNAT().
pub const ID_NAT_ALREADY_IN_PROGRESS: u8 = 0x41;
/// NATPunchthrough plugin: This message is generated on the local system, and does not come from the network.
///  packet::guid contains the destination field of NatPunchthroughClient::OpenNAT(). Byte 1 contains 1 if you are the sender, 0 if not
pub const ID_NAT_PUNCHTHROUGH_FAILED: u8 = 0x42;
/// NATPunchthrough plugin: Punchthrough succeeded. See packet::systemAddress and packet::guid. Byte 1 contains 1 if you are the sender,
///  0 if not. You can now use RakPeer::Connect() or other calls to communicate with this system.
pub const ID_NAT_PUNCHTHROUGH_SUCCEEDED: u8 = 0x43;

/// ReadyEvent plugin - Set the ready state for a particular system
/// First 4 bytes after the message contains the id
pub const ID_READY_EVENT_SET: u8 = 0x44;
/// ReadyEvent plugin - Unset the ready state for a particular system
/// First 4 bytes after the message contains the id
pub const ID_READY_EVENT_UNSET: u8 = 0x45;
/// All systems are in state ID_READY_EVENT_SET
/// First 4 bytes after the message contains the id
pub const ID_READY_EVENT_ALL_SET: u8 = 0x46;
/// \internal, do not process in your game
/// ReadyEvent plugin - Request of ready event state - used for pulling data when newly connecting
pub const ID_READY_EVENT_QUERY: u8 = 0x47;

/// Lobby packets. Second byte indicates type.
pub const ID_LOBBY_GENERAL: u8 = 0x48;

// RPC3, RPC4 error
pub const ID_RPC_REMOTE_ERROR: u8 = 0x49;
/// Plugin based replacement for RPC system
pub const ID_RPC_PLUGIN: u8 = 0x4a;

/// FileListTransfer transferring large files in chunks that are read only when needed, to save memory
pub const ID_FILE_LIST_REFERENCE_PUSH: u8 = 0x4b;
/// Force the ready event to all set
pub const ID_READY_EVENT_FORCE_ALL_SET: u8 = 0x4c;

/// Rooms function
pub const ID_ROOMS_EXECUTE_FUNC: u8 = 0x4d;
pub const ID_ROOMS_LOGON_STATUS: u8 = 0x4e;
pub const ID_ROOMS_HANDLE_CHANGE: u8 = 0x4f;

/// Lobby2 message
pub const ID_LOBBY2_SEND_MESSAGE: u8 = 0x50;
pub const ID_LOBBY2_SERVER_ERROR: u8 = 0x51;

/// Informs user of a new host GUID. Packet::Guid contains this new host RakNetGuid. The old host can be read out using BitStream->Read(RakNetGuid) starting on byte 1
/// This is not returned until connected to a remote system
/// If the oldHost is UNASSIGNED_RAKNET_GUID, then this is the first time the host has been determined
pub const ID_FCM2_NEW_HOST: u8 = 0x52;
/// \internal For FullyConnectedMesh2 plugin
pub const ID_FCM2_REQUEST_FCMGUID: u8 = 0x53;
/// \internal For FullyConnectedMesh2 plugin
pub const ID_FCM2_RESPOND_CONNECTION_COUNT: u8 = 0x54;
/// \internal For FullyConnectedMesh2 plugin
pub const ID_FCM2_INFORM_FCMGUID: u8 = 0x55;
/// \internal For FullyConnectedMesh2 plugin
pub const ID_FCM2_UPDATE_MIN_TOTAL_CONNECTION_COUNT: u8 = 0x56;
/// A remote system (not necessarily the host) called FullyConnectedMesh2::StartVerifiedJoin() with our system as the client
/// Use FullyConnectedMesh2::GetVerifiedJoinRequiredProcessingList() to read systems
/// For each system, attempt NatPunchthroughClient::OpenNAT() and/or RakPeerInterface::Connect()
/// When this has been done for all systems, the remote system will automatically be informed of the results
/// \note Only the designated client gets this message
/// \note You won't get this message if you are already connected to all target systems
/// \note If you fail to connect to a system, this does not automatically mean you will get ID_FCM2_VERIFIED_JOIN_FAILED as that system may have been shutting down from the host too
/// \sa FullyConnectedMesh2::StartVerifiedJoin()
pub const ID_FCM2_VERIFIED_JOIN_START: u8 = 0x57;
/// \internal The client has completed processing for all systems designated in ID_FCM2_VERIFIED_JOIN_START
pub const ID_FCM2_VERIFIED_JOIN_CAPABLE: u8 = 0x58;
/// Client failed to connect to a required systems notified via FullyConnectedMesh2::StartVerifiedJoin()
/// RakPeerInterface::CloseConnection() was automatically called for all systems connected due to ID_FCM2_VERIFIED_JOIN_START
/// Programmer should inform the player via the UI that they cannot join this session, and to choose a different session
/// \note Server normally sends us this message, however if connection to the server was lost, message will be returned locally
/// \note Only the designated client gets this message
pub const ID_FCM2_VERIFIED_JOIN_FAILED: u8 = 0x59;
/// The system that called StartVerifiedJoin() got ID_FCM2_VERIFIED_JOIN_CAPABLE from the client and then called RespondOnVerifiedJoinCapable() with true
/// AddParticipant() has automatically been called for this system
/// Use GetVerifiedJoinAcceptedAdditionalData() to read any additional data passed to RespondOnVerifiedJoinCapable()
/// \note All systems in the mesh get this message
/// \sa RespondOnVerifiedJoinCapable()
pub const ID_FCM2_VERIFIED_JOIN_ACCEPTED: u8 = 0x5a;
/// The system that called StartVerifiedJoin() got ID_FCM2_VERIFIED_JOIN_CAPABLE from the client and then called RespondOnVerifiedJoinCapable() with false
/// CloseConnection() has been automatically called for each system connected to since ID_FCM2_VERIFIED_JOIN_START.
/// The connection is NOT automatically closed to the original host that sent StartVerifiedJoin()
/// Use GetVerifiedJoinRejectedAdditionalData() to read any additional data passed to RespondOnVerifiedJoinCapable()
/// \note Only the designated client gets this message
/// \sa RespondOnVerifiedJoinCapable()
pub const ID_FCM2_VERIFIED_JOIN_REJECTED: u8 = 0x5b;

/// UDP proxy messages. Second byte indicates type.
pub const ID_UDP_PROXY_GENERAL: u8 = 0x5c;

/// SQLite3Plugin - execute
pub const ID_SQLITE3_EXEC: u8 = 0x5d;
/// SQLite3Plugin - Remote database is unknown
pub const ID_SQLITE3_UNKNOWN_DB: u8 = 0x5e;
/// Events happening with SQLiteClientLoggerPlugin
pub const ID_SQLLITE_LOGGER: u8 = 0x5f;

/// Sent to NatTypeDetectionServer
pub const ID_NAT_TYPE_DETECTION_REQUEST: u8 = 0x60;
/// Sent to NatTypeDetectionClient. Byte 1 contains the type of NAT detected.
pub const ID_NAT_TYPE_DETECTION_RESULT: u8 = 0x61;

/// Used by the router2 plugin
pub const ID_ROUTER_2_INTERNAL: u8 = 0x62;
/// No path is available or can be established to the remote system
/// Packet::guid contains the endpoint guid that we were trying to reach
pub const ID_ROUTER_2_FORWARDING_NO_PATH: u8 = 0x63;
/// \brief You can now call connect, ping, or other operations to the destination system.
///
/// Connect as follows:
///
/// RakNet::BitStream bs(packet->data, packet->length, false);
/// bs.IgnoreBytes(sizeof(MessageID));
/// RakNetGUID endpointGuid;
/// bs.Read(endpointGuid);
/// unsigned short sourceToDestPort;
/// bs.Read(sourceToDestPort);
/// char ipAddressString[32];
/// packet->systemAddress.ToString(false, ipAddressString);
/// rakPeerInterface->Connect(ipAddressString, sourceToDestPort, 0,0);
pub const ID_ROUTER_2_FORWARDING_ESTABLISHED: u8 = 0x64;
/// The IP address for a forwarded connection has changed
/// Read endpointGuid and port as per ID_ROUTER_2_FORWARDING_ESTABLISHED
pub const ID_ROUTER_2_REROUTED: u8 = 0x65;

/// \internal Used by the team balancer plugin
pub const ID_TEAM_BALANCER_INTERNAL: u8 = 0x66;
/// Cannot switch to the desired team because it is full. However, if someone on that team leaves, you will
///  get ID_TEAM_BALANCER_TEAM_ASSIGNED later.
/// For TeamBalancer: Byte 1 contains the team you requested to join. Following bytes contain NetworkID of which member
pub const ID_TEAM_BALANCER_REQUESTED_TEAM_FULL: u8 = 0x67;
/// Cannot switch to the desired team because all teams are locked. However, if someone on that team leaves,
///  you will get ID_TEAM_BALANCER_SET_TEAM later.
/// For TeamBalancer: Byte 1 contains the team you requested to join.
pub const ID_TEAM_BALANCER_REQUESTED_TEAM_LOCKED: u8 = 0x68;
pub const ID_TEAM_BALANCER_TEAM_REQUESTED_CANCELLED: u8 = 0x69;
/// Team balancer plugin informing you of your team. Byte 1 contains the team you requested to join. Following bytes contain NetworkID of which member.
pub const ID_TEAM_BALANCER_TEAM_ASSIGNED: u8 = 0x6a;

/// Gamebryo Lightspeed integration
pub const ID_LIGHTSPEED_INTEGRATION: u8 = 0x6b;

/// XBOX integration
pub const ID_XBOX_LOBBY: u8 = 0x6c;

/// The password we used to challenge the other system passed, meaning the other system has called TwoWayAuthentication::AddPassword() with the same password we passed to TwoWayAuthentication::Challenge()
/// You can read the identifier used to challenge as follows:
/// RakNet::BitStream bs(packet->data, packet->length, false); bs.IgnoreBytes(sizeof(RakNet::MessageID)); RakNet::RakString password; bs.Read(password);
pub const ID_TWO_WAY_AUTHENTICATION_INCOMING_CHALLENGE_SUCCESS: u8 = 0x6d;
pub const ID_TWO_WAY_AUTHENTICATION_OUTGOING_CHALLENGE_SUCCESS: u8 = 0x6e;
/// A remote system sent us a challenge using TwoWayAuthentication::Challenge(), and the challenge failed.
/// If the other system must pass the challenge to stay connected, you should call RakPeer::CloseConnection() to terminate the connection to the other system.
pub const ID_TWO_WAY_AUTHENTICATION_INCOMING_CHALLENGE_FAILURE: u8 = 0x6f;
/// The other system did not add the password we used to TwoWayAuthentication::AddPassword()
/// You can read the identifier used to challenge as follows:
/// RakNet::BitStream bs(packet->data, packet->length, false); bs.IgnoreBytes(sizeof(MessageID)); RakNet::RakString password; bs.Read(password);
pub const ID_TWO_WAY_AUTHENTICATION_OUTGOING_CHALLENGE_FAILURE: u8 = 0x70;
/// The other system did not respond within a timeout threshhold. Either the other system is not running the plugin or the other system was blocking on some operation for a long time.
/// You can read the identifier used to challenge as follows:
/// RakNet::BitStream bs(packet->data, packet->length, false); bs.IgnoreBytes(sizeof(MessageID)); RakNet::RakString password; bs.Read(password);
pub const ID_TWO_WAY_AUTHENTICATION_OUTGOING_CHALLENGE_TIMEOUT: u8 = 0x71;
/// \internal
pub const ID_TWO_WAY_AUTHENTICATION_NEGOTIATION: u8 = 0x72;

/// CloudClient / CloudServer
pub const ID_CLOUD_POST_REQUEST: u8 = 0x73;
pub const ID_CLOUD_RELEASE_REQUEST: u8 = 0x74;
pub const ID_CLOUD_GET_REQUEST: u8 = 0x75;
pub const ID_CLOUD_GET_RESPONSE: u8 = 0x76;
pub const ID_CLOUD_UNSUBSCRIBE_REQUEST: u8 = 0x77;
pub const ID_CLOUD_SERVER_TO_SERVER_COMMAND: u8 = 0x78;
pub const ID_CLOUD_SUBSCRIPTION_NOTIFICATION: u8 = 0x79;

// LibVoice
pub const ID_LIB_VOICE: u8 = 0x7a;

pub const ID_RELAY_PLUGIN: u8 = 0x7b;
pub const ID_NAT_REQUEST_BOUND_ADDRESSES: u8 = 0x7c;
pub const ID_NAT_RESPOND_BOUND_ADDRESSES: u8 = 0x7d;
pub const ID_FCM2_UPDATE_USER_CONTEXT: u8 = 0x7e;
pub const ID_RESERVED_3: u8 = 0x7f;
pub const ID_RESERVED_4: u8 = 0x80;
pub const ID_RESERVED_5: u8 = 0x81;
pub const ID_RESERVED_6: u8 = 0x82;
pub const ID_RESERVED_7: u8 = 0x83;
pub const ID_RESERVED_8: u8 = 0x84;
pub const ID_RESERVED_9: u8 = 0x85;

// For the user to use.  Start your first enumeration at this value.
pub const ID_USER_PACKET_ENUM: u8 = 0x86;
//-------------------------------------------------------------------------------------------------------------