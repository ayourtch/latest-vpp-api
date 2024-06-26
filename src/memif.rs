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
use crate::ethernet_types::*;
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum MemifRole {
	 MEMIF_ROLE_API_MASTER=0,
	 MEMIF_ROLE_API_SLAVE=1,
}
impl Default for MemifRole {
	fn default() -> Self { MemifRole::MEMIF_ROLE_API_MASTER }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum MemifMode {
	 MEMIF_MODE_API_ETHERNET=0,
	 MEMIF_MODE_API_IP=1,
	 MEMIF_MODE_API_PUNT_INJECT=2,
}
impl Default for MemifMode {
	fn default() -> Self { MemifMode::MEMIF_MODE_API_ETHERNET }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_socket_filename_add_del_a2ce1a10)]
pub struct MemifSocketFilenameAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub socket_id: u32,
	pub socket_filename: FixedSizeString<typenum::U108>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_socket_filename_add_del_reply_e8d4e804)]
pub struct MemifSocketFilenameAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_socket_filename_add_del_v2_34223bdf)]
pub struct MemifSocketFilenameAddDelV2 {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub socket_id: u32,
	pub socket_filename: VariableSizeString,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_socket_filename_add_del_v2_reply_9f29bdb9)]
pub struct MemifSocketFilenameAddDelV2Reply {
	pub context: u32,
	pub retval: i32,
	pub socket_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_create_b1b25061)]
pub struct MemifCreate {
	pub client_index: u32,
	pub context: u32,
	pub role: MemifRole,
	pub mode: MemifMode,
	pub rx_queues: u8,
	pub tx_queues: u8,
	pub id: u32,
	pub socket_id: u32,
	pub ring_size: u32,
	pub buffer_size: u16,
	pub no_zero_copy: bool,
	pub hw_addr: MacAddress,
	pub secret: FixedSizeString<typenum::U24>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_create_reply_5383d31f)]
pub struct MemifCreateReply {
	pub context: u32,
	pub retval: i32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_create_v2_8c7de5f7)]
pub struct MemifCreateV2 {
	pub client_index: u32,
	pub context: u32,
	pub role: MemifRole,
	pub mode: MemifMode,
	pub rx_queues: u8,
	pub tx_queues: u8,
	pub id: u32,
	pub socket_id: u32,
	pub ring_size: u32,
	pub buffer_size: u16,
	pub no_zero_copy: bool,
	pub use_dma: bool,
	pub hw_addr: MacAddress,
	pub secret: FixedSizeString<typenum::U24>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_create_v2_reply_5383d31f)]
pub struct MemifCreateV2Reply {
	pub context: u32,
	pub retval: i32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_delete_f9e6675e)]
pub struct MemifDelete {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_delete_reply_e8d4e804)]
pub struct MemifDeleteReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_socket_filename_details_7ff326f7)]
pub struct MemifSocketFilenameDetails {
	pub context: u32,
	pub socket_id: u32,
	pub socket_filename: FixedSizeString<typenum::U108>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_socket_filename_dump_51077d14)]
pub struct MemifSocketFilenameDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_details_da34feb9)]
pub struct MemifDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub hw_addr: MacAddress,
	pub id: u32,
	pub role: MemifRole,
	pub mode: MemifMode,
	pub zero_copy: bool,
	pub socket_id: u32,
	pub ring_size: u32,
	pub buffer_size: u16,
	 pub flags: EnumFlag<IfStatusFlags>,
	pub if_name: FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(memif_dump_51077d14)]
pub struct MemifDump {
	pub client_index: u32,
	pub context: u32,
}
