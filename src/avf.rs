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
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(avf_create_daab8ae2)] 
pub struct AvfCreate { 
	pub client_index : u32, 
	pub context : u32, 
	pub pci_addr : u32, 
	pub enable_elog : i32, 
	pub rxq_num : u16, 
	pub rxq_size : u16, 
	pub txq_size : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(avf_create_reply_5383d31f)] 
pub struct AvfCreateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(avf_delete_f9e6675e)] 
pub struct AvfDelete { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(avf_delete_reply_e8d4e804)] 
pub struct AvfDeleteReply { 
	pub context : u32, 
	pub retval : i32, 
} 
