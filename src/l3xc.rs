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
use crate::fib_types::*; 
use crate::interface_types::*; 
// Implementation for l3xc 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct L3xc { 
	pub sw_if_index : InterfaceIndex, 
	pub is_ip6 : bool, 
	pub n_paths : u8, 
	pub paths : VariableSizeArray<FibPath>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(l3xc_plugin_get_version_51077d14)] 
pub struct L3xcPluginGetVersion { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(l3xc_plugin_get_version_reply_9b32cf86)] 
pub struct L3xcPluginGetVersionReply { 
	pub context : u32, 
	pub major : u32, 
	pub minor : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(l3xc_update_e96aabdf)] 
pub struct L3xcUpdate { 
	pub client_index : u32, 
	pub context : u32, 
	pub l3xc : L3xc, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(l3xc_update_reply_1992deab)] 
pub struct L3xcUpdateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub stats_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(l3xc_del_e7dbef91)] 
pub struct L3xcDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ip6 : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(l3xc_del_reply_e8d4e804)] 
pub struct L3xcDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(l3xc_dump_f9e6675e)] 
pub struct L3xcDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(l3xc_details_bc5bf852)] 
pub struct L3xcDetails { 
	pub context : u32, 
	pub l3xc : L3xc, 
} 
