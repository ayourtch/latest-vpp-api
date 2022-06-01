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
use crate::nat_types::*;
use crate::interface_types::*;
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_plugin_enable_disable_45948b90)]
pub struct Nat64PluginEnableDisable {
	pub client_index : u32,
	pub context : u32,
	pub bib_buckets : u32,
	pub bib_memory_size : u32,
	pub st_buckets : u32,
	pub st_memory_size : u32,
	pub enable : bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_plugin_enable_disable_reply_e8d4e804)]
pub struct Nat64PluginEnableDisableReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_set_timeouts_d4746b16)]
pub struct Nat64SetTimeouts {
	pub client_index : u32,
	pub context : u32,
	pub udp : u32,
	pub tcp_established : u32,
	pub tcp_transitory : u32,
	pub icmp : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_set_timeouts_reply_e8d4e804)]
pub struct Nat64SetTimeoutsReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_get_timeouts_51077d14)]
pub struct Nat64GetTimeouts {
	pub client_index : u32,
	pub context : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_get_timeouts_reply_3c4df4e1)]
pub struct Nat64GetTimeoutsReply {
	pub context : u32,
	pub retval : i32,
	pub udp : u32,
	pub tcp_established : u32,
	pub tcp_transitory : u32,
	pub icmp : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_add_del_pool_addr_range_a3b944e3)]
pub struct Nat64AddDelPoolAddrRange {
	pub client_index : u32,
	pub context : u32,
	pub start_addr : Ip4Address,
	pub end_addr : Ip4Address,
	pub vrf_id : u32,
	pub is_add : bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_add_del_pool_addr_range_reply_e8d4e804)]
pub struct Nat64AddDelPoolAddrRangeReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_pool_addr_dump_51077d14)]
pub struct Nat64PoolAddrDump {
	pub client_index : u32,
	pub context : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_pool_addr_details_9bb99cdb)]
pub struct Nat64PoolAddrDetails {
	pub context : u32,
	pub address : Ip4Address,
	pub vrf_id : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_add_del_interface_f3699b83)]
pub struct Nat64AddDelInterface {
	pub client_index : u32,
	pub context : u32,
	pub is_add : bool,
	 pub flags : EnumFlag<NatConfigFlags>,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_add_del_interface_reply_e8d4e804)]
pub struct Nat64AddDelInterfaceReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_interface_dump_51077d14)]
pub struct Nat64InterfaceDump {
	pub client_index : u32,
	pub context : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_interface_details_5d286289)]
pub struct Nat64InterfaceDetails {
	pub context : u32,
	 pub flags : EnumFlag<NatConfigFlags>,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_add_del_static_bib_1c404de5)]
pub struct Nat64AddDelStaticBib {
	pub client_index : u32,
	pub context : u32,
	pub i_addr : Ip6Address,
	pub o_addr : Ip4Address,
	pub i_port : u16,
	pub o_port : u16,
	pub vrf_id : u32,
	pub proto : u8,
	pub is_add : bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_add_del_static_bib_reply_e8d4e804)]
pub struct Nat64AddDelStaticBibReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_bib_dump_cfcb6b75)]
pub struct Nat64BibDump {
	pub client_index : u32,
	pub context : u32,
	pub proto : u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_bib_details_43bc3ddf)]
pub struct Nat64BibDetails {
	pub context : u32,
	pub i_addr : Ip6Address,
	pub o_addr : Ip4Address,
	pub i_port : u16,
	pub o_port : u16,
	pub vrf_id : u32,
	pub proto : u8,
	 pub flags : EnumFlag<NatConfigFlags>,
	pub ses_num : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_st_dump_cfcb6b75)]
pub struct Nat64StDump {
	pub client_index : u32,
	pub context : u32,
	pub proto : u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_st_details_dd3361ed)]
pub struct Nat64StDetails {
	pub context : u32,
	pub il_addr : Ip6Address,
	pub ol_addr : Ip4Address,
	pub il_port : u16,
	pub ol_port : u16,
	pub ir_addr : Ip6Address,
	pub or_addr : Ip4Address,
	pub r_port : u16,
	pub vrf_id : u32,
	pub proto : u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_add_del_prefix_727b2f4c)]
pub struct Nat64AddDelPrefix {
	pub client_index : u32,
	pub context : u32,
	pub prefix : Ip6Prefix,
	pub vrf_id : u32,
	pub is_add : bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_add_del_prefix_reply_e8d4e804)]
pub struct Nat64AddDelPrefixReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_prefix_dump_51077d14)]
pub struct Nat64PrefixDump {
	pub client_index : u32,
	pub context : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_prefix_details_20568de3)]
pub struct Nat64PrefixDetails {
	pub context : u32,
	pub prefix : Ip6Prefix,
	pub vrf_id : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_add_del_interface_addr_47d6e753)]
pub struct Nat64AddDelInterfaceAddr {
	pub client_index : u32,
	pub context : u32,
	pub is_add : bool,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat64_add_del_interface_addr_reply_e8d4e804)]
pub struct Nat64AddDelInterfaceAddrReply {
	pub context : u32,
	pub retval : i32,
}
