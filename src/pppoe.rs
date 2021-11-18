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
use crate::ethernet_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pppoe_add_del_session_f6fd759e)] 
pub struct PppoeAddDelSession { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub session_id : u16, 
	pub client_ip : Address, 
	pub decap_vrf_id : u32, 
	pub client_mac : MacAddress, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pppoe_add_del_session_reply_5383d31f)] 
pub struct PppoeAddDelSessionReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pppoe_session_dump_f9e6675e)] 
pub struct PppoeSessionDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pppoe_session_details_4b8e8a4a)] 
pub struct PppoeSessionDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub session_id : u16, 
	pub client_ip : Address, 
	pub encap_if_index : InterfaceIndex, 
	pub decap_vrf_id : u32, 
	pub local_mac : MacAddress, 
	pub client_mac : MacAddress, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pppoe_add_del_cp_eacd9aaa)] 
pub struct PppoeAddDelCp { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_add : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pppoe_add_del_cp_reply_e8d4e804)] 
pub struct PppoeAddDelCpReply { 
	pub context : u32, 
	pub retval : i32, 
} 
