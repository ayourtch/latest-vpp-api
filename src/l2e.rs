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
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(l2_emulation_ae6cfcfb)] 
pub struct L2Emulation { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(l2_emulation_reply_e8d4e804)] 
pub struct L2EmulationReply { 
	pub context : u32, 
	pub retval : i32, 
} 
