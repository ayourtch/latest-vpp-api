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
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(dhcp6_pd_client_enable_disable_a75a0772)] 
pub struct Dhcp6PdClientEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub prefix_group : FixedSizeString<typenum::U64>, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(dhcp6_pd_client_enable_disable_reply_e8d4e804)] 
pub struct Dhcp6PdClientEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip6_add_del_address_using_prefix_3982f30a)] 
pub struct Ip6AddDelAddressUsingPrefix { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub prefix_group : FixedSizeString<typenum::U64>, 
	pub address_with_prefix : Ip6AddressWithPrefix, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(ip6_add_del_address_using_prefix_reply_e8d4e804)] 
pub struct Ip6AddDelAddressUsingPrefixReply { 
	pub context : u32, 
	pub retval : i32, 
} 
