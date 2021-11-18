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
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vxlan_gpe_ioam_export_enable_disable_d4c76d3a)] 
pub struct VxlanGpeIoamExportEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_disable : bool, 
	pub collector_address : Ip4Address, 
	pub src_address : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vxlan_gpe_ioam_export_enable_disable_reply_e8d4e804)] 
pub struct VxlanGpeIoamExportEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
