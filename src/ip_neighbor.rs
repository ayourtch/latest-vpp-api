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
use crate::ethernet_types::*; 
// Implementation for ip_neighbor 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct IpNeighbor { 
	pub sw_if_index : InterfaceIndex, 
	pub flags : EnumFlag<IpNeighborFlags>, 
	pub mac_address : MacAddress, 
	pub ip_address : Address, 
} 
#[derive(Debug,Serialize, Deserialize, Clone, Copy)] 
pub enum IpNeighborFlags { 
	 IP_API_NEIGHBOR_FLAG_NONE=0, 
	 IP_API_NEIGHBOR_FLAG_STATIC=1, 
	 IP_API_NEIGHBOR_FLAG_NO_FIB_ENTRY=2, 
} 
impl Default for IpNeighborFlags { 
	fn default() -> Self { IpNeighborFlags::IP_API_NEIGHBOR_FLAG_NONE }
}
impl AsEnumFlag for IpNeighborFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 0 => IpNeighborFlags::IP_API_NEIGHBOR_FLAG_NONE, 
			 1 => IpNeighborFlags::IP_API_NEIGHBOR_FLAG_STATIC, 
			 2 => IpNeighborFlags::IP_API_NEIGHBOR_FLAG_NO_FIB_ENTRY, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 8 as u32
	}
}
#[derive(Debug,Serialize, Deserialize, Clone, Copy)] 
pub enum IpNeighborEventFlags { 
	 IP_NEIGHBOR_API_EVENT_FLAG_ADDED=1, 
	 IP_NEIGHBOR_API_EVENT_FLAG_REMOVED=2, 
} 
impl Default for IpNeighborEventFlags { 
	fn default() -> Self { IpNeighborEventFlags::IP_NEIGHBOR_API_EVENT_FLAG_ADDED }
}
impl AsEnumFlag for IpNeighborEventFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => IpNeighborEventFlags::IP_NEIGHBOR_API_EVENT_FLAG_ADDED, 
			 2 => IpNeighborEventFlags::IP_NEIGHBOR_API_EVENT_FLAG_REMOVED, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_add_del_0607c257)] 
pub struct IpNeighborAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub neighbor : IpNeighbor, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_add_del_reply_1992deab)] 
pub struct IpNeighborAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub stats_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_dump_d817a484)] 
pub struct IpNeighborDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub af : AddressFamily, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_details_e29d79f0)] 
pub struct IpNeighborDetails { 
	pub context : u32, 
	pub age : f64, 
	pub neighbor : IpNeighbor, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_config_f4a5cf44)] 
pub struct IpNeighborConfig { 
	pub client_index : u32, 
	pub context : u32, 
	pub af : AddressFamily, 
	pub max_number : u32, 
	pub max_age : u32, 
	pub recycle : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_config_reply_e8d4e804)] 
pub struct IpNeighborConfigReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_replace_begin_51077d14)] 
pub struct IpNeighborReplaceBegin { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_replace_begin_reply_e8d4e804)] 
pub struct IpNeighborReplaceBeginReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_replace_end_51077d14)] 
pub struct IpNeighborReplaceEnd { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_replace_end_reply_e8d4e804)] 
pub struct IpNeighborReplaceEndReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_flush_16aa35d2)] 
pub struct IpNeighborFlush { 
	pub client_index : u32, 
	pub context : u32, 
	pub af : AddressFamily, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_flush_reply_e8d4e804)] 
pub struct IpNeighborFlushReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(want_ip_neighbor_events_73e70a86)] 
pub struct WantIpNeighborEvents { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable : bool, 
	pub pid : u32, 
	pub ip : Address, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(want_ip_neighbor_events_reply_e8d4e804)] 
pub struct WantIpNeighborEventsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_event_bdb092b2)] 
pub struct IpNeighborEvent { 
	pub client_index : u32, 
	pub pid : u32, 
	pub neighbor : IpNeighbor, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(want_ip_neighbor_events_v2_73e70a86)] 
pub struct WantIpNeighborEventsV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable : bool, 
	pub pid : u32, 
	pub ip : Address, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(want_ip_neighbor_events_v2_reply_e8d4e804)] 
pub struct WantIpNeighborEventsV2Reply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip_neighbor_event_v2_c1d53dc0)] 
pub struct IpNeighborEventV2 { 
	pub client_index : u32, 
	pub pid : u32, 
	 pub flags : EnumFlag<IpNeighborEventFlags>, 
	pub neighbor : IpNeighbor, 
} 
