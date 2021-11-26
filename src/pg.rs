/*
   Autogenerated Data, Do not Edit! 
*/
#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]
use vpp_api_macros::{VppMessage,VppUnionIdent}; 
use std::convert::TryInto; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use vpp_api_encoding::typ::*;
pub use vpp_api_encoding;
use vpp_api_message::VppApiMessage;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum;
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum PgInterfaceMode { 
	 PG_API_MODE_ETHERNET=0, 
	 PG_API_MODE_IP4=1, 
	 PG_API_MODE_IP6=2, 
} 
impl Default for PgInterfaceMode { 
	fn default() -> Self { PgInterfaceMode::PG_API_MODE_ETHERNET }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pg_create_interface_b7c893d7)] 
pub struct PgCreateInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub interface_id : InterfaceIndex, 
	pub gso_enabled : bool, 
	pub gso_size : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pg_create_interface_v2_8657466a)] 
pub struct PgCreateInterfaceV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub interface_id : InterfaceIndex, 
	pub gso_enabled : bool, 
	pub gso_size : u32, 
	pub mode : PgInterfaceMode, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pg_create_interface_reply_5383d31f)] 
pub struct PgCreateInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pg_create_interface_v2_reply_5383d31f)] 
pub struct PgCreateInterfaceV2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pg_interface_enable_disable_coalesce_a2ef99e7)] 
pub struct PgInterfaceEnableDisableCoalesce { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub coalesce_enabled : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pg_interface_enable_disable_coalesce_reply_e8d4e804)] 
pub struct PgInterfaceEnableDisableCoalesceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pg_capture_3712fb6c)] 
pub struct PgCapture { 
	pub client_index : u32, 
	pub context : u32, 
	pub interface_id : InterfaceIndex, 
	pub is_enabled : bool, 
	pub count : u32, 
	pub pcap_file_name : VariableSizeString, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pg_capture_reply_e8d4e804)] 
pub struct PgCaptureReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pg_enable_disable_01f94f3a)] 
pub struct PgEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enabled : bool, 
	pub stream_name : VariableSizeString, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pg_enable_disable_reply_e8d4e804)] 
pub struct PgEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
