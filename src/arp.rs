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
// Implementation for proxy_arp 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct ProxyArp { 
	pub table_id : u32, 
	pub low : Ip4Address, 
	pub hi : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(proxy_arp_add_del_1823c3e7)] 
pub struct ProxyArpAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub proxy : ProxyArp, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(proxy_arp_add_del_reply_e8d4e804)] 
pub struct ProxyArpAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(proxy_arp_dump_51077d14)] 
pub struct ProxyArpDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(proxy_arp_details_5b948673)] 
pub struct ProxyArpDetails { 
	pub context : u32, 
	pub proxy : ProxyArp, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(proxy_arp_intfc_enable_disable_ae6cfcfb)] 
pub struct ProxyArpIntfcEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(proxy_arp_intfc_enable_disable_reply_e8d4e804)] 
pub struct ProxyArpIntfcEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(proxy_arp_intfc_dump_51077d14)] 
pub struct ProxyArpIntfcDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(proxy_arp_intfc_details_f6458e5f)] 
pub struct ProxyArpIntfcDetails { 
	pub context : u32, 
	pub sw_if_index : u32, 
} 
