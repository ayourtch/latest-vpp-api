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
#[message_name_and_crc(ip6_nd_address_autoconfig_9e14a4a7)] 
pub struct Ip6NdAddressAutoconfig { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub enable : bool, 
	pub install_default_routes : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip6_nd_address_autoconfig_reply_e8d4e804)] 
pub struct Ip6NdAddressAutoconfigReply { 
	pub context : u32, 
	pub retval : i32, 
} 
