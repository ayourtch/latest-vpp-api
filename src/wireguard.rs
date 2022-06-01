/*
   Autogenerated Data, Do not Edit!
*/
#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]
use vpp_api_macros::{VppMessage,VppUnionIdent};
use std::convert::TryInto;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use vpp_api_encoding::typ::*;
pub use vpp_api_encoding;
use vpp_api_message::VppApiMessage;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum;
use crate::ip_types::*;
use crate::interface_types::*;
// Implementation for wireguard_interface
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WireguardInterface {
	pub user_instance : u32,
	pub sw_if_index : InterfaceIndex,
	pub private_key : FixedSizeArray<u8, typenum::U32>,
	pub public_key : FixedSizeArray<u8, typenum::U32>,
	pub port : u16,
	pub src_ip : Address,
}
// Implementation for wireguard_peer
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WireguardPeer {
	pub peer_index : u32,
	pub public_key : FixedSizeArray<u8, typenum::U32>,
	pub port : u16,
	pub persistent_keepalive : u16,
	pub table_id : u32,
	pub endpoint : Address,
	pub sw_if_index : InterfaceIndex,
	pub flags : EnumFlag<WireguardPeerFlags>,
	pub n_allowed_ips : u8,
	pub allowed_ips : VariableSizeArray<Prefix>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum WireguardPeerFlags {
	 WIREGUARD_PEER_STATUS_DEAD=1,
	 WIREGUARD_PEER_ESTABLISHED=2,
}
impl Default for WireguardPeerFlags {
	fn default() -> Self { WireguardPeerFlags::WIREGUARD_PEER_STATUS_DEAD }
}
impl AsEnumFlag for WireguardPeerFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => WireguardPeerFlags::WIREGUARD_PEER_STATUS_DEAD,
			 2 => WireguardPeerFlags::WIREGUARD_PEER_ESTABLISHED,
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 8 as u32
	}
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_interface_create_a530137e)]
pub struct WireguardInterfaceCreate {
	pub client_index : u32,
	pub context : u32,
	pub interface : WireguardInterface,
	pub generate_key : bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_interface_create_reply_5383d31f)]
pub struct WireguardInterfaceCreateReply {
	pub context : u32,
	pub retval : i32,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_interface_delete_f9e6675e)]
pub struct WireguardInterfaceDelete {
	pub client_index : u32,
	pub context : u32,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_interface_delete_reply_e8d4e804)]
pub struct WireguardInterfaceDeleteReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_interface_dump_2c954158)]
pub struct WireguardInterfaceDump {
	pub client_index : u32,
	pub context : u32,
	pub show_private_key : bool,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_interface_details_0dd4865d)]
pub struct WireguardInterfaceDetails {
	pub context : u32,
	pub interface : WireguardInterface,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(want_wireguard_peer_events_3bc666c8)]
pub struct WantWireguardPeerEvents {
	pub client_index : u32,
	pub context : u32,
	pub sw_if_index : InterfaceIndex,
	pub peer_index : u32,
	pub enable_disable : u32,
	pub pid : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(want_wireguard_peer_events_reply_e8d4e804)]
pub struct WantWireguardPeerEventsReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_peer_event_4e1b5d67)]
pub struct WireguardPeerEvent {
	pub client_index : u32,
	pub pid : u32,
	pub peer_index : u32,
	 pub flags : EnumFlag<WireguardPeerFlags>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_peer_add_9b8aad61)]
pub struct WireguardPeerAdd {
	pub client_index : u32,
	pub context : u32,
	pub peer : WireguardPeer,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_peer_add_reply_084a0cd3)]
pub struct WireguardPeerAddReply {
	pub context : u32,
	pub retval : i32,
	pub peer_index : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_peer_remove_3b74607a)]
pub struct WireguardPeerRemove {
	pub client_index : u32,
	pub context : u32,
	pub peer_index : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_peer_remove_reply_e8d4e804)]
pub struct WireguardPeerRemoveReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_peers_dump_3b74607a)]
pub struct WireguardPeersDump {
	pub client_index : u32,
	pub context : u32,
	pub peer_index : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wireguard_peers_details_6a9f6bc3)]
pub struct WireguardPeersDetails {
	pub context : u32,
	pub peer : WireguardPeer,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wg_set_async_mode_a6465f7c)]
pub struct WgSetAsyncMode {
	pub client_index : u32,
	pub context : u32,
	pub async_enable : bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(wg_set_async_mode_reply_e8d4e804)]
pub struct WgSetAsyncModeReply {
	pub context : u32,
	pub retval : i32,
}
