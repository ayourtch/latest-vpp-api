/*
   Autogenerated Data, Do not Edit! 
*/
#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]
use vpp_api_macros::{VppMessage,VppUnionIdent}; 
use std::convert::TryInto; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use crate::VppNamedMessage;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum;
use crate::ip_types::*; 
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum BfdState { 
	 BFD_STATE_API_ADMIN_DOWN=0, 
	 BFD_STATE_API_DOWN=1, 
	 BFD_STATE_API_INIT=2, 
	 BFD_STATE_API_UP=3, 
} 
impl Default for BfdState { 
	fn default() -> Self { BfdState::BFD_STATE_API_ADMIN_DOWN }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_set_echo_source_f9e6675e)] 
pub struct BfdUdpSetEchoSource { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_set_echo_source_reply_e8d4e804)] 
pub struct BfdUdpSetEchoSourceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_del_echo_source_51077d14)] 
pub struct BfdUdpDelEchoSource { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_del_echo_source_reply_e8d4e804)] 
pub struct BfdUdpDelEchoSourceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_get_echo_source_51077d14)] 
pub struct BfdUdpGetEchoSource { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_get_echo_source_reply_e3d736a1)] 
pub struct BfdUdpGetEchoSourceReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_set : bool, 
	pub have_usable_ip4 : bool, 
	pub ip4_addr : Ip4Address, 
	pub have_usable_ip6 : bool, 
	pub ip6_addr : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_add_939cd26a)] 
pub struct BfdUdpAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub desired_min_tx : u32, 
	pub required_min_rx : u32, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	pub detect_mult : u8, 
	pub is_authenticated : bool, 
	pub bfd_key_id : u8, 
	pub conf_key_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_add_reply_e8d4e804)] 
pub struct BfdUdpAddReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_mod_913df085)] 
pub struct BfdUdpMod { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub desired_min_tx : u32, 
	pub required_min_rx : u32, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	pub detect_mult : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_mod_reply_e8d4e804)] 
pub struct BfdUdpModReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_del_dcb13a89)] 
pub struct BfdUdpDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_del_reply_e8d4e804)] 
pub struct BfdUdpDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_session_dump_51077d14)] 
pub struct BfdUdpSessionDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_session_details_09fb2f2d)] 
pub struct BfdUdpSessionDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	pub state : BfdState, 
	pub is_authenticated : bool, 
	pub bfd_key_id : u8, 
	pub conf_key_id : u32, 
	pub required_min_rx : u32, 
	pub desired_min_tx : u32, 
	pub detect_mult : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_session_set_flags_04b4bdfd)] 
pub struct BfdUdpSessionSetFlags { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	 pub flags : EnumFlag<IfStatusFlags>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_session_set_flags_reply_e8d4e804)] 
pub struct BfdUdpSessionSetFlagsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(want_bfd_events_c5e2af94)] 
pub struct WantBfdEvents { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable_disable : bool, 
	pub pid : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(want_bfd_events_reply_e8d4e804)] 
pub struct WantBfdEventsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_session_event_8eaaf062)] 
pub struct BfdUdpSessionEvent { 
	pub client_index : u32, 
	pub pid : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	pub state : BfdState, 
	pub is_authenticated : bool, 
	pub bfd_key_id : u8, 
	pub conf_key_id : u32, 
	pub required_min_rx : u32, 
	pub desired_min_tx : u32, 
	pub detect_mult : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_auth_set_key_690b8877)] 
pub struct BfdAuthSetKey { 
	pub client_index : u32, 
	pub context : u32, 
	pub conf_key_id : u32, 
	pub key_len : u8, 
	pub auth_type : u8, 
	pub key : FixedSizeArray<u8, typenum::U20>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_auth_set_key_reply_e8d4e804)] 
pub struct BfdAuthSetKeyReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_auth_del_key_65310b22)] 
pub struct BfdAuthDelKey { 
	pub client_index : u32, 
	pub context : u32, 
	pub conf_key_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_auth_del_key_reply_e8d4e804)] 
pub struct BfdAuthDelKeyReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_auth_keys_dump_51077d14)] 
pub struct BfdAuthKeysDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_auth_keys_details_84130e9f)] 
pub struct BfdAuthKeysDetails { 
	pub context : u32, 
	pub conf_key_id : u32, 
	pub use_count : u32, 
	pub auth_type : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_auth_activate_21fd1bdb)] 
pub struct BfdUdpAuthActivate { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	pub is_delayed : bool, 
	pub bfd_key_id : u8, 
	pub conf_key_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_auth_activate_reply_e8d4e804)] 
pub struct BfdUdpAuthActivateReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_auth_deactivate_9a05e2e0)] 
pub struct BfdUdpAuthDeactivate { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local_addr : Address, 
	pub peer_addr : Address, 
	pub is_delayed : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(bfd_udp_auth_deactivate_reply_e8d4e804)] 
pub struct BfdUdpAuthDeactivateReply { 
	pub context : u32, 
	pub retval : i32, 
} 
