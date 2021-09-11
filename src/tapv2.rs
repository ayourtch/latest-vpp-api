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
#[derive(Debug,Serialize, Deserialize, Clone, Copy)] 
pub enum TapFlags { 
	 TAP_API_FLAG_GSO=1, 
	 TAP_API_FLAG_CSUM_OFFLOAD=2, 
	 TAP_API_FLAG_PERSIST=4, 
	 TAP_API_FLAG_ATTACH=8, 
	 TAP_API_FLAG_TUN=16, 
	 TAP_API_FLAG_GRO_COALESCE=32, 
	 TAP_API_FLAG_PACKED=64, 
	 TAP_API_FLAG_IN_ORDER=128, 
} 
impl Default for TapFlags { 
	fn default() -> Self { TapFlags::TAP_API_FLAG_GSO }
}
impl AsEnumFlag for TapFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => TapFlags::TAP_API_FLAG_GSO, 
			 2 => TapFlags::TAP_API_FLAG_CSUM_OFFLOAD, 
			 4 => TapFlags::TAP_API_FLAG_PERSIST, 
			 8 => TapFlags::TAP_API_FLAG_ATTACH, 
			 16 => TapFlags::TAP_API_FLAG_TUN, 
			 32 => TapFlags::TAP_API_FLAG_GRO_COALESCE, 
			 64 => TapFlags::TAP_API_FLAG_PACKED, 
			 128 => TapFlags::TAP_API_FLAG_IN_ORDER, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(tap_create_v2_2d0d6570)] 
pub struct TapCreateV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub id : u32, 
	pub use_random_mac : bool, 
	pub mac_address : MacAddress, 
	pub num_rx_queues : u8, 
	pub tx_ring_sz : u16, 
	pub rx_ring_sz : u16, 
	pub host_mtu_set : bool, 
	pub host_mtu_size : u32, 
	pub host_mac_addr_set : bool, 
	pub host_mac_addr : MacAddress, 
	pub host_ip4_prefix_set : bool, 
	pub host_ip4_prefix : Ip4AddressWithPrefix, 
	pub host_ip6_prefix_set : bool, 
	pub host_ip6_prefix : Ip6AddressWithPrefix, 
	pub host_ip4_gw_set : bool, 
	pub host_ip4_gw : Ip4Address, 
	pub host_ip6_gw_set : bool, 
	pub host_ip6_gw : Ip6Address, 
	 pub tap_flags : EnumFlag<TapFlags>, 
	pub host_namespace_set : bool, 
	pub host_namespace : FixedSizeString<typenum::U64>, 
	pub host_if_name_set : bool, 
	pub host_if_name : FixedSizeString<typenum::U64>, 
	pub host_bridge_set : bool, 
	pub host_bridge : FixedSizeString<typenum::U64>, 
	pub tag : VariableSizeString, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(tap_create_v2_reply_5383d31f)] 
pub struct TapCreateV2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(tap_delete_v2_f9e6675e)] 
pub struct TapDeleteV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(tap_delete_v2_reply_e8d4e804)] 
pub struct TapDeleteV2Reply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sw_interface_tap_v2_dump_f9e6675e)] 
pub struct SwInterfaceTapV2Dump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sw_interface_tap_v2_details_1e2b2a47)] 
pub struct SwInterfaceTapV2Details { 
	pub context : u32, 
	pub sw_if_index : u32, 
	pub id : u32, 
	pub tx_ring_sz : u16, 
	pub rx_ring_sz : u16, 
	pub host_mtu_size : u32, 
	pub host_mac_addr : MacAddress, 
	pub host_ip4_prefix : Ip4AddressWithPrefix, 
	pub host_ip6_prefix : Ip6AddressWithPrefix, 
	 pub tap_flags : EnumFlag<TapFlags>, 
	pub dev_name : FixedSizeString<typenum::U64>, 
	pub host_if_name : FixedSizeString<typenum::U64>, 
	pub host_namespace : FixedSizeString<typenum::U64>, 
	pub host_bridge : FixedSizeString<typenum::U64>, 
} 