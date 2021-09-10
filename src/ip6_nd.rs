/*
   Autogenerated Data, Do not Edit! 
*/
#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]
use vpp_api_macros::{VppMessage,VppUnionIdent}; 
use std::convert::TryInto; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum;
use crate::ip_types::*; 
use crate::interface_types::*; 
// Implementation for ip6_ra_prefix_info 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct Ip6RaPrefixInfo { 
	pub prefix : Prefix, 
	pub flags : u8, 
	pub valid_time : u32, 
	pub preferred_time : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sw_interface_ip6nd_ra_config_3eb00b1c)] 
pub struct SwInterfaceIp6ndRaConfig { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub suppress : u8, 
	pub managed : u8, 
	pub other : u8, 
	pub ll_option : u8, 
	pub send_unicast : u8, 
	pub cease : u8, 
	pub is_no : bool, 
	pub default_router : u8, 
	pub max_interval : u32, 
	pub min_interval : u32, 
	pub lifetime : u32, 
	pub initial_count : u32, 
	pub initial_interval : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sw_interface_ip6nd_ra_config_reply_e8d4e804)] 
pub struct SwInterfaceIp6ndRaConfigReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sw_interface_ip6nd_ra_prefix_82cc1b28)] 
pub struct SwInterfaceIp6ndRaPrefix { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub prefix : Prefix, 
	pub use_default : bool, 
	pub no_advertise : bool, 
	pub off_link : bool, 
	pub no_autoconfig : bool, 
	pub no_onlink : bool, 
	pub is_no : bool, 
	pub val_lifetime : u32, 
	pub pref_lifetime : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sw_interface_ip6nd_ra_prefix_reply_e8d4e804)] 
pub struct SwInterfaceIp6ndRaPrefixReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip6nd_proxy_add_del_c2e4a686)] 
pub struct Ip6ndProxyAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_add : bool, 
	pub ip : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip6nd_proxy_add_del_reply_e8d4e804)] 
pub struct Ip6ndProxyAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip6nd_proxy_details_30b9ff4a)] 
pub struct Ip6ndProxyDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub ip : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip6nd_proxy_dump_51077d14)] 
pub struct Ip6ndProxyDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip6nd_send_router_solicitation_e5de609c)] 
pub struct Ip6ndSendRouterSolicitation { 
	pub client_index : u32, 
	pub context : u32, 
	pub irt : u32, 
	pub mrt : u32, 
	pub mrc : u32, 
	pub mrd : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub stop : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip6nd_send_router_solicitation_reply_e8d4e804)] 
pub struct Ip6ndSendRouterSolicitationReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(want_ip6_ra_events_3ec6d6c2)] 
pub struct WantIp6RaEvents { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable : bool, 
	pub pid : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(want_ip6_ra_events_reply_e8d4e804)] 
pub struct WantIp6RaEventsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip6_ra_event_0364c1c5)] 
pub struct Ip6RaEvent { 
	pub client_index : u32, 
	pub pid : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub router_addr : Ip6Address, 
	pub current_hop_limit : u8, 
	pub flags : u8, 
	pub router_lifetime_in_sec : u16, 
	pub neighbor_reachable_time_in_msec : u32, 
	pub time_in_msec_between_retransmitted_neighbor_solicitations : u32, 
	pub n_prefixes : u32, 
	pub prefixes : VariableSizeArray<Ip6RaPrefixInfo>, 
} 
