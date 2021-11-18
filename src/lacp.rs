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
use crate::interface_types::*; 
use crate::ethernet_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sw_interface_lacp_dump_51077d14)] 
pub struct SwInterfaceLacpDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sw_interface_lacp_details_d9a83d2f)] 
pub struct SwInterfaceLacpDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub interface_name : FixedSizeString<typenum::U64>, 
	pub rx_state : u32, 
	pub tx_state : u32, 
	pub mux_state : u32, 
	pub ptx_state : u32, 
	pub bond_interface_name : FixedSizeString<typenum::U64>, 
	pub actor_system_priority : u16, 
	pub actor_system : MacAddress, 
	pub actor_key : u16, 
	pub actor_port_priority : u16, 
	pub actor_port_number : u16, 
	pub actor_state : u8, 
	pub partner_system_priority : u16, 
	pub partner_system : MacAddress, 
	pub partner_key : u16, 
	pub partner_port_priority : u16, 
	pub partner_port_number : u16, 
	pub partner_state : u8, 
} 
