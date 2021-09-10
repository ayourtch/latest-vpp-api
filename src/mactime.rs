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
use crate::interface_types::*; 
use crate::ethernet_types::*; 
// Implementation for time_range 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct TimeRange { 
	pub start : f64, 
	pub end : f64, 
} 
// Implementation for mactime_time_range 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct MactimeTimeRange { 
	pub start : f64, 
	pub end : f64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(mactime_enable_disable_3865946c)] 
pub struct MactimeEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable_disable : bool, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(mactime_enable_disable_reply_e8d4e804)] 
pub struct MactimeEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(mactime_add_del_range_cb56e877)] 
pub struct MactimeAddDelRange { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub drop : bool, 
	pub allow : bool, 
	pub allow_quota : u8, 
	pub no_udp_10001 : bool, 
	pub data_quota : u64, 
	pub mac_address : MacAddress, 
	pub device_name : FixedSizeString<typenum::U64>, 
	pub count : u32, 
	pub ranges : VariableSizeArray<TimeRange>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(mactime_add_del_range_reply_e8d4e804)] 
pub struct MactimeAddDelRangeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(mactime_dump_8f454e23)] 
pub struct MactimeDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub my_table_epoch : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(mactime_details_da25b13a)] 
pub struct MactimeDetails { 
	pub context : u32, 
	pub pool_index : u32, 
	pub mac_address : MacAddress, 
	pub data_quota : u64, 
	pub data_used_in_range : u64, 
	pub flags : u32, 
	pub device_name : FixedSizeString<typenum::U64>, 
	pub nranges : u32, 
	pub ranges : VariableSizeArray<MactimeTimeRange>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(mactime_dump_reply_49bcc753)] 
pub struct MactimeDumpReply { 
	pub context : u32, 
	pub retval : i32, 
	pub table_epoch : u32, 
} 
